// Jira API documentation:
// * https://docs.atlassian.com/software/jira/docs/api/REST/latest/
// * https://docs.atlassian.com/jira-software/REST/latest/

use restson::{RestClient, RestPath};

use crate::errors::JiraQueryError;
use crate::issue_model::{Issue, JqlResults};

// The prefix of every subsequent REST request.
// This string comes directly after the host in the URL.
const REST_PREFIX: &str = "rest/api/2";

/// Configuration and credentials to access a Jira instance.
pub struct JiraInstance {
    pub host: String,
    pub auth: Auth,
    pub pagination: Pagination,
    client: RestClient,
}

/// The authentication method used to contact Jira.
#[derive(Default)]
pub enum Auth {
    #[default]
    Anonymous,
    ApiKey(String),
    Basic {
        user: String,
        password: String,
    },
}

/// Controls the upper limit of how many tickets the response from Jira can contain:
///
/// * `Default`: Use the default settings of this instance, which sets an arbitrary limit on the number of tickets.
/// * `MaxResults`: Set the upper limit to this value. Note that each instance has a maximum allowed value,
/// and if you set `MaxResults` higher than that, the instance uses its own maximum allowed value.
/// * `ChunkSize`: Access the tickets in a series of requests, each accessing the number of tickets equal to the chunk size.
/// This enables you to access an unlimited number of tickets, as long as the chunk size is smaller
/// than the maximum allowed results size for the instance.
#[derive(Default)]
pub enum Pagination {
    #[default]
    Default,
    MaxResults(u32),
    ChunkSize(u32),
}

/// This struct temporarily groups together all the parameters to make a REST request.
/// It exists here because `RestPath` is only generic over a single parameter.
struct Request<'a> {
    method: Method<'a>,
    pagination: &'a Pagination,
    start_at: u32,
}

/// The method of the request to Jira. Either request specific IDs,
/// or use a free-form JQL search query.
enum Method<'a> {
    Keys(&'a [&'a str]),
    Search(&'a str),
}

impl<'a> Method<'a> {
    fn url_fragment(&self) -> String {
        match self {
            Self::Keys(ids) => format!("id%20in%20({})", ids.join(",")),
            Self::Search(query) => (*query).to_string(),
        }
    }
}

/// API call with one &str parameter
impl RestPath<&str> for Issue {
    fn get_path(param: &str) -> Result<String, restson::Error> {
        Ok(format!("{}/issue/{}", REST_PREFIX, param))
    }
}

/// API call with several &str parameters representing the IDs of issues.
impl RestPath<&Request<'_>> for JqlResults {
    fn get_path(request: &Request) -> Result<String, restson::Error> {
        let max_results = match request.pagination {
            Pagination::Default => String::new(),
            // For both MaxResults and ChunkSIze, set the maxResults size to the value set in the variant.
            // The maxResults size is relevant for ChunkSize in that each chunk requires its own results
            // to be at least this large.
            Pagination::MaxResults(n) | Pagination::ChunkSize(n) => format!("&maxResults={}", n),
        };
        let start_at = format!("&startAt={}", request.start_at);
        Ok(format!(
            "{}/search?jql={}{}{}",
            REST_PREFIX,
            request.method.url_fragment(),
            max_results,
            start_at,
        ))
    }
}

impl JiraInstance {
    /// Create a new `BzInstance` struct using a host URL, with default values
    /// for all options.
    pub fn at(host: String) -> Result<Self, JiraQueryError> {
        // TODO: This function takes host as a String, even though client is happy with &str.
        // The String is only used in the host struct attribute.
        let client = RestClient::new(&host)?;

        Ok(Self {
            host,
            client,
            auth: Auth::default(),
            pagination: Pagination::default(),
        })
    }

    /// Set the authentication method of this `JiraInstance`.
    pub fn authenticate(mut self, auth: Auth) -> Result<Self, JiraQueryError> {
        self.auth = auth;
        // Apply the configured authentication.
        // If the user selects the API key authorization, set the API key in the request header.
        // If the user selects the basic user and password authentication, set them in the client.
        // Otherwise, the anonymous authorization doesn't modify the request in any way.
        match &self.auth {
            Auth::ApiKey(key) => {
                self.client
                    .set_header("Authorization", &format!("Bearer {}", key))?;
            }
            Auth::Basic { user, password } => {
                self.client.set_auth(user, password);
            }
            Auth::Anonymous => {}
        }
        Ok(self)
    }

