use std::default::Default;

use jira_query::*;

/// A common convenience function to get anonymous access
/// to the Red Hat Jira instance.
fn rh_jira() -> JiraInstance {
    JiraInstance {
        host: "https://issues.redhat.com".to_string(),
        ..Default::default()
    }
}

/// Try accessing several public issues separately
/// to test the client and the deserialization.
#[test]
fn access_issue() {
    let instance = rh_jira();
    let _issue1 = instance.issue("CS-1113").unwrap();
    let _issue2 = instance.issue("CS-1111").unwrap();
}

/// Try accessing several public issues at once
/// to test the client and the deserialization.
#[test]
fn access_issues() {
    let instance = rh_jira();
    let issues = instance.issues(&["CS-1086", "CS-1084"]).unwrap();

    assert_eq!(issues.len(), 2);
}

/// Check that the issue fields contain the expected values.
/// Work with fields that are standard in Jira, rather than custom extensions.
#[test]
fn check_standard_fields() {
    let instance = rh_jira();
    let issue = instance.issue("CS-1113").unwrap();

    assert_eq!(issue.id, "14658916");
    assert_eq!(issue.key, "CS-1113");
    assert_eq!(
        issue.fields.summary,
        "Set gitlab.com/redhat/centos-stream/tests to public"
    );
    assert_eq!(issue.fields.assignee.display_name, "aoife moloney");
    assert_eq!(issue.fields.reporter.display_name, "Don Zickus");
    assert_eq!(issue.fields.issuetype.name, "Task");
    assert_eq!(issue.fields.project.key, "CS");
    assert_eq!(issue.fields.project.name, "CentOS Stream");
    assert_eq!(issue.fields.priority.name, "Normal");
}
