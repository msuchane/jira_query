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
