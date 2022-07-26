use chrono::prelude::*;
use tokio;

use jira_query::*;

/// A common convenience function to get anonymous access
/// to the Red Hat Jira instance.
fn rh_jira() -> JiraInstance {
    JiraInstance::at("https://issues.redhat.com".to_string()).unwrap()
}

/// Try accessing several public issues separately
/// to test the client and the deserialization.
#[tokio::test]
async fn access_issue() {
    let instance = rh_jira();
    let _issue1 = instance.issue("CS-1113").await.unwrap();
    let _issue2 = instance.issue("CS-1111").await.unwrap();
}

/// Try accessing several public issues at once
/// to test the client and the deserialization.
#[tokio::test]
async fn access_issues() {
    let instance = rh_jira();
    let issues = instance.issues(&["CS-1086", "CS-1084"]).await.unwrap();

    assert_eq!(issues.len(), 2);
}

/// Try accessing several public issues at once
/// to test the client and the deserialization.
#[tokio::test]
async fn access_missing_issue() {
    let instance = rh_jira();
    let issues = instance.issues(&["CS-11111111111111111111"]).await;

    assert!(issues.is_err());
    // TODO: This case should actually match JiraQueryError::NoIssues, not JiraQueryError::Rest. Fix it.
    assert!(matches!(issues.unwrap_err(), JiraQueryError::Rest(_)));
}

/// Check that the issue fields contain the expected values.
/// Work with fields that are standard in Jira, rather than custom extensions.
#[tokio::test]
async fn check_standard_fields() {
    let instance = rh_jira();
    let issue = instance.issue("CS-1113").await.unwrap();

    assert_eq!(issue.id, "14658916");
    assert_eq!(issue.key, "CS-1113");
    assert_eq!(
        issue.fields.summary,
        "Set gitlab.com/redhat/centos-stream/tests to public"
    );
    assert_eq!(issue.fields.assignee.unwrap().display_name, "aoife moloney");
    assert_eq!(issue.fields.reporter.display_name, "Don Zickus");
    assert_eq!(issue.fields.issuetype.name, "Task");
    assert_eq!(issue.fields.project.key, "CS");
    assert_eq!(issue.fields.project.name, "CentOS Stream");
    assert_eq!(issue.fields.priority.name, "Normal");
}

/// Check that the issue was created at the expected date, and that time deserialization
/// works as expected.
#[tokio::test]
async fn check_time() {
    let instance = rh_jira();
    let issue = instance.issue("CS-1113").await.unwrap();

    let date_created = chrono::Utc.ymd(2022, 5, 24);
    assert_eq!(issue.fields.created.date(), date_created);
}

/// Try accessing issues that match a JQL search.
#[tokio::test]
async fn search_for_issues() {
    let instance = rh_jira();
    let query = r#"project="CentOS Stream" AND priority = High"#;
    let _issues = instance.search(query).await.unwrap();
}

/// Make sure that no IDs on the input result in no bugs, without errors.
#[tokio::test]
async fn check_no_issues() {
    let instance = rh_jira();
    let issues = instance.issues(&[]).await;

    assert_eq!(issues.ok(), Some(vec![]));
}

/// Try accessing issues that match a JQL search.
/// Check that their number isn't limited by a page size.
#[tokio::test]
async fn search_for_issues_start_at() {
    let instance = rh_jira().paginate(Pagination::ChunkSize(30));
    let query = r#"project="CentOS Stream""#;
    let issues = instance.search(query).await.unwrap();
    // The query should result in hundreds of issues.
    assert!(issues.len() > 50);
    eprintln!("The number of issues: {}", issues.len());
}