    /// Set the pagination method of this `JiraInstance`.
    #[must_use]
    pub const fn paginate(mut self, pagination: Pagination) -> Self {
        self.pagination = pagination;
        self
    }

    // This method uses a separate implementation from `issues` because Jira provides a way
    // to request a single ticket specifically. That conveniently handles error cases
    // where no tickets might match, or more than one might.
    /// Access a single issue by its key.
    pub async fn issue(&self, key: &str) -> Result<Issue, JiraQueryError> {
        // Gets an issue by ID and deserializes the JSON to data variable
        let data: restson::Response<Issue> = self.client.get(key).await?;
        let issue = data.into_inner();
        log::debug!("{:#?}", issue);

        Ok(issue)
    }

    /// Access several issues by their keys.
    ///
    /// If the list of keys is empty, returns an empty list back with no errors.
    pub async fn issues(&self, keys: &[&str]) -> Result<Vec<Issue>, JiraQueryError> {
        // If the user specifies no keys, skip network requests and return no bugs.
        // Returning an error could also be valid, but I believe that this behavior
        // is less surprising and more practical.
        if keys.is_empty() {
            return Ok(Vec::new());
        }

        // The initial request, common to all pagination methods.
        let mut request = Request {
            method: Method::Keys(keys),
            pagination: &self.pagination,
            start_at: 0,
        };

        // If Pagination is set to ChunkSize, split the issue keys into chunk by chunk size
        // and request each chunk separately.
        if let Pagination::ChunkSize(size) = self.pagination {
            let mut all_issues = Vec::new();

            loop {
                let mut chunk_issues = self.chunk_of_issues(&request).await?;

                // End the loop if no more issues are coming on the this page.
                if chunk_issues.is_empty() {
                    break;
                }

                all_issues.append(&mut chunk_issues);

                request.start_at += size;
            }

            Ok(all_issues)
        // If Pagination is not set to ChunkSize, use a single chunk request for all issues.
        } else {
            let issues = self.chunk_of_issues(&request).await?;

            // If the resulting list is empty, return an error.
            // TODO: The REST parsing above already results in an error if the results are empty.
            // Try to catch the error there.
            if issues.is_empty() {
                Err(JiraQueryError::NoIssues)
            } else {
                Ok(issues)
            }
        }
    }

    /// Download a specific list (chunk) of issues.
    /// Reused elsewhere as a building block of different pagination methods.
    async fn chunk_of_issues(&self, request: &Request<'_>) -> Result<Vec<Issue>, JiraQueryError> {
        let data: restson::Response<JqlResults> = self.client.get(request).await?;
        let results = data.into_inner();
        log::debug!("{:#?}", results);

        Ok(results.issues)
    }

    /// Access issues using a free-form JQL search.
    ///
    /// An example of a query: `project="CentOS Stream" AND priority = High`.
    pub async fn search(&self, query: &str) -> Result<Vec<Issue>, JiraQueryError> {
        // The initial request, common to all pagination methods.
        let mut request = Request {
            method: Method::Search(query),
            pagination: &self.pagination,
            start_at: 0,
        };

        // If Pagination is set to ChunkSize, split the issue keys into chunk by chunk size
        // and request each chunk separately.
        if let Pagination::ChunkSize(size) = self.pagination {
            let mut all_issues = Vec::new();

            loop {
                let mut chunk_issues = self.chunk_of_issues(&request).await?;

                // End the loop if no more issues are coming on the this page.
                if chunk_issues.is_empty() {
                    break;
                }

                all_issues.append(&mut chunk_issues);

                request.start_at += size;
            }

            Ok(all_issues)
        // If Pagination is not set to ChunkSize, use a single chunk request for all issues.
        } else {
            let issues = self.chunk_of_issues(&request).await?;

            Ok(issues)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // #[test]
    // fn issues() {
    //     let results = crate::issues("todo", &["todo"], "todo");
    //     eprintln!("{:#?}", results);
    //     assert_eq!(results.issues.len(), todo);
    // }
}
