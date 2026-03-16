mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_logs_download() {
    let csv_data = "date,from,to,subject,status\n2024-01-01,sender@example.com,recipient@example.com,Test,sent\n";

    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/logs/download")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .with_body(csv_data)
        .with_header("content-type", "text/csv")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "logs", "download"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("date,from,to,subject,status"))
        .stdout(predicate::str::contains("sender@example.com"));

    mock.assert();
}

#[test]
fn test_logs_download_with_domain_filter() {
    let csv_data = "date,from,to,subject,status\n2024-01-01,sender@example.com,recipient@example.com,Test,sent\n";

    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/logs/download")
        .match_header(
            "authorization",
            mockito::Matcher::Regex("Basic .+".to_string()),
        )
        .match_query(mockito::Matcher::UrlEncoded(
            "domain".into(),
            "example.com".into(),
        ))
        .with_body(csv_data)
        .with_header("content-type", "text/csv")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "test-key",
        "logs",
        "download",
        "--domain",
        "example.com",
    ])
    .env("FORWARDEMAIL_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("sender@example.com"));

    mock.assert();
}
