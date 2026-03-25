//! Live integration tests against the Forward Email API.
//!
//! Gated behind FORWARDEMAIL_LIVE_TEST=1. Requires FORWARDEMAIL_API_KEY set
//! (via env var or config file).
//!
//! Run: FORWARDEMAIL_LIVE_TEST=1 cargo test --test live_test -- --test-threads=1

mod common;

use serial_test::serial;

/// Skip test if FORWARDEMAIL_LIVE_TEST is not set.
fn require_live() {
    if std::env::var("FORWARDEMAIL_LIVE_TEST").unwrap_or_default() != "1" {
        eprintln!("Skipping live test (set FORWARDEMAIL_LIVE_TEST=1 to enable)");
        std::process::exit(0);
    }
}

/// Run the CLI with `--json` and return parsed JSON from stdout.
fn run_json(args: &[&str]) -> serde_json::Value {
    let mut cmd = common::binary();
    let output = cmd
        .arg("--json")
        .args(args)
        .output()
        .expect("failed to execute binary");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Command failed: {:?}\nstdout: {}\nstderr: {}",
        args,
        stdout,
        stderr
    );
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("Invalid JSON from {:?}: {}\nstdout: {}", args, e, stdout))
}

/// Run the CLI and assert success, returning stdout as string.
fn run_ok(args: &[&str]) -> String {
    let mut cmd = common::binary();
    let output = cmd.args(args).output().expect("failed to execute binary");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Command failed: {:?}\nstdout: {}\nstderr: {}",
        args,
        stdout,
        stderr
    );
    stdout
}

// ---------------------------------------------------------------------------
// Read-only tests — no mutations, safe to run against any account
// ---------------------------------------------------------------------------

#[test]
#[serial]
fn live_account_get() {
    require_live();
    let json = run_json(&["account", "get"]);
    assert!(json.get("email").is_some(), "account should have email field");
    assert!(json.get("plan").is_some(), "account should have plan field");
}

#[test]
#[serial]
fn live_domains_list() {
    require_live();
    let json = run_json(&["domains", "list"]);
    assert!(json.is_array(), "domains list should return an array");
}

#[test]
#[serial]
fn live_emails_list() {
    require_live();
    let json = run_json(&["emails", "list"]);
    assert!(json.is_array(), "emails list should return an array");
}

#[test]
#[serial]
fn live_emails_limit() {
    require_live();
    let json = run_json(&["emails", "limit"]);
    assert!(json.get("limit").is_some(), "should have limit field");
    assert!(json.get("count").is_some(), "should have count field");
}

#[test]
#[serial]
fn live_encrypt() {
    require_live();
    let json = run_json(&["encrypt", "forward-email=test@example.com"]);
    assert!(
        json.get("encrypted").is_some(),
        "should have encrypted field"
    );
}

#[test]
#[serial]
fn live_logs_download() {
    require_live();
    // Logs returns CSV, not JSON. Just verify success.
    let stdout = run_ok(&["logs", "download"]);
    // Might be empty if no logs, but command should succeed.
    let _ = stdout;
}

// ---------------------------------------------------------------------------
// Domain lifecycle: create → get → update → verify-records → verify-smtp → delete
// ---------------------------------------------------------------------------

/// Guard that deletes a domain on drop (cleanup on panic).
struct DomainGuard {
    domain: String,
}

impl Drop for DomainGuard {
    fn drop(&mut self) {
        let _ = common::binary()
            .args(["domains", "delete", &self.domain])
            .output();
    }
}

