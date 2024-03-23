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

use chrono::{DateTime, NaiveDate, Utc};
/// This module replicates the fields in a Jira issue as strongly typed structs.
/// Any extra fields that come from a custom Jira configuration are captured
/// in the `extra` hash map in the parent struct.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The response from Jira to a JQL query,
/// which includes the list of requested issues and additional metadata.
#[derive(Clone, Debug, Deserialize)]
pub struct JqlResults {
    pub issues: Vec<Issue>,
    #[serde(flatten)]
    pub extra: Value,
}

/// A single Jira issue with all its fields.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub expand: String,
    pub fields: Fields,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A container for most fields of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fields {
    #[serde(rename = "lastViewed")]
    pub last_viewed: Option<DateTime<Utc>>,
    pub labels: Vec<String>,
    pub assignee: Option<User>,
    pub description: Option<String>,
    pub duedate: Option<NaiveDate>,
    // Both `versions` and `fixVersions` are optional fields and they might
    // either be missing or set to an empty list.
    // I'm consolidating both cases as an empty list, because I don't believe
    // that there's a meaningful semantic difference between them here.
    #[serde(default)]
    pub versions: Vec<Version>,
    #[serde(default)]
    #[serde(rename = "fixVersions")]
    pub fix_versions: Vec<Version>,
    pub reporter: User,
    pub status: Status,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub issuetype: IssueType,
    pub timeestimate: Option<i32>,
    pub aggregatetimeestimate: Option<i32>,
    pub timeoriginalestimate: Option<i32>,
    pub timespent: Option<i32>,
    pub aggregatetimespent: Option<i32>,
    pub aggregatetimeoriginalestimate: Option<i32>,
    pub progress: Progress,
    pub aggregateprogress: Progress,
    pub workratio: i64,
    pub summary: String,
    pub creator: User,
    pub project: Project,
    pub priority: Option<Priority>,
    pub components: Vec<Component>,
    pub watches: Watches,
    pub archiveddate: Option<DateTime<Utc>>,
    pub archivedby: Option<DateTime<Utc>>,
    pub resolution: Option<Resolution>,
    pub resolutiondate: Option<DateTime<Utc>>,
    pub comment: Option<Comments>,
    pub issuelinks: Vec<IssueLink>,
    pub votes: Votes,
    pub parent: Option<CondensedIssue>,
    pub subtasks: Vec<CondensedIssue>,
    pub environment: Option<String>,
    pub security: Option<Security>,
    #[serde(flatten)]
    pub extra: Value,
}

/// The representation of a Jira user account.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub active: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    pub key: String,
    pub name: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The representation of a Jira product version.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    pub id: String,
    pub description: Option<String>,
    pub name: String,
    pub archived: bool,
    pub released: bool,
    /// Jira stores `releaseDate` only as `YYYY-MM-DD`, so it can't Serialize, Deserialize to full `DateTime`.
    #[serde(rename = "releaseDate")]
    pub release_date: Option<NaiveDate>,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The Jira issue status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    pub extra: Value,
}

/// The category of a Jira issue status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusCategory {
    #[serde(rename = "colorName")]
    pub color_name: String,
    pub id: i32,
    pub key: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The resolution of a Jira issue when it's closed.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolution {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The type of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct IssueType {
    #[serde(rename = "avatarId")]
    pub avatar_id: Option<i32>,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    pub subtask: bool,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A project namespace that groups Jira issues.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: String,
    #[serde(rename = "projectCategory")]
    pub project_category: Option<ProjectCategory>,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The category of a Jira project.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectCategory {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The priority of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Priority {
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The component of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Component {
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// Users watching a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Watches {
    #[serde(rename = "isWatching")]
    pub is_watching: bool,
    #[serde(rename = "watchCount")]
    pub watch_count: i32,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The progress of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Progress {
    pub progress: i32,
    pub total: i32,
    #[serde(flatten)]
    pub extra: Value,
}

/// A comment below a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Comment {
    pub author: User,
    pub body: String,
    pub created: DateTime<Utc>,
    pub id: String,
    #[serde(rename = "updateAuthor")]
    pub update_author: User,
    pub updated: DateTime<Utc>,
    pub visibility: Option<Visibility>,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A container for all comments below a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Comments {
    pub comments: Vec<Comment>,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    pub total: i32,
    #[serde(flatten)]
    pub extra: Value,
}

/// A link from one Jira issue to another.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    pub extra: Value,
}

/// A Jira issue linked from another one.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedIssue {
    pub id: String,
    pub key: String,
    pub fields: LinkedIssueFields,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The reduced fields of a linked Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedIssueFields {
    pub issuetype: IssueType,
    pub priority: Option<Priority>,
    pub status: Status,
    pub summary: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The direction of a link to a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct IssueLinkType {
    pub id: String,
    pub inward: String,
    pub name: String,
    pub outward: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The votes for a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Votes {
    #[serde(rename = "hasVoted")]
    pub has_voted: bool,
    pub votes: i32,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A Jira avatar in several different sizes:
///
/// * `xsmall` = 16x16 px
/// * `small` = 24x24 px
/// * `medium` = 48x48 px
/// * `full` = maximum
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    pub extra: Value,
}

/// A minimal, reduced representation of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CondensedIssue {
    pub fields: CondensedFields,
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A minimal, reduced listing of the fields of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CondensedFields {
    pub issuetype: IssueType,
    pub priority: Option<Priority>,
    pub status: Status,
    pub summary: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The visibility of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Visibility {
    pub r#type: String,
    pub value: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The security level of a Jira issue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// TODO: This seems to be a generic container, similar to several other structs.
// In a future major release, try to consolidate them into one generic struct with:
// description, id, name.
// Also see if Serde can convert id to a number after all, somehow.
pub struct Security {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}
