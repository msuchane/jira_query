use jira_query::*;

/// A common convenience function to get anonymous access
/// to the Red Hat Jira instance.
fn rh_jira() -> JiraInstance {
    JiraInstance {
        host: "https://issues.redhat.com".to_string(),
        auth: Auth::Anonymous,
    }
}

/// Try accessing several public issues to test the client and the deserialization.
#[test]
fn access_issues() {
    let instance = rh_jira();
    let _issue1 = instance.issue("CS-1113").unwrap();
    let _issue2 = instance.issue("CS-1111").unwrap();
    let _issues = instance.issues(&["CS-1086", "CS-1084"]);
}
