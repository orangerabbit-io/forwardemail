mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_aliases_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/example.com/aliases")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("aliases_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "aliases", "list", "example.com"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("alias123"))
        .stdout(predicate::str::contains("info"));

    mock.assert();
}

#[test]
fn test_aliases_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/example.com/aliases")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("aliases_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "--json",
        "aliases",
        "list",
        "example.com",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"id\""))
    .stdout(predicate::str::contains("alias123"));

    mock.assert();
}

#[test]
fn test_aliases_get() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/example.com/aliases/alias123")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("alias_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "aliases",
        "get",
        "example.com",
        "alias123",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("alias123"))
    .stdout(predicate::str::contains("info"))
    .stdout(predicate::str::contains("user@example.com, other@example.com"));

    mock.assert();
}

#[test]
fn test_aliases_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v1/domains/example.com/aliases")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("alias_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "aliases",
        "create",
        "example.com",
        "--name",
        "info",
        "--recipients",
        "user@example.com",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Alias created: info (alias123)"));

    mock.assert();
}

#[test]
fn test_aliases_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v1/domains/example.com/aliases/alias123")
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
        "aliases",
        "delete",
        "example.com",
        "alias123",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("Alias deleted: alias123"));

    mock.assert();
}
