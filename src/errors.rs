use thiserror::Error;

/// All errors that might occur in this crate.
#[derive(Error, Debug)]
pub enum JiraQueryError {
    #[error("Required issues are missing in the Jira response: {}.", .0.join(", "))]
    MissingIssues(Vec<String>),
    #[error("The Jira query returned no issues.")]
    NoIssues,
    #[error("Error in the Jira REST API.")]
    Rest(#[from] restson::Error),
}
