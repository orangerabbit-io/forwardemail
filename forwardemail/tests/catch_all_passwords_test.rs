mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_catch_all_passwords_list() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/example.com/catch-all-passwords")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("catch_all_passwords_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "catch-all-passwords",
        "list",
        "example.com",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("pw123"))
    .stdout(predicate::str::contains("My catch-all password"));

    mock.assert();
}

#[test]
fn test_catch_all_passwords_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v1/domains/example.com/catch-all-passwords")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"id":"pw456","description":"New password","created_at":"2024-01-01T00:00:00Z"}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "catch-all-passwords",
        "create",
        "example.com",
        "--password",
        "mysecretpass",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Catch-all password created: pw456"));

    mock.assert();
}

#[test]
fn test_catch_all_passwords_delete() {
    let mut server = Server::new();
    let mock = server
        .mock(
            "DELETE",
            "/v1/domains/example.com/catch-all-passwords/pw123",
        )
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"deleted": true}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "catch-all-passwords",
        "delete",
        "example.com",
        "pw123",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Catch-all password deleted: pw123"));

    mock.assert();
}
