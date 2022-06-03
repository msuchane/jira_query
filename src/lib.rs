// API documentation:
// * https://docs.atlassian.com/software/jira/docs/api/REST/latest/
// * https://docs.atlassian.com/jira-software/REST/latest/

use log::debug;
use restson::{Error, Response as RestResponse, RestClient, RestPath};

mod issue_model;

use crate::issue_model::{JqlResults, Issue};


// API call with one &str parameter (e.g. "https://issues.redhat.com/rest/api/2/issue/RHELPLAN-12345")
impl RestPath<&str> for Issue {
    fn get_path(param: &str) -> Result<String, Error> {
        Ok(format!("rest/api/2/issue/{}", param))
    }
}

pub fn issue(host: &str, issue: &str, api_key: &str) -> Result<Issue, Error> {
    let mut client = RestClient::builder().blocking(host)?;
    client.set_header("Authorization", &format!("Bearer {}", api_key))?;
    // Gets a bug by ID and deserializes the JSON to data variable
    let data: RestResponse<Issue> = client.get(issue)?;
    let issue = data.into_inner();
    debug!("{:#?}", issue);

    Ok(issue)
}

// API call with several &str parameters representing the IDs of issues.
// TODO: Make this generic over &[&str] and &[String].
impl RestPath<&[&str]> for JqlResults {
    fn get_path(params: &[&str]) -> Result<String, Error> {
        Ok(format!(
            "rest/api/2/search?jql=id%20in%20({})",
            params.join(",")
        ))
    }
}

pub fn issues(host: &str, issues: &[&str], api_key: &str) -> Result<Vec<Issue>, Error> {
    let mut client = RestClient::builder().blocking(host)?;
    client.set_header("Authorization", &format!("Bearer {}", api_key))?;
    // Gets a bug by ID and deserializes the JSON to data variable
    let data: RestResponse<JqlResults> = client.get(issues)?;
    let results = data.into_inner();
    debug!("{:#?}", results);

    // TODO: Note that the resulting list might be empty and still Ok
    Ok(results.issues)
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
