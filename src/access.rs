// API documentation:
// * https://docs.atlassian.com/software/jira/docs/api/REST/latest/
// * https://docs.atlassian.com/jira-software/REST/latest/

use log::debug;
use restson::blocking::RestClient as BlockingRestClient;
use restson::{Error, Response as RestResponse, RestClient, RestPath};

use crate::issue_model::{Issue, JqlResults};

// The prefix of every subsequent REST request.
// This string comes directly after the host in the URL.
const REST_PREFIX: &str = "rest/api/2";

pub struct JiraInstance {
    pub host: String,
    pub auth: Auth,
}

pub enum Auth {
    Anonymous,
    ApiKey(String),
}

// API call with one &str parameter (e.g. "https://issues.redhat.com/rest/api/2/issue/RHELPLAN-12345")
impl RestPath<&str> for Issue {
    fn get_path(param: &str) -> Result<String, Error> {
        Ok(format!("{}/issue/{}", REST_PREFIX, param))
    }
}

// API call with several &str parameters representing the IDs of issues.
// TODO: Make this generic over &[&str] and &[String].
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

    pub fn issue(&self, id: &str) -> Result<Issue, Error> {
        let client = self.client()?;

        // Gets a bug by ID and deserializes the JSON to data variable
        let data: RestResponse<Issue> = client.get(id)?;
        let issue = data.into_inner();
        debug!("{:#?}", issue);

        Ok(issue)
    }

    pub fn issues(&self, ids: &[&str]) -> Result<Vec<Issue>, Error> {
        let client = self.client()?;

        // Gets a bug by ID and deserializes the JSON to data variable
        let data: RestResponse<JqlResults> = client.get(ids)?;
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
