use jira_query::*;

/// A common convenience function to get anonymous access
/// to the Red Hat Jira instance.
fn rh_jira() -> JiraInstance {
    JiraInstance {
        host: "https://issues.redhat.com".to_string(),
        auth: Auth::Anonymous,
    }
}

/// Try accessing a public issue
#[test]
fn access_issue() {
    let instance = rh_jira();
    let _issue = instance.issue("CS-1113").unwrap();
}
