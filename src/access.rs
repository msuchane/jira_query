/*
Copyright 2022 Marek Suchánek <msuchane@redhat.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Jira API documentation:
// * https://docs.atlassian.com/software/jira/docs/api/REST/latest/
// * https://docs.atlassian.com/jira-software/REST/latest/

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
    client: reqwest::Client,
}

/// The authentication method used to contact Jira.
pub enum Auth {
    Anonymous,
    ApiKey(String),
    Basic { user: String, password: String },
}

// We could set a default enum variant and derive, but that raises the MSRV to 1.62.
impl Default for Auth {
    fn default() -> Self {
        Self::Anonymous
    }
}

/// Controls the upper limit of how many tickets the response from Jira can contain:
///
/// * `Default`: Use the default settings of this instance, which sets an arbitrary limit on the number of tickets.
/// * `MaxResults`: Set the upper limit to this value. Note that each instance has a maximum allowed value,
/// and if you set `MaxResults` higher than that, the instance uses its own maximum allowed value.
/// * `ChunkSize`: Access the tickets in a series of requests, each accessing the number of tickets equal to the chunk size.
/// This enables you to access an unlimited number of tickets, as long as the chunk size is smaller
/// than the maximum allowed results size for the instance.
pub enum Pagination {
    Default,
    MaxResults(u32),
    ChunkSize(u32),
}

// We could set a default enum variant and derive, but that raises the MSRV to 1.62.
impl Default for Pagination {
    fn default() -> Self {
        Self::Default
    }
}

/// The method of the request to Jira. Either request specific IDs,
/// or use a free-form JQL search query.
enum Method<'a> {
    Key(&'a str),
    Keys(&'a [&'a str]),
    Search(&'a str),
}

impl<'a> Method<'a> {
    fn url_fragment(&self) -> String {
        match self {
            Self::Key(id) => format!("issue/{id}"),
            Self::Keys(ids) => format!("search?jql=id%20in%20({})", ids.join(",")),
            Self::Search(query) => format!("search?jql={query}"),
        }
    }
}

impl JiraInstance {
    /// Create a new `BzInstance` struct using a host URL, with default values
    /// for all options.
    pub fn at(host: String) -> Result<Self, JiraQueryError> {
        // TODO: This function takes host as a String, even though client is happy with &str.
        // The String is only used in the host struct attribute.
        let client = reqwest::Client::new();

        Ok(Self {
            host,
            client,
            auth: Auth::default(),
            pagination: Pagination::default(),
        })
    }

    /// Set the authentication method of this `JiraInstance`.
    #[must_use]
    pub fn authenticate(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    /// Set the http client of this `JiraInstance`.
    #[must_use]
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Set the pagination method of this `JiraInstance`.
    #[must_use]
    pub const fn paginate(mut self, pagination: Pagination) -> Self {
        self.pagination = pagination;
        self
    }

    /// Based on the request method, form a complete, absolute URL
    /// to download the tickets from the REST API.
    #[must_use]
    fn path(&self, method: &Method, start_at: u32) -> String {
        let max_results = match self.pagination {
            Pagination::Default => String::new(),
            // For both MaxResults and ChunkSIze, set the maxResults size to the value set in the variant.
            // The maxResults size is relevant for ChunkSize in that each chunk requires its own results
            // to be at least this large.
            Pagination::MaxResults(n) | Pagination::ChunkSize(n) => format!("&maxResults={n}"),
        };

        // The `startAt` option is only valid with JQL. With a URL by key, it breaks the REST query.
        let start_at = match method {
            Method::Key(_) => String::new(),
            Method::Keys(_) | Method::Search(_) => format!("&startAt={start_at}"),
        };

        format!(
            "{}/{}/{}{}{}",
            self.host,
            REST_PREFIX,
            method.url_fragment(),
            max_results,
            start_at,
        )
    }

    /// Download the specified URL using the configured authentication.
    async fn authenticated_get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let request_builder = self.client.get(url);
        let authenticated = match &self.auth {
            Auth::Anonymous => request_builder,
            Auth::ApiKey(key) => request_builder.header("Authorization", &format!("Bearer {key}")),
            Auth::Basic { user, password } => request_builder.basic_auth(user, Some(password)),
        };
        authenticated.send().await
    }

    // This method uses a separate implementation from `issues` because Jira provides a way
    // to request a single ticket specifically. That conveniently handles error cases
    // where no tickets might match, or more than one might.
    /// Access a single issue by its key.
    pub async fn issue(&self, key: &str) -> Result<Issue, JiraQueryError> {
        let url = self.path(&Method::Key(key), 0);

        // Gets an issue by ID and deserializes the JSON to data variable
        let issue = self.authenticated_get(&url).await?.json::<Issue>().await?;

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

        let method = Method::Keys(keys);

        // If Pagination is set to ChunkSize, split the issue keys into chunk by chunk size
        // and request each chunk separately.
        if let Pagination::ChunkSize(chunk_size) = self.pagination {
            self.paginated_issues(&method, chunk_size).await
        // If Pagination is not set to ChunkSize, use a single chunk request for all issues.
        } else {
            let issues = self.chunk_of_issues(&method, 0).await?;

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

    /// Download all issues specified in the request as a series of chunks or pages.
    /// The request controls whether the download works with IDs or JQL.
    /// This function only processes the resulting pages coming back from Jira
    /// and stops the iteration at the last page.
    ///
    /// See the Jira documentation:
    /// <https://confluence.atlassian.com/jirakb/changing-maxresults-parameter-for-jira-rest-api-779160706.html>.
    async fn paginated_issues(
        &self,
        method: &Method<'_>,
        chunk_size: u32,
    ) -> Result<Vec<Issue>, JiraQueryError> {
        let mut all_issues = Vec::new();
        let mut start_at = 0;

        loop {
            let mut chunk_issues = self.chunk_of_issues(method, start_at).await?;
            // Calculate the length now before the content moves to `all_issues`.
            let page_size = chunk_issues.len();
            all_issues.append(&mut chunk_issues);

            // If this page contains fewer issues than the chunk size,
            // it's the last page. Stop the loop.
            if page_size < chunk_size as usize {
                break;
            }

            start_at += chunk_size;
        }

        Ok(all_issues)
    }

    /// Download a specific list (chunk) of issues.
    /// Reused elsewhere as a building block of different pagination methods.
    async fn chunk_of_issues(
        &self,
        method: &Method<'_>,
        start_at: u32,
    ) -> Result<Vec<Issue>, JiraQueryError> {
        let url = self.path(method, start_at);

        let results = self
            .authenticated_get(&url)
            .await?
            .json::<JqlResults>()
            .await?;

        log::debug!("{:#?}", results);

        Ok(results.issues)
    }

    /// Access issues using a free-form JQL search.
    ///
    /// An example of a query: `project="CentOS Stream" AND priority = High`.
    pub async fn search(&self, query: &str) -> Result<Vec<Issue>, JiraQueryError> {
        let method = Method::Search(query);

        // If Pagination is set to ChunkSize, split the issue keys into chunk by chunk size
        // and request each chunk separately.
        if let Pagination::ChunkSize(chunk_size) = self.pagination {
            self.paginated_issues(&method, chunk_size).await
        // If Pagination is not set to ChunkSize, use a single chunk request for all issues.
        } else {
            let issues = self.chunk_of_issues(&method, 0).await?;

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