#[test]
#[serial]
fn live_domain_lifecycle() {
    require_live();
    let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let domain = format!("test-{}.example.com", id);
    let _guard = DomainGuard {
        domain: domain.clone(),
    };

    // Create
    let json = run_json(&["domains", "create", &domain]);
    assert_eq!(
        json.get("name").and_then(|v| v.as_str()),
        Some(domain.as_str()),
        "created domain name should match"
    );

    // Get
    let json = run_json(&["domains", "get", &domain]);
    assert_eq!(
        json.get("name").and_then(|v| v.as_str()),
        Some(domain.as_str())
    );

    // Update
    let stdout = run_ok(&[
        "domains",
        "update",
        &domain,
        "--has-adult-content-protection",
        "true",
    ]);
    assert!(stdout.contains("Domain updated"), "should confirm update");

    // Get again to verify update applied
    let json = run_json(&["domains", "get", &domain]);
    assert_eq!(
        json.get("has_adult_content_protection")
            .and_then(|v| v.as_bool()),
        Some(true),
        "update should have been applied"
    );

    // Verify records — expected to fail (400) since test domain has no DNS.
    // Just confirm the command runs without crashing.
    {
        let mut cmd = common::binary();
        let output = cmd
            .args(["domains", "verify-records", &domain])
            .output()
            .expect("failed to execute binary");
        // 400 is expected (no DNS records), but the command should not crash
        let _ = output;
    }

    // Verify SMTP — same situation, no real DNS
    {
        let mut cmd = common::binary();
        let output = cmd
            .args(["domains", "verify-smtp", &domain])
            .output()
            .expect("failed to execute binary");
        let _ = output;
    }

    // Delete (guard will also try, but explicit is better for assertion)
    let stdout = run_ok(&["domains", "delete", &domain]);
    assert!(stdout.contains("Domain deleted"), "should confirm deletion");
}

// ---------------------------------------------------------------------------
// Alias lifecycle: create → get → list → update → delete
// (within a temporary domain)
// ---------------------------------------------------------------------------

#[test]
#[serial]
fn live_alias_lifecycle() {
    require_live();
    let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let domain = format!("test-{}.example.com", id);
    let _guard = DomainGuard {
        domain: domain.clone(),
    };

    // Create domain first
    run_json(&["domains", "create", &domain]);

    // Create alias
    let json = run_json(&[
        "aliases",
        "create",
        &domain,
        "--name",
        "testuser",
        "--description",
        "Live test alias",
    ]);
    let alias_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .expect("alias create should return id")
        .to_string();

    // List aliases — should contain our alias
    let json = run_json(&["aliases", "list", &domain]);
    assert!(json.is_array(), "aliases list should be an array");
    let aliases = json.as_array().unwrap();
    assert!(
        aliases
            .iter()
            .any(|a| a.get("id").and_then(|v| v.as_str()) == Some(&alias_id)),
        "aliases list should contain created alias"
    );

    // Get alias
    let json = run_json(&["aliases", "get", &domain, &alias_id]);
    assert_eq!(
        json.get("name").and_then(|v| v.as_str()),
        Some("testuser")
    );
    assert_eq!(
        json.get("description").and_then(|v| v.as_str()),
        Some("Live test alias")
    );

    // Update alias
    let stdout = run_ok(&[
        "aliases",
        "update",
        &domain,
        &alias_id,
        "--description",
        "Updated description",
    ]);
    assert!(stdout.contains("Alias updated"), "should confirm update");

    // Verify update
    let json = run_json(&["aliases", "get", &domain, &alias_id]);
    assert_eq!(
        json.get("description").and_then(|v| v.as_str()),
        Some("Updated description")
    );

    // Delete alias
    let stdout = run_ok(&["aliases", "delete", &domain, &alias_id]);
    assert!(stdout.contains("Alias deleted"), "should confirm deletion");

    // Domain cleanup happens via guard
}

// ---------------------------------------------------------------------------
// Catch-all password lifecycle: create → list → delete
// (within a temporary domain)
// ---------------------------------------------------------------------------

#[test]
#[serial]
fn live_catch_all_password_lifecycle() {
    require_live();
    let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let domain = format!("test-{}.example.com", id);
    let _guard = DomainGuard {
        domain: domain.clone(),
    };

    // Create domain first
    run_json(&["domains", "create", &domain]);

    // Create catch-all password
    let json = run_json(&[
        "catch-all-passwords",
        "create",
        &domain,
        "--password",
        "TestP@ssw0rd!Live",
        "--description",
        "live test password",
    ]);
    let token_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .expect("catch-all password create should return id")
        .to_string();

    // List — should contain our password
    let json = run_json(&["catch-all-passwords", "list", &domain]);
    assert!(json.is_array());
    let passwords = json.as_array().unwrap();
    assert!(
        passwords
            .iter()
            .any(|p| p.get("id").and_then(|v| v.as_str()) == Some(&token_id)),
        "list should contain created password"
    );

    // Delete
    let stdout = run_ok(&["catch-all-passwords", "delete", &domain, &token_id]);
    assert!(
        stdout.contains("Catch-all password deleted"),
        "should confirm deletion"
    );

    // Domain cleanup via guard
}
