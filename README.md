# jira_query

[![Crates.io](https://img.shields.io/crates/v/jira_query.svg)](https://crates.io/crates/jira_query)
[![Apache-2.0 license](https://img.shields.io/crates/l/jira_query)](https://crates.io/crates/jira_query)
[![Documentation](https://docs.rs/jira_query/badge.svg)](https://docs.rs/jira_query)

[![CI tests](https://github.com/msuchane/jira_query/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/msuchane/jira_query/actions/workflows/rust-tests.yml)
[![Dependency status](https://deps.rs/repo/github/msuchane/jira_query/status.svg)](https://deps.rs/repo/github/msuchane/jira_query)

Access issues on a remote Jira instance.

## Description

The `jira_query` crate is a Rust library that can query a Jira instance using its REST API. It returns a strongly typed representation of the requested issues.

This library provides no functionality to create or modify issues. The access is read-only.

## Usage

### Basic anonymous query

Without logging in, search for a single ticket and check for its priority:

```rust
use tokio;
use jira_query::JiraInstance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let jira = JiraInstance::at("https://issues.redhat.com".to_string())?;

    let issue = jira.issue("CS-1113").await?;

    assert_eq!(issue.fields.priority.name, "Normal");

    Ok(())
}
```

### Advanced query

Use an API key to log into Jira. Search for all CentOS Stream tickets that are of the Blocker priority. Check that there is more than one ticket:

```rust
use tokio;
use jira_query::{Auth, JiraInstance, Pagination};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let jira = JiraInstance::at("https://bugzilla.redhat.com".to_string())?
        .authenticate(Auth::ApiKey("My API Key".to_string()))
        .paginate(Pagination::ChunkSize(32));

    let query = r#"project="CentOS Stream" AND priority=Blocker"#;

    let issues = jira.search(query).await?;

    assert!(issues.len() > 1);

    Ok(())
}
```

## See also

* [`bugzilla_query`](https://crates.io/crates/bugzilla_query), a similar interface to Bugzilla