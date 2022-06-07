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
pub struct JiraInstance {
    pub host: String,
    pub auth: Auth,
}

/// The authentication method used to contact Jira.
pub enum Auth {
    Anonymous,
    ApiKey(String),
}

/// API call with one &str parameter (e.g. "https://issues.redhat.com/rest/api/2/issue/RHELPLAN-12345")
impl RestPath<&str> for Issue {
    fn get_path(param: &str) -> Result<String, Error> {
        Ok(format!("{}/issue/{}", REST_PREFIX, param))
    }
}

// TODO: Make this generic over &[&str] and &[String].
/// API call with several &str parameters representing the IDs of issues.
impl RestPath<&[&str]> for JqlResults {
    fn get_path(params: &[&str]) -> Result<String, Error> {
        Ok(format!(
            "{}/search?jql=id%20in%20({})",
            REST_PREFIX,
            params.join(",")
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

        // Gets a bug by ID and deserializes the JSON to data variable
        let data: RestResponse<JqlResults> = client.get(keys)?;
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
