mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_encrypt_no_auth_required() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v1/encrypt")
        .with_body(common::fixture("encrypt_response.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["encrypt", "v=spf1 a mx include:spf.forwardemail.net ~all"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .env_remove("FORWARDEMAIL_API_KEY")
        .env("HOME", "/tmp/nonexistent-forwardemail-test")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "v=spf1 a mx include:spf.forwardemail.net ~all",
        ));

    mock.assert();
}

#[test]
fn test_encrypt_json_output() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v1/encrypt")
        .with_body(common::fixture("encrypt_response.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--json",
        "encrypt",
        "v=spf1 a mx include:spf.forwardemail.net ~all",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .env_remove("FORWARDEMAIL_API_KEY")
    .env("HOME", "/tmp/nonexistent-forwardemail-test")
    .assert()
    .success()
    .stdout(predicate::str::contains("\"encrypted\""));

    mock.assert();
}
