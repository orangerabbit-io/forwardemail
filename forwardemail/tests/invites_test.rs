mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_invites_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v1/domains/example.com/invites")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"email":"invitee@example.com","group":"admin","created_at":"2024-01-01T00:00:00Z"}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "invites",
        "create",
        "example.com",
        "--email",
        "invitee@example.com",
        "--group",
        "admin",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Invite created for invitee@example.com (admin)"));

    mock.assert();
}

#[test]
fn test_invites_accept() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/example.com/invites")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"message":"ok"}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "invites",
        "accept",
        "example.com",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Invite accepted for domain: example.com"));

    mock.assert();
}

#[test]
fn test_invites_remove() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v1/domains/example.com/invites")
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
        "invites",
        "remove",
        "example.com",
        "--email",
        "invitee@example.com",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Invite removed for invitee@example.com"));

    mock.assert();
}
