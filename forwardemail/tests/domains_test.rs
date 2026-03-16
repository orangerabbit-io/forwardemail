mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_domains_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("domains_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "domains", "list"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("example.org"));

    mock.assert();
}

#[test]
fn test_domains_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("domains_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "domains", "list"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\""))
        .stdout(predicate::str::contains("example.com"));

    mock.assert();
}

#[test]
fn test_domains_get() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/example.com")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("domain_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "domains", "get", "example.com"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("enhanced_protection"))
        .stdout(predicate::str::contains("yes"));

    mock.assert();
}

#[test]
fn test_domains_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v1/domains")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("domain_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "domains", "create", "example.com"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Domain created: example.com"));

    mock.assert();
}

#[test]
fn test_domains_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v1/domains/example.com")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(r#"{"deleted": true}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "domains", "delete", "example.com"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Domain deleted: example.com"));

    mock.assert();
}

#[test]
fn test_domains_not_found() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains/nonexistent.com")
        .with_status(404)
        .with_body(r#"{"error": "Domain not found"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "domains", "get", "nonexistent.com"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Not found"));

    mock.assert();
}

#[test]
fn test_domains_list_pagination() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/domains")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(common::fixture("domains_list.json"))
        .with_header("content-type", "application/json")
        .with_header("X-Page-Current", "1")
        .with_header("X-Page-Count", "3")
        .with_header("X-Item-Count", "25")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "domains", "list"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("Page 1 of 3 (25 total items)"));

    mock.assert();
}
