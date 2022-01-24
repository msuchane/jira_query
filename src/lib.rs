// API documentation:
// * https://docs.atlassian.com/software/jira/docs/api/REST/latest/
// * https://docs.atlassian.com/jira-software/REST/latest/

use std::collections::HashMap;

use restson::{Error as RestError, Response as RestResponse, RestClient, RestPath};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct JiraIssue {
    pub id: String,
    pub key: String,
    pub expand: String,
    pub fields: Fields,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "lastViewed")]
    pub last_viewed: Option<String>,
    pub labels: Vec<String>,
    pub versions: Vec<Version>,
    pub assignee: User,
    pub description: Option<String>,
    pub duedate: Option<String>,
    #[serde(rename = "fixVersions")]
    pub fix_versions: Vec<Version>,
    pub reporter: User,
    pub status: Status,
    pub created: String,
    pub updated: String,
    pub issuetype: IssueType,
    pub timeestimate: Option<i32>,
    pub aggregatetimeestimate: Option<i32>,
    pub timeoriginalestimate: Option<i32>,
    pub timespent: Option<i32>,
    pub aggregatetimespent: Option<i32>,
    pub aggregatetimeoriginalestimate: Option<i32>,
    pub progress: Progress,
    pub aggregateprogress: Progress,
    pub workratio: i32,
    pub summary: String,
    pub creator: User,
    pub project: Project,
    pub priority: Priority,
    pub components: Vec<Component>,
    pub watches: Watches,
    pub archiveddate: Option<String>,
    pub archivedby: Option<String>,
    pub resolution: Option<Resolution>,
    pub resolutiondate: Option<String>,
    pub comment: Comments,
    pub issuelinks: Vec<IssueLink>,
    pub votes: Votes,
    pub parent: Option<CondensedIssue>,
    pub subtasks: Vec<CondensedIssue>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub active: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: String,
    pub description: Option<String>,
    pub name: String,
    pub archived: bool,
    pub released: bool,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct StatusCategory {
    #[serde(rename = "colorName")]
    pub color_name: String,
    pub id: i32,
    pub key: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Resolution {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct IssueType {
    #[serde(rename = "avatarId")]
    pub avatar_id: i32,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    pub subtask: bool,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: String,
    #[serde(rename = "projectCategory")]
    pub project_category: ProjectCategory,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectCategory {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Priority {
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Watches {
    #[serde(rename = "isWatching")]
    pub is_watching: bool,
    #[serde(rename = "watchCount")]
    pub watch_count: i32,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Progress {
    pub progress: i32,
    pub total: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub author: User,
    pub body: String,
    pub created: String,
    pub id: String,
    #[serde(rename = "updateAuthor")]
    pub update_author: User,
    pub updated: String,
    pub visibility: Option<Visibility>,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Comments {
    pub comments: Vec<Comment>,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    pub total: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct IssueLink {
    pub id: String,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Option<LinkedIssue>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Option<LinkedIssue>,
    #[serde(rename = "type")]
    pub link_type: IssueLinkType,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct LinkedIssue {
    pub id: String,
    pub key: String,
    pub fields: LinkedIssueFields,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct LinkedIssueFields {
    pub issuetype: IssueType,
    pub priority: Option<Priority>,
    pub status: Status,
    pub summary: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct IssueLinkType {
    pub id: String,
    pub inward: String,
    pub name: String,
    pub outward: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Votes {
    #[serde(rename = "hasVoted")]
    pub has_voted: bool,
    pub votes: i32,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct AvatarUrls {
    #[serde(rename = "16x16")]
    pub xsmall: String,
    #[serde(rename = "24x24")]
    pub small: String,
    #[serde(rename = "32x32")]
    pub medium: String,
    #[serde(rename = "48x48")]
    pub full: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct CondensedIssue {
    pub fields: CondensedFields,
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct CondensedFields {
    pub issuetype: IssueType,
    pub priority: Priority,
    pub status: Status,
    pub summary: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Visibility {
    pub r#type: String,
    pub value: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// API call with one String parameter (e.g. "https://issues.redhat.com/rest/api/2/issue/RHELPLAN-12345")
impl RestPath<&str> for JiraIssue {
    fn get_path(param: &str) -> Result<String, RestError> {
        Ok(format!("rest/api/2/issue/{}", param))
    }
}

pub fn issue(host: &str, issue: &str, api_key: &str) -> JiraIssue {
    let mut client = RestClient::builder().blocking(host).unwrap();
    client
        .set_header("Authorization", &format!("Bearer {}", api_key))
        .unwrap();
    // Gets a bug by ID and deserializes the JSON to data variable
    let data: RestResponse<JiraIssue> = client.get(issue).unwrap();
    let issue = data.into_inner();
    println!("{:#?}", issue);

    issue
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
