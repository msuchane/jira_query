// Enable additional clippy lints by default.
#![warn(
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::clone_on_ref_ptr,
    clippy::todo
)]
// Forbid unsafe code in this program.
#![forbid(unsafe_code)]

mod access;
mod errors;
mod issue_model;

pub use access::{Auth, JiraInstance, Pagination};
pub use errors::JiraQueryError;
pub use issue_model::{
    AvatarUrls, Comment, Comments, Component, CondensedFields, CondensedIssue, Fields, Issue,
    IssueLink, IssueLinkType, IssueType, LinkedIssue, LinkedIssueFields, Priority, Progress,
    Project, ProjectCategory, Resolution, Status, StatusCategory, User, Version, Visibility, Votes,
    Watches,
};
// Re-export JSON Value because it's an integral part of the issue model.
pub use serde_json::Value;
