mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_emails_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/emails")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("emails_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "emails", "list"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("email456"))
        .stdout(predicate::str::contains("queued"))
        .stdout(predicate::str::contains("Test Email"));

    mock.assert();
}

#[test]
fn test_emails_get() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/emails/email456")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("email_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "emails", "get", "email456"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("email456"))
        .stdout(predicate::str::contains("sender@example.com"))
        .stdout(predicate::str::contains("Test Email"));

    mock.assert();
}

#[test]
fn test_emails_limit() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/emails/limit")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("email_limit.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "emails", "limit"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("5"))
        .stdout(predicate::str::contains("300"));

    mock.assert();
}

#[test]
fn test_emails_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v1/emails/email456")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"deleted": true}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "emails", "delete", "email456"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Email deleted: email456"));

    mock.assert();
}
