// Jira API documentation:
// * https://docs.atlassian.com/software/jira/docs/api/REST/latest/
// * https://docs.atlassian.com/jira-software/REST/latest/

use log::debug;
use restson::blocking::RestClient as BlockingRestClient;
use restson::{Error, Response as RestResponse, RestClient, RestPath};

use crate::issue_model::{Issue, JqlResults};

// The prefix of every subsequent REST request.
// This string comes directly after the host in the URL.
const REST_PREFIX: &str = "rest/api/2";

/// Configuration and credentials to access a Jira instance.
#[derive(Default)]
pub struct JiraInstance {
    pub host: String,
    pub auth: Auth,
    pub pagination: Pagination,
}

/// The authentication method used to contact Jira.
#[derive(Default)]
pub enum Auth {
    #[default]
    Anonymous,
    ApiKey(String),
}

/// Controls the upper limit of how many tickets the response from Jira can contain:
///
/// * `Default`: Use the default settings of this instance, which sets an arbitrary limit on the number of tickets.
/// * `MaxResults`: Set the upper limit to this value. Note that each instance has a maximum allowed value,
/// and if you set `MaxResults` higher than that, the instance uses its own maximum allowed value.
/// * `Chunks`: Access the tickets in a series of requests, each accessing the number of tickets equal to the chunk size.
/// This enables you to access an unlimited number of tickets, as long as the chunk size is smaller
/// than the maximum allowed results size for the instance.
#[derive(Default)]
pub enum Pagination {
    #[default]
    Default,
    MaxResults(u32),
    Chunks(u32),
}

/// This struct temporarily groups together all the parameters to make a REST request.
/// It exists here because `RestPath` is only generic over a single parameter.
struct Request<'a> {
    keys: &'a [&'a str],
    pagination: &'a Pagination,
}

/// API call with one &str parameter
impl RestPath<&str> for Issue {
    fn get_path(param: &str) -> Result<String, Error> {
        Ok(format!("{}/issue/{}", REST_PREFIX, param))
    }
}

// TODO: Make this generic over &[&str] and &[String].
/// API call with several &str parameters representing the IDs of issues.
impl RestPath<Request<'_>> for JqlResults {
    fn get_path(request: Request) -> Result<String, Error> {
        let max_results = match request.pagination {
            Pagination::Default => String::new(),
            // For both MaxResults and Chunks, set the maxResults size to the value set in the variant.
            // The maxResults size is relevant for Chunks in that each chunk requires its own results
            // to be at least this large.
            Pagination::MaxResults(n) | Pagination::Chunks(n) => format!("&maxResults={}", n),
        };
        Ok(format!(
            "{}/search?jql=id%20in%20({}){}",
            REST_PREFIX,
            request.keys.join(","),
            max_results,
        ))
    }
}

impl JiraInstance {
    /// Build a client that connects to Jira.
    /// This is a separate function so that both `issue` and `issues` can reuse it.
    fn client(&self) -> Result<BlockingRestClient, Error> {
        let mut client = RestClient::builder().blocking(&self.host)?;

        if let Auth::ApiKey(api_key) = &self.auth {
            client.set_header("Authorization", &format!("Bearer {}", api_key))?;
        }

        Ok(client)
    }

    // This method uses a separate implementation from `issues` because Jira provides a way
    // to request a single ticket specifically. That conveniently handles error cases
    // where no tickets might match, or more than one might.
    /// Access a single issue by its key.
    pub fn issue(&self, key: &str) -> Result<Issue, Error> {
        let client = self.client()?;

        // Gets a bug by ID and deserializes the JSON to data variable
        let data: RestResponse<Issue> = client.get(key)?;
        let issue = data.into_inner();
        debug!("{:#?}", issue);

        Ok(issue)
    }

    /// Access several issues by their keys.
    pub fn issues(&self, keys: &[&str]) -> Result<Vec<Issue>, Error> {
        let client = self.client()?;

        let request = Request {
            keys,
            pagination: &self.pagination,
        };

        // Gets a bug by ID and deserializes the JSON to data variable
        let data: RestResponse<JqlResults> = client.get(request)?;
        let results = data.into_inner();
        debug!("{:#?}", results);

        // TODO: Note that the resulting list might be empty and still Ok
        Ok(results.issues)
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
