mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_account_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/account")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("account_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "account", "get"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("user@example.com"))
        .stdout(predicate::str::contains("John Doe"))
        .stdout(predicate::str::contains("enhanced_protection"));

    mock.assert();
}

#[test]
fn test_account_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/account")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("account_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "account", "get"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"email\""))
        .stdout(predicate::str::contains("user@example.com"));

    mock.assert();
}

#[test]
fn test_account_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/account")
        .with_status(401)
        .with_body(r#"{"error": "Invalid API key"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "account", "get"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
