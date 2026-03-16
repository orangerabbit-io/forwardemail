mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_members_update() {
    let mut server = Server::new();
    let mock = server
        .mock("PUT", "/v1/domains/example.com/members/member123")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"id":"member123","email":"member@example.com","group":"admin","created_at":"2024-01-01T00:00:00Z"}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "members",
        "update",
        "example.com",
        "member123",
        "--group",
        "admin",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Member updated: member@example.com (admin)"));

    mock.assert();
}

#[test]
fn test_members_remove() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v1/domains/example.com/members/member123")
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
        "members",
        "remove",
        "example.com",
        "member123",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Member removed: member123"));

    mock.assert();
}
