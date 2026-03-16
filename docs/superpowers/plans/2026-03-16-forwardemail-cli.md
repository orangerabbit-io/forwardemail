# Forward Email CLI + TUI Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI and TUI for the complete Forward Email API, structured as a Cargo workspace with a reusable library crate.

**Architecture:** Two-crate workspace — `forwardemail-lib` (API client, models, config) and `forwardemail` (binary with CLI commands + TUI). Blocking HTTP via reqwest. Output as markdown tables via tabled or JSON. TUI via ratatui for read-only browsing.

**Tech Stack:** Rust 2021, reqwest (blocking), clap (derive), tabled, ratatui, crossterm, serde, anyhow, mockito, assert_cmd

**Spec:** `docs/superpowers/specs/2026-03-16-forwardemail-cli-design.md`

**Reference project:** `/home/alindsay/projects/orangerabbit-io/updown-io/` — follow its patterns for client, config, output, models, commands, and tests.

---

## File Structure

### Library crate: `forwardemail-lib/`

| File | Responsibility |
|---|---|
| `Cargo.toml` | Library crate manifest |
| `src/lib.rs` | Public re-exports |
| `src/client.rs` | HTTP client with Basic Auth, error mapping, unauthenticated mode |
| `src/config.rs` | 3-tier config resolution (flag > env > file) |
| `src/models/mod.rs` | Module re-exports |
| `src/models/account.rs` | Account model + row + request structs |
| `src/models/domain.rs` | Domain model + row + request structs |
| `src/models/alias.rs` | Alias model + row + request structs |
| `src/models/email.rs` | Email model + row + request structs + EmailLimit |
| `src/models/log.rs` | Log download types |
| `src/models/invite.rs` | Invite model + row + request structs |
| `src/models/member.rs` | Member model + row + request structs |
| `src/models/catch_all_password.rs` | CatchAllPassword model + row + request structs |
| `src/models/encrypt.rs` | Encrypt request/response structs |

### Binary crate: `forwardemail/`

| File | Responsibility |
|---|---|
| `Cargo.toml` | Binary crate manifest |
| `src/main.rs` | Clap CLI definition, dispatch, error handling |
| `src/output.rs` | OutputMode enum, markdown table, JSON, KV, confirm, raw |
| `src/cmd/mod.rs` | Module re-exports |
| `src/cmd/account.rs` | Account subcommands (create, get, update) |
| `src/cmd/domains.rs` | Domain subcommands (list, create, get, update, delete, verify-records, verify-smtp) |
| `src/cmd/aliases.rs` | Alias subcommands (list, create, get, update, delete, generate-password) |
| `src/cmd/emails.rs` | Email subcommands (list, send, get, delete, limit) |
| `src/cmd/logs.rs` | Log subcommands (download) |
| `src/cmd/invites.rs` | Invite subcommands (create, accept, remove) |
| `src/cmd/members.rs` | Member subcommands (update, remove) |
| `src/cmd/catch_all_passwords.rs` | CatchAllPassword subcommands (list, create, delete) |
| `src/cmd/encrypt.rs` | Encrypt subcommand |
| `src/tui/mod.rs` | TUI entry point |
| `src/tui/app.rs` | App state machine |
| `src/tui/ui.rs` | Rendering (header, content, footer) |
| `src/tui/views/mod.rs` | View module re-exports |
| `src/tui/views/dashboard.rs` | Account dashboard view |
| `src/tui/views/domains.rs` | Domain list + detail views |
| `src/tui/views/aliases.rs` | Alias list + detail views |
| `src/tui/views/emails.rs` | Email list + detail views |

### Integration tests: `forwardemail/tests/`

| File | Responsibility |
|---|---|
| `common/mod.rs` | Test helpers (fixture loader, binary invoker) |
| `fixtures/*.json` | JSON response fixtures for all resources |
| `account_test.rs` | Account command tests |
| `domains_test.rs` | Domain command tests |
| `aliases_test.rs` | Alias command tests |
| `emails_test.rs` | Email command tests |
| `logs_test.rs` | Log command tests |
| `invites_test.rs` | Invite command tests |
| `members_test.rs` | Member command tests |
| `catch_all_passwords_test.rs` | CatchAllPassword command tests |
| `encrypt_test.rs` | Encrypt command tests |

### Root files

| File | Responsibility |
|---|---|
| `Cargo.toml` | Workspace manifest |
| `flake.nix` | Nix build + dev shell |
| `.releaserc.json` | Semantic release config |
| `.github/workflows/release.yml` | CI release workflow |
| `CLAUDE.md` | Project-specific Claude instructions |
| `.gitignore` | Git ignore rules |

---

## Chunk 1: Project Scaffolding

### Task 1: Workspace and Crate Setup

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `forwardemail-lib/Cargo.toml`
- Create: `forwardemail/Cargo.toml`
- Create: `forwardemail-lib/src/lib.rs`
- Create: `forwardemail/src/main.rs`
- Modify: `.gitignore`

- [ ] **Step 1: Create workspace root Cargo.toml**

```toml
[workspace]
members = ["forwardemail-lib", "forwardemail"]
resolver = "2"
```

- [ ] **Step 2: Create library crate Cargo.toml**

```toml
[package]
name = "forwardemail-lib"
version = "0.1.0"
edition = "2021"
description = "Rust client library for the Forward Email API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/orangerabbit-io/forwardemail"
readme = "../README.md"
keywords = ["forwardemail", "email", "api", "client"]
categories = ["api-bindings"]

[dependencies]
anyhow = "1"
reqwest = { version = "0.12", features = ["blocking", "json", "gzip"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tabled = "0.17"
toml = "0.8"

[dev-dependencies]
serial_test = "3"
```

- [ ] **Step 3: Create binary crate Cargo.toml**

```toml
[package]
name = "forwardemail"
version = "0.1.0"
edition = "2021"
description = "Command-line interface and TUI for the Forward Email API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/orangerabbit-io/forwardemail"
homepage = "https://github.com/orangerabbit-io/forwardemail"
readme = "../README.md"
keywords = ["forwardemail", "email", "cli", "tui"]
categories = ["command-line-utilities"]

[dependencies]
forwardemail-lib = { path = "../forwardemail-lib" }
anyhow = "1"
clap = { version = "4", features = ["derive"] }
serde_json = "1"
tabled = "0.17"
ratatui = "0.29"
crossterm = "0.28"
flate2 = "1"

[dev-dependencies]
mockito = "1"
assert_cmd = "2"
predicates = "3"
serial_test = "3"
```

- [ ] **Step 4: Create minimal lib.rs**

```rust
// forwardemail-lib/src/lib.rs
pub mod client;
pub mod config;
pub mod models;
```

- [ ] **Step 5: Create minimal main.rs that compiles**

```rust
// forwardemail/src/main.rs
fn main() {
    println!("forwardemail");
}
```

- [ ] **Step 6: Update .gitignore**

```
/target
forwardemail-lib/target
forwardemail/target
.env
```

- [ ] **Step 7: Create stub modules so the workspace compiles**

Create `forwardemail-lib/src/client.rs`:
```rust
// Placeholder — implemented in Task 3
```

Create `forwardemail-lib/src/config.rs`:
```rust
// Placeholder — implemented in Task 2
```

Create `forwardemail-lib/src/models/mod.rs`:
```rust
// Placeholder — implemented in Task 5+
```

- [ ] **Step 8: Verify workspace compiles**

Run: `cargo check --workspace`
Expected: compiles with no errors

- [ ] **Step 9: Commit**

```bash
git add -A
git commit -m "feat: scaffold workspace with forwardemail-lib and forwardemail crates"
```

### Task 2: Config Module

**Files:**
- Modify: `forwardemail-lib/src/config.rs`

Reference: `/home/alindsay/projects/orangerabbit-io/updown-io/src/config.rs`

- [ ] **Step 1: Write config unit tests**

```rust
// forwardemail-lib/src/config.rs
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_base_url() -> String {
    "https://api.forwardemail.net".to_string()
}

impl Config {
    pub fn load(api_key_override: Option<&str>) -> Result<Self> {
        let base_url =
            std::env::var("FORWARDEMAIL_BASE_URL").unwrap_or_else(|_| default_base_url());

        // 1. CLI flag override
        if let Some(key) = api_key_override {
            return Ok(Config {
                api_key: key.to_string(),
                base_url,
            });
        }

        // 2. Environment variable
        if let Ok(key) = std::env::var("FORWARDEMAIL_API_KEY") {
            return Ok(Config {
                api_key: key,
                base_url,
            });
        }

        // 3. Config file
        let path = Self::config_path()?;
        let contents = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "No API key found. Create a config file at {} with:\n\n  api_key = \"your-api-key\"\n\nOr set FORWARDEMAIL_API_KEY environment variable.",
                path.display()
            )
        })?;

        let mut config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file at {}", path.display()))?;

        if std::env::var("FORWARDEMAIL_BASE_URL").is_ok() {
            config.base_url = base_url;
        }

        Ok(config)
    }

    /// Returns the base URL without requiring an API key.
    /// Used by unauthenticated endpoints like encrypt.
    pub fn base_url_only() -> String {
        std::env::var("FORWARDEMAIL_BASE_URL").unwrap_or_else(|_| default_base_url())
    }

    fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".config/forwardemail/config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_cli_flag_override_takes_priority() {
        let config = Config::load(Some("flag-key")).unwrap();
        assert_eq!(config.api_key, "flag-key");
    }

    #[test]
    #[serial]
    fn test_env_var_override() {
        std::env::set_var("FORWARDEMAIL_API_KEY", "env-key");
        let config = Config::load(None).unwrap();
        assert_eq!(config.api_key, "env-key");
        std::env::remove_var("FORWARDEMAIL_API_KEY");
    }

    #[test]
    #[serial]
    fn test_base_url_env_override() {
        std::env::set_var("FORWARDEMAIL_BASE_URL", "http://localhost:9999");
        let config = Config::load(Some("key")).unwrap();
        assert_eq!(config.base_url, "http://localhost:9999");
        std::env::remove_var("FORWARDEMAIL_BASE_URL");
    }

    #[test]
    #[serial]
    fn test_default_base_url() {
        std::env::remove_var("FORWARDEMAIL_BASE_URL");
        let config = Config::load(Some("key")).unwrap();
        assert_eq!(config.base_url, "https://api.forwardemail.net");
    }

    #[test]
    #[serial]
    fn test_missing_api_key_errors() {
        std::env::remove_var("FORWARDEMAIL_API_KEY");
        let result = Config::load(None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("No API key found") || err.contains("config"));
    }

    #[test]
    fn test_base_url_only() {
        let url = Config::base_url_only();
        assert!(!url.is_empty());
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p forwardemail-lib -- config`
Expected: all 6 tests pass

- [ ] **Step 3: Commit**

```bash
git add forwardemail-lib/src/config.rs
git commit -m "feat: add config module with 3-tier API key resolution"
```

### Task 3: HTTP Client

**Files:**
- Modify: `forwardemail-lib/src/client.rs`

Reference: `/home/alindsay/projects/orangerabbit-io/updown-io/src/client.rs`

The Forward Email API uses HTTP Basic Auth (API token as username, empty password) rather than an `X-API-KEY` header.

- [ ] **Step 1: Implement client with Basic Auth**

```rust
// forwardemail-lib/src/client.rs
use anyhow::{bail, Context, Result};
use reqwest::blocking::{Client as HttpClient, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct Client {
    http: HttpClient,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String, base_url: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Client {
            http,
            base_url,
            api_key,
        })
    }

    /// Create a client without authentication for endpoints like encrypt.
    pub fn unauthenticated(base_url: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Client {
            http,
            base_url,
            api_key: String::new(),
        })
    }

    pub fn get(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.get(&url);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        Self::check_status(resp)
    }

    pub fn get_with_params(&self, path: &str, params: &[(&str, &str)]) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.get(&url).query(params);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        Self::check_status(resp)
    }

    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self.get(path)?;
        resp.json::<T>().context("Failed to parse JSON response")
    }

    #[allow(dead_code)]
    pub fn get_json_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let resp = self.get_with_params(path, params)?;
        resp.json::<T>().context("Failed to parse JSON response")
    }

    pub fn get_bytes(&self, path: &str, params: &[(&str, &str)]) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.get(&url).query(params);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        let resp = Self::check_status(resp)?;
        resp.bytes()
            .map(|b| b.to_vec())
            .context("Failed to read response bytes")
    }

    pub fn post(&self, path: &str, body: &HashMap<String, serde_json::Value>) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.post(&url).json(body);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn put(&self, path: &str, body: &HashMap<String, serde_json::Value>) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.put(&url).json(body);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: PUT {}", url))?;
        Self::check_status(resp)
    }

    pub fn delete(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.delete(&url);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: DELETE {}", url))?;
        Self::check_status(resp)
    }

    pub fn delete_with_body(
        &self,
        path: &str,
        body: &HashMap<String, serde_json::Value>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.delete(&url).json(body);
        if !self.api_key.is_empty() {
            req = req.basic_auth(&self.api_key, Some(""));
        }
        let resp = req
            .send()
            .with_context(|| format!("Request failed: DELETE {}", url))?;
        Self::check_status(resp)
    }

    fn check_status(resp: Response) -> Result<Response> {
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }
        let url = resp.url().to_string();
        let body = resp.text().unwrap_or_default();
        match status.as_u16() {
            400 => bail!("Bad request (HTTP {}): {}", status, body),
            401 | 403 => bail!("Authentication failed (HTTP {}): {}", status, body),
            404 => bail!("Not found (HTTP {}): {}", status, body),
            422 => bail!("Validation error (HTTP {}): {}", status, body),
            429 => bail!("Rate limited (HTTP {}): {}", status, body),
            _ => bail!("API error (HTTP {}) for {}: {}", status, url, body),
        }
    }
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo check -p forwardemail-lib`
Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add forwardemail-lib/src/client.rs
git commit -m "feat: add HTTP client with Basic Auth and unauthenticated mode"
```

### Task 4: Output Module

**Files:**
- Create: `forwardemail/src/output.rs`

Reference: `/home/alindsay/projects/orangerabbit-io/updown-io/src/output.rs`

Key difference: uses `Style::markdown()` instead of default tabled style.

- [ ] **Step 1: Implement output module**

```rust
// forwardemail/src/output.rs
use tabled::{settings::Style, Table, Tabled};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Table,
    Json,
}

impl OutputMode {
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            OutputMode::Json
        } else {
            OutputMode::Table
        }
    }
}

pub fn print_json(value: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    );
}

pub fn print_table<T: Tabled>(items: &[T]) {
    if items.is_empty() {
        println!("No results.");
        return;
    }
    let mut table = Table::new(items);
    table.with(Style::markdown());
    println!("{}", table);
}

pub fn print_kv(pairs: &[(&str, String)]) {
    let max_key_len = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, value) in pairs {
        println!("{:>width$}:  {}", key, value, width = max_key_len);
    }
}

pub fn print_confirm(message: &str) {
    println!("{}", message);
}

pub fn print_raw(text: &str) {
    print!("{}", text);
}

/// Print pagination footer from response headers.
pub fn print_pagination(current: u32, total_pages: u32, total_items: u32) {
    println!("Page {} of {} ({} total items)", current, total_pages, total_items);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_mode_from_flag() {
        assert_eq!(OutputMode::from_json_flag(true), OutputMode::Json);
        assert_eq!(OutputMode::from_json_flag(false), OutputMode::Table);
    }
}
```

- [ ] **Step 2: Update main.rs to declare output module**

```rust
// forwardemail/src/main.rs
mod output;

fn main() {
    println!("forwardemail");
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p forwardemail`
Expected: compiles

- [ ] **Step 4: Run output tests**

Run: `cargo test -p forwardemail -- output`
Expected: 1 test passes

- [ ] **Step 5: Commit**

```bash
git add forwardemail/src/output.rs forwardemail/src/main.rs
git commit -m "feat: add output module with markdown table, JSON, KV formatting"
```

---

## Chunk 2: Models

### Task 5: Account Model

**Files:**
- Create: `forwardemail-lib/src/models/account.rs`
- Modify: `forwardemail-lib/src/models/mod.rs`

- [ ] **Step 1: Write account model with tests**

```rust
// forwardemail-lib/src/models/account.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct AccountRow {
    #[tabled(rename = "EMAIL")]
    pub email: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "PLAN")]
    pub plan: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&Account> for AccountRow {
    fn from(a: &Account) -> Self {
        let name = match (&a.given_name, &a.family_name) {
            (Some(g), Some(f)) => format!("{} {}", g, f),
            (Some(g), None) => g.clone(),
            (None, Some(f)) => f.clone(),
            (None, None) => "-".to_string(),
        };
        AccountRow {
            email: a.email.clone().unwrap_or("-".to_string()),
            name,
            plan: a.plan.clone().unwrap_or("-".to_string()),
            created_at: a.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_account() {
        let json = r#"{
            "email": "user@example.com",
            "given_name": "John",
            "family_name": "Doe",
            "plan": "enhanced_protection",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-06-01T00:00:00Z"
        }"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.email, Some("user@example.com".to_string()));
        assert_eq!(account.plan, Some("enhanced_protection".to_string()));
    }

    #[test]
    fn test_account_row_full_name() {
        let account = Account {
            email: Some("user@example.com".to_string()),
            given_name: Some("John".to_string()),
            family_name: Some("Doe".to_string()),
            avatar_url: None,
            plan: Some("free".to_string()),
            created_at: Some("2024-01-01".to_string()),
            updated_at: None,
        };
        let row = AccountRow::from(&account);
        assert_eq!(row.name, "John Doe");
    }

    #[test]
    fn test_deserialize_minimal_account() {
        let json = r#"{}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.email, None);
        let row = AccountRow::from(&account);
        assert_eq!(row.email, "-");
        assert_eq!(row.name, "-");
    }
}
```

- [ ] **Step 2: Update models/mod.rs**

```rust
// forwardemail-lib/src/models/mod.rs
pub mod account;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p forwardemail-lib -- models::account`
Expected: 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add forwardemail-lib/src/models/
git commit -m "feat: add account model with row type and serde tests"
```

### Task 6: Domain Model

**Files:**
- Create: `forwardemail-lib/src/models/domain.rs`
- Modify: `forwardemail-lib/src/models/mod.rs`

- [ ] **Step 1: Write domain model with tests**

```rust
// forwardemail-lib/src/models/domain.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub name: String,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub max_recipients_per_alias: Option<u32>,
    #[serde(default)]
    pub smtp_port: Option<String>,
    #[serde(default)]
    pub has_adult_content_protection: Option<bool>,
    #[serde(default)]
    pub has_phishing_protection: Option<bool>,
    #[serde(default)]
    pub has_executable_protection: Option<bool>,
    #[serde(default)]
    pub has_virus_protection: Option<bool>,
    #[serde(default)]
    pub has_recipient_verification: Option<bool>,
    #[serde(default)]
    pub retention_days: Option<u32>,
    #[serde(default)]
    pub has_mx_record: Option<bool>,
    #[serde(default)]
    pub has_txt_record: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct DomainRow {
    #[tabled(rename = "DOMAIN")]
    pub name: String,
    #[tabled(rename = "PLAN")]
    pub plan: String,
    #[tabled(rename = "MX")]
    pub mx: String,
    #[tabled(rename = "TXT")]
    pub txt: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&Domain> for DomainRow {
    fn from(d: &Domain) -> Self {
        DomainRow {
            name: d.name.clone(),
            plan: d.plan.clone().unwrap_or("-".to_string()),
            mx: d
                .has_mx_record
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
            txt: d
                .has_txt_record
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
            created_at: d.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_domain() {
        let json = r#"{
            "name": "example.com",
            "plan": "enhanced_protection",
            "has_mx_record": true,
            "has_txt_record": true,
            "retention_days": 30,
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let domain: Domain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.has_mx_record, Some(true));
        assert_eq!(domain.retention_days, Some(30));
    }

    #[test]
    fn test_domain_row() {
        let domain = Domain {
            name: "example.com".to_string(),
            plan: Some("free".to_string()),
            max_recipients_per_alias: None,
            smtp_port: None,
            has_adult_content_protection: None,
            has_phishing_protection: None,
            has_executable_protection: None,
            has_virus_protection: None,
            has_recipient_verification: None,
            retention_days: None,
            has_mx_record: Some(true),
            has_txt_record: Some(false),
            created_at: Some("2024-01-01".to_string()),
            updated_at: None,
        };
        let row = DomainRow::from(&domain);
        assert_eq!(row.mx, "yes");
        assert_eq!(row.txt, "no");
    }
}
```

- [ ] **Step 2: Add to mod.rs**

Add `pub mod domain;` to `forwardemail-lib/src/models/mod.rs`.

- [ ] **Step 3: Run tests**

Run: `cargo test -p forwardemail-lib -- models::domain`
Expected: 2 tests pass

- [ ] **Step 4: Commit**

```bash
git add forwardemail-lib/src/models/
git commit -m "feat: add domain model with row type and serde tests"
```

### Task 7: Alias Model

**Files:**
- Create: `forwardemail-lib/src/models/alias.rs`
- Modify: `forwardemail-lib/src/models/mod.rs`

- [ ] **Step 1: Write alias model with tests**

```rust
// forwardemail-lib/src/models/alias.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub recipients: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    #[serde(default)]
    pub is_enabled: Option<bool>,
    #[serde(default)]
    pub has_recipient_verification: Option<bool>,
    #[serde(default)]
    pub has_imap: Option<bool>,
    #[serde(default)]
    pub has_pgp: Option<bool>,
    #[serde(default)]
    pub error_code_if_disabled: Option<u32>,
    #[serde(default)]
    pub vacation_responder_is_enabled: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct AliasRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "RECIPIENTS")]
    pub recipients: String,
    #[tabled(rename = "ENABLED")]
    pub enabled: String,
    #[tabled(rename = "IMAP")]
    pub imap: String,
}

impl From<&Alias> for AliasRow {
    fn from(a: &Alias) -> Self {
        AliasRow {
            id: a.id.clone(),
            name: a.name.clone().unwrap_or("-".to_string()),
            recipients: a
                .recipients
                .as_ref()
                .map(|r| r.join(", "))
                .unwrap_or("-".to_string()),
            enabled: a
                .is_enabled
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
            imap: a
                .has_imap
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedPassword {
    #[serde(default)]
    pub password: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_alias() {
        let json = r#"{
            "id": "alias123",
            "name": "info",
            "recipients": ["user@gmail.com", "other@gmail.com"],
            "is_enabled": true,
            "has_imap": false,
            "has_pgp": false,
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let alias: Alias = serde_json::from_str(json).unwrap();
        assert_eq!(alias.id, "alias123");
        assert_eq!(alias.name, Some("info".to_string()));
        assert_eq!(alias.recipients.unwrap().len(), 2);
    }

    #[test]
    fn test_alias_row_formatting() {
        let alias = Alias {
            id: "a1".to_string(),
            name: Some("info".to_string()),
            recipients: Some(vec!["a@b.com".to_string(), "c@d.com".to_string()]),
            description: None,
            domain: None,
            labels: None,
            is_enabled: Some(true),
            has_recipient_verification: None,
            has_imap: Some(true),
            has_pgp: None,
            error_code_if_disabled: None,
            vacation_responder_is_enabled: None,
            created_at: None,
            updated_at: None,
        };
        let row = AliasRow::from(&alias);
        assert_eq!(row.recipients, "a@b.com, c@d.com");
        assert_eq!(row.enabled, "yes");
        assert_eq!(row.imap, "yes");
    }

    #[test]
    fn test_deserialize_minimal_alias() {
        let json = r#"{"id": "a1"}"#;
        let alias: Alias = serde_json::from_str(json).unwrap();
        assert_eq!(alias.name, None);
        let row = AliasRow::from(&alias);
        assert_eq!(row.name, "-");
    }
}
```

- [ ] **Step 2: Add to mod.rs**

Add `pub mod alias;` to `forwardemail-lib/src/models/mod.rs`.

- [ ] **Step 3: Run tests**

Run: `cargo test -p forwardemail-lib -- models::alias`
Expected: 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add forwardemail-lib/src/models/
git commit -m "feat: add alias model with row type and serde tests"
```

### Task 8: Email Model

**Files:**
- Create: `forwardemail-lib/src/models/email.rs`
- Modify: `forwardemail-lib/src/models/mod.rs`

- [ ] **Step 1: Write email model with tests**

```rust
// forwardemail-lib/src/models/email.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<Vec<String>>,
    #[serde(default)]
    pub cc: Option<Vec<String>>,
    #[serde(default)]
    pub bcc: Option<Vec<String>>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct EmailRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "FROM")]
    pub from: String,
    #[tabled(rename = "TO")]
    pub to: String,
    #[tabled(rename = "SUBJECT")]
    pub subject: String,
}

impl From<&Email> for EmailRow {
    fn from(e: &Email) -> Self {
        EmailRow {
            id: e.id.clone(),
            status: e.status.clone().unwrap_or("-".to_string()),
            from: e.from.clone().unwrap_or("-".to_string()),
            to: e
                .to
                .as_ref()
                .map(|t| t.join(", "))
                .unwrap_or("-".to_string()),
            subject: e.subject.clone().unwrap_or("-".to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailLimit {
    pub count: u32,
    pub limit: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_email() {
        let json = r#"{
            "id": "email123",
            "status": "queued",
            "from": "sender@example.com",
            "to": ["recipient@example.com"],
            "subject": "Hello",
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let email: Email = serde_json::from_str(json).unwrap();
        assert_eq!(email.id, "email123");
        assert_eq!(email.status, Some("queued".to_string()));
        assert_eq!(email.to.unwrap().len(), 1);
    }

    #[test]
    fn test_deserialize_email_limit() {
        let json = r#"{"count": 5, "limit": 300}"#;
        let limit: EmailLimit = serde_json::from_str(json).unwrap();
        assert_eq!(limit.count, 5);
        assert_eq!(limit.limit, 300);
    }

    #[test]
    fn test_email_row() {
        let email = Email {
            id: "e1".to_string(),
            status: Some("sent".to_string()),
            from: Some("a@b.com".to_string()),
            to: Some(vec!["c@d.com".to_string()]),
            cc: None,
            bcc: None,
            subject: Some("Test".to_string()),
            created_at: None,
            updated_at: None,
        };
        let row = EmailRow::from(&email);
        assert_eq!(row.status, "sent");
        assert_eq!(row.to, "c@d.com");
    }
}
```

- [ ] **Step 2: Add to mod.rs**

Add `pub mod email;` to `forwardemail-lib/src/models/mod.rs`.

- [ ] **Step 3: Run tests**

Run: `cargo test -p forwardemail-lib -- models::email`
Expected: 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add forwardemail-lib/src/models/
git commit -m "feat: add email model with EmailLimit and serde tests"
```

### Task 9: Remaining Models (invite, member, catch_all_password, encrypt, log)

**Files:**
- Create: `forwardemail-lib/src/models/invite.rs`
- Create: `forwardemail-lib/src/models/member.rs`
- Create: `forwardemail-lib/src/models/catch_all_password.rs`
- Create: `forwardemail-lib/src/models/encrypt.rs`
- Create: `forwardemail-lib/src/models/log.rs`
- Modify: `forwardemail-lib/src/models/mod.rs`

- [ ] **Step 1: Write invite model**

```rust
// forwardemail-lib/src/models/invite.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Invite {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct InviteRow {
    #[tabled(rename = "EMAIL")]
    pub email: String,
    #[tabled(rename = "GROUP")]
    pub group: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&Invite> for InviteRow {
    fn from(i: &Invite) -> Self {
        InviteRow {
            email: i.email.clone().unwrap_or("-".to_string()),
            group: i.group.clone().unwrap_or("-".to_string()),
            created_at: i.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_invite() {
        let json = r#"{"email": "user@example.com", "group": "admin"}"#;
        let invite: Invite = serde_json::from_str(json).unwrap();
        assert_eq!(invite.email, Some("user@example.com".to_string()));
        assert_eq!(invite.group, Some("admin".to_string()));
    }
}
```

- [ ] **Step 2: Write member model**

```rust
// forwardemail-lib/src/models/member.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub id: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct MemberRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "EMAIL")]
    pub email: String,
    #[tabled(rename = "GROUP")]
    pub group: String,
}

impl From<&Member> for MemberRow {
    fn from(m: &Member) -> Self {
        MemberRow {
            id: m.id.clone(),
            email: m.email.clone().unwrap_or("-".to_string()),
            group: m.group.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_member() {
        let json = r#"{"id": "m1", "email": "user@example.com", "group": "user"}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id, "m1");
        assert_eq!(member.group, Some("user".to_string()));
    }
}
```

- [ ] **Step 3: Write catch_all_password model**

```rust
// forwardemail-lib/src/models/catch_all_password.rs
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct CatchAllPassword {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct CatchAllPasswordRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "DESCRIPTION")]
    pub description: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&CatchAllPassword> for CatchAllPasswordRow {
    fn from(c: &CatchAllPassword) -> Self {
        CatchAllPasswordRow {
            id: c.id.clone().unwrap_or("-".to_string()),
            description: c.description.clone().unwrap_or("-".to_string()),
            created_at: c.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_catch_all_password() {
        let json = r#"{"id": "cap1", "description": "Main password"}"#;
        let cap: CatchAllPassword = serde_json::from_str(json).unwrap();
        assert_eq!(cap.id, Some("cap1".to_string()));
    }
}
```

- [ ] **Step 4: Write encrypt model**

```rust
// forwardemail-lib/src/models/encrypt.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct EncryptRequest {
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptResponse {
    #[serde(default)]
    pub encrypted: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_encrypt_response() {
        let json = r#"{"encrypted": "v=spf1 a mx include:spf.forwardemail.net"}"#;
        let resp: EncryptResponse = serde_json::from_str(json).unwrap();
        assert!(resp.encrypted.is_some());
    }
}
```

- [ ] **Step 5: Write log model (minimal — response is CSV bytes)**

```rust
// forwardemail-lib/src/models/log.rs
// Log downloads return gzipped CSV, not JSON.
// No model struct needed — the binary crate handles decompression and output.
// This module exists for consistency and future expansion.
```

- [ ] **Step 6: Update mod.rs**

```rust
// forwardemail-lib/src/models/mod.rs
pub mod account;
pub mod alias;
pub mod catch_all_password;
pub mod domain;
pub mod email;
pub mod encrypt;
pub mod invite;
pub mod log;
pub mod member;
```

- [ ] **Step 7: Run all model tests**

Run: `cargo test -p forwardemail-lib -- models`
Expected: all tests pass (account: 3, domain: 2, alias: 3, email: 3, invite: 1, member: 1, catch_all_password: 1, encrypt: 1 = 15 total)

- [ ] **Step 8: Commit**

```bash
git add forwardemail-lib/src/models/
git commit -m "feat: add invite, member, catch-all password, encrypt, and log models"
```

---

## Chunk 3: CLI Commands Part 1

### Task 10: Main CLI Structure

**Files:**
- Modify: `forwardemail/src/main.rs`
- Create: `forwardemail/src/cmd/mod.rs`

- [ ] **Step 1: Write full main.rs with clap CLI and dispatch**

```rust
// forwardemail/src/main.rs
mod cmd;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "forwardemail", about = "CLI and TUI for the Forward Email API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force JSON output
    #[arg(long, global = true)]
    pub json: bool,

    /// API key (overrides config file and env var)
    #[arg(long, global = true)]
    pub api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage account
    Account {
        #[command(subcommand)]
        action: cmd::account::AccountAction,
    },
    /// Manage domains
    Domains {
        #[command(subcommand)]
        action: cmd::domains::DomainsAction,
    },
    /// Manage aliases
    Aliases {
        #[command(subcommand)]
        action: cmd::aliases::AliasesAction,
    },
    /// Manage outbound emails
    Emails {
        #[command(subcommand)]
        action: cmd::emails::EmailsAction,
    },
    /// Download logs
    Logs {
        #[command(subcommand)]
        action: cmd::logs::LogsAction,
    },
    /// Manage domain invites
    Invites {
        #[command(subcommand)]
        action: cmd::invites::InvitesAction,
    },
    /// Manage domain members
    Members {
        #[command(subcommand)]
        action: cmd::members::MembersAction,
    },
    /// Manage catch-all passwords
    #[command(name = "catch-all-passwords")]
    CatchAllPasswords {
        #[command(subcommand)]
        action: cmd::catch_all_passwords::CatchAllPasswordsAction,
    },
    /// Encrypt TXT records
    Encrypt {
        /// Plaintext TXT record to encrypt
        input: String,
    },
    /// Launch interactive TUI
    Tui,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {:#}", e);

        let exit_code = if format!("{:#}", e).contains("No API key found")
            || format!("{:#}", e).contains("Failed to parse config")
            || format!("{:#}", e).contains("HOME environment variable")
        {
            2
        } else {
            1
        };
        process::exit(exit_code);
    }
}

fn run(cli: Cli) -> Result<()> {
    let mode = output::OutputMode::from_json_flag(cli.json);

    match cli.command {
        Commands::Encrypt { input } => cmd::encrypt::run(&input, mode),
        Commands::Tui => {
            eprintln!("TUI not yet implemented");
            Ok(())
        }
        command => {
            // All other commands require authentication
            let config =
                forwardemail_lib::config::Config::load(cli.api_key.as_deref())?;
            let client =
                forwardemail_lib::client::Client::new(config.api_key, config.base_url)?;

            match command {
                Commands::Account { action } => cmd::account::run(action, &client, mode),
                Commands::Domains { action } => cmd::domains::run(action, &client, mode),
                Commands::Aliases { action } => cmd::aliases::run(action, &client, mode),
                Commands::Emails { action } => cmd::emails::run(action, &client, mode),
                Commands::Logs { action } => cmd::logs::run(action, &client, mode),
                Commands::Invites { action } => cmd::invites::run(action, &client, mode),
                Commands::Members { action } => cmd::members::run(action, &client, mode),
                Commands::CatchAllPasswords { action } => {
                    cmd::catch_all_passwords::run(action, &client, mode)
                }
                Commands::Encrypt { .. } | Commands::Tui => unreachable!(),
            }
        }
    }
}
```

- [ ] **Step 2: Create cmd/mod.rs with all module declarations**

```rust
// forwardemail/src/cmd/mod.rs
pub mod account;
pub mod aliases;
pub mod catch_all_passwords;
pub mod domains;
pub mod emails;
pub mod encrypt;
pub mod invites;
pub mod logs;
pub mod members;
```

- [ ] **Step 3: Create stub files for all command modules**

Create each of these files with minimal compilable content. Each file follows this pattern (using account as the example):

```rust
// forwardemail/src/cmd/account.rs
use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use crate::output::OutputMode;

#[derive(Subcommand)]
pub enum AccountAction {
    /// Create a new account
    Create {
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    /// Get account details
    Get,
    /// Update account
    Update {
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        given_name: Option<String>,
        #[arg(long)]
        family_name: Option<String>,
        #[arg(long)]
        avatar_url: Option<String>,
    },
}

pub fn run(action: AccountAction, client: &Client, mode: OutputMode) -> Result<()> {
    let _ = (action, client, mode);
    todo!()
}
```

Create stubs for: `domains.rs`, `aliases.rs`, `emails.rs`, `logs.rs`, `invites.rs`, `members.rs`, `catch_all_passwords.rs`, `encrypt.rs`. Each needs the correct `Action` enum matching the spec's CLI reference. See the spec at `docs/superpowers/specs/2026-03-16-forwardemail-cli-design.md` for exact subcommands and arguments per resource.

Key differences from account:
- `encrypt.rs`: Takes `input: &str` and `OutputMode`, no `Client` — uses unauthenticated client internally.
- `domains.rs`: Actions include `List`, `Create`, `Get`, `Update`, `Delete`, `VerifyRecords`, `VerifySMTP`. List/create/update have the full set of flags from the spec.
- `aliases.rs`: All actions take `domain: String` as first positional arg. Includes `GeneratePassword`.
- `emails.rs`: `Send` has from/to/cc/bcc/subject/text/html/reply-to/priority flags. `Limit` has no args.
- `logs.rs`: `Download` with `--domain`, `-q`, `--bounce-category`, `--response-code`.
- `invites.rs`: Actions take `domain: String`. Create needs `--email` and `--group`. Remove needs `--email`.
- `members.rs`: Actions take `domain: String` and `member_id: String`. Update needs `--group`.
- `catch_all_passwords.rs`: Actions take `domain: String`. Delete takes `token_id: String`.

- [ ] **Step 4: Verify everything compiles**

Run: `cargo check --workspace`
Expected: compiles (with `todo!()` warnings)

- [ ] **Step 5: Commit**

```bash
git add forwardemail/src/
git commit -m "feat: add CLI structure with clap subcommands and lazy auth dispatch"
```

### Task 11: Account Commands

**Files:**
- Modify: `forwardemail/src/cmd/account.rs`

Reference: `/home/alindsay/projects/orangerabbit-io/updown-io/src/cmd/checks.rs` for the dispatch pattern.

- [ ] **Step 1: Implement account commands**

Replace the `todo!()` in `account.rs` with full implementations:

```rust
pub fn run(action: AccountAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        AccountAction::Create { email, password } => {
            let mut body = std::collections::HashMap::new();
            body.insert("email".to_string(), serde_json::json!(email));
            body.insert("password".to_string(), serde_json::json!(password));
            let resp = client.post("/v1/account", &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let account: Account = resp.json()?;
                    output::print_confirm(&format!(
                        "Account created: {}",
                        account.email.unwrap_or_default()
                    ));
                }
            }
            Ok(())
        }
        AccountAction::Get => {
            let resp = client.get("/v1/account")?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let account: Account = resp.json()?;
                    let pairs = vec![
                        ("Email", account.email.clone().unwrap_or("-".to_string())),
                        ("Name", {
                            match (&account.given_name, &account.family_name) {
                                (Some(g), Some(f)) => format!("{} {}", g, f),
                                (Some(g), None) => g.clone(),
                                (None, Some(f)) => f.clone(),
                                (None, None) => "-".to_string(),
                            }
                        }),
                        ("Plan", account.plan.clone().unwrap_or("-".to_string())),
                        (
                            "Created",
                            account.created_at.clone().unwrap_or("-".to_string()),
                        ),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
        AccountAction::Update {
            email,
            given_name,
            family_name,
            avatar_url,
        } => {
            let mut body = std::collections::HashMap::new();
            if let Some(e) = email {
                body.insert("email".to_string(), serde_json::json!(e));
            }
            if let Some(g) = given_name {
                body.insert("given_name".to_string(), serde_json::json!(g));
            }
            if let Some(f) = family_name {
                body.insert("family_name".to_string(), serde_json::json!(f));
            }
            if let Some(a) = avatar_url {
                body.insert("avatar_url".to_string(), serde_json::json!(a));
            }
            let resp = client.put("/v1/account", &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    output::print_confirm("Account updated");
                }
            }
            Ok(())
        }
    }
}
```

Add necessary imports at top: `use forwardemail_lib::models::account::Account;` and `use crate::output;`.

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p forwardemail`
Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add forwardemail/src/cmd/account.rs
git commit -m "feat: implement account commands (create, get, update)"
```

### Task 12: Domain Commands

**Files:**
- Modify: `forwardemail/src/cmd/domains.rs`

- [ ] **Step 1: Implement domain commands**

Full implementation following the account command pattern. The `DomainsAction` enum definition:

```rust
#[derive(Subcommand)]
pub enum DomainsAction {
    /// List all domains
    List {
        #[arg(short, long)]
        q: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        sort: Option<String>,
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Create a new domain
    Create {
        /// Domain name (FQDN)
        domain: String,
        #[arg(long, value_parser = ["free", "enhanced_protection", "team"])]
        plan: Option<String>,
        #[arg(long)]
        team_domain: Option<String>,
        #[arg(long)]
        catchall: Option<String>,
        #[arg(long)]
        has_adult_content_protection: Option<bool>,
        #[arg(long)]
        has_phishing_protection: Option<bool>,
        #[arg(long)]
        has_executable_protection: Option<bool>,
        #[arg(long)]
        has_virus_protection: Option<bool>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        ignore_mx_check: Option<bool>,
        #[arg(long)]
        retention_days: Option<u32>,
        #[arg(long)]
        bounce_webhook: Option<String>,
        #[arg(long)]
        max_quota_per_alias: Option<String>,
    },
    /// Get domain details
    Get { domain: String },
    /// Update domain settings
    Update {
        domain: String,
        #[arg(long)]
        smtp_port: Option<String>,
        #[arg(long)]
        has_adult_content_protection: Option<bool>,
        #[arg(long)]
        has_phishing_protection: Option<bool>,
        #[arg(long)]
        has_executable_protection: Option<bool>,
        #[arg(long)]
        has_virus_protection: Option<bool>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        ignore_mx_check: Option<bool>,
        #[arg(long)]
        retention_days: Option<u32>,
        #[arg(long)]
        bounce_webhook: Option<String>,
        #[arg(long)]
        max_quota_per_alias: Option<String>,
    },
    /// Delete a domain
    Delete { domain: String },
    /// Verify DNS records
    #[command(name = "verify-records")]
    VerifyRecords { domain: String },
    /// Verify SMTP
    #[command(name = "verify-smtp")]
    VerifySmtp { domain: String },
}
```

The `run` function dispatches to handlers. The list handler builds query params, calls `client.get_with_params("/v1/domains", &params)`, and parses pagination headers from the response. Use this helper to extract pagination from response headers:

```rust
fn parse_pagination(resp: &reqwest::blocking::Response) -> Option<(u32, u32, u32)> {
    let current = resp.headers().get("X-Page-Current")?.to_str().ok()?.parse().ok()?;
    let total_pages = resp.headers().get("X-Page-Count")?.to_str().ok()?.parse().ok()?;
    let total_items = resp.headers().get("X-Item-Count")?.to_str().ok()?.parse().ok()?;
    Some((current, total_pages, total_items))
}
```

Call `parse_pagination` before consuming the response body (since `.json()` consumes it). Extract headers first, then deserialize. Pattern for list handlers:

```rust
let resp = client.get_with_params("/v1/domains", &params)?;
let pagination = parse_pagination(&resp);
// then consume body
match mode {
    OutputMode::Json => { let json: serde_json::Value = resp.json()?; output::print_json(&json); }
    OutputMode::Table => {
        let domains: Vec<Domain> = resp.json()?;
        let rows: Vec<DomainRow> = domains.iter().map(DomainRow::from).collect();
        output::print_table(&rows);
        if let Some((current, total, items)) = pagination {
            output::print_pagination(current, total, items);
        }
    }
}
```

Create/update handlers build `HashMap<String, Value>` from provided flags. Get handler uses `print_kv`. Delete uses `print_confirm`. VerifyRecords/VerifySmtp call GET on `/v1/domains/{domain}/verify-records` and `/v1/domains/{domain}/verify-smtp` respectively and display the result as KV or JSON.

The `DomainsAction` enum should have:

- `List` with `q`, `name`, `sort`, `page`, `limit` options
- `Create` with `domain` positional + all optional flags (plan, team_domain, catchall, retention_days, protections, etc.)
- `Get` with `domain` positional
- `Update` with `domain` positional + all optional flags
- `Delete` with `domain` positional
- `VerifyRecords` with `domain` positional
- `VerifySmtp` with `domain` positional

List handler builds query params from provided flags and passes to `client.get_with_params("/v1/domains", &params)`. Create/update handlers build a `HashMap<String, Value>` body from provided flags.

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p forwardemail`
Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add forwardemail/src/cmd/domains.rs
git commit -m "feat: implement domain commands (list, CRUD, verify)"
```

### Task 13: Alias Commands

**Files:**
- Modify: `forwardemail/src/cmd/aliases.rs`

- [ ] **Step 1: Implement alias commands**

Full implementation following the domain command pattern. The `AliasesAction` enum definition:

```rust
#[derive(Subcommand)]
pub enum AliasesAction {
    /// List aliases for a domain
    List {
        domain: String,
        #[arg(short, long)]
        q: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        recipient: Option<String>,
        #[arg(long)]
        sort: Option<String>,
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Create an alias
    Create {
        domain: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_delimiter = ',')]
        recipients: Option<Vec<String>>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_delimiter = ',')]
        labels: Option<Vec<String>>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        is_enabled: Option<bool>,
        #[arg(long, value_parser = ["250", "421", "550"])]
        error_code_if_disabled: Option<String>,
        #[arg(long)]
        has_imap: Option<bool>,
        #[arg(long)]
        has_pgp: Option<bool>,
        #[arg(long)]
        public_key: Option<String>,
        #[arg(long)]
        max_quota: Option<String>,
        #[arg(long)]
        vacation_responder_is_enabled: Option<bool>,
        #[arg(long)]
        vacation_responder_start_date: Option<String>,
        #[arg(long)]
        vacation_responder_end_date: Option<String>,
        #[arg(long)]
        vacation_responder_subject: Option<String>,
        #[arg(long)]
        vacation_responder_message: Option<String>,
    },
    /// Get alias details
    Get {
        domain: String,
        alias_id: String,
    },
    /// Update an alias
    Update {
        domain: String,
        alias_id: String,
        // Same optional fields as Create
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_delimiter = ',')]
        recipients: Option<Vec<String>>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_delimiter = ',')]
        labels: Option<Vec<String>>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        is_enabled: Option<bool>,
        #[arg(long, value_parser = ["250", "421", "550"])]
        error_code_if_disabled: Option<String>,
        #[arg(long)]
        has_imap: Option<bool>,
        #[arg(long)]
        has_pgp: Option<bool>,
        #[arg(long)]
        public_key: Option<String>,
        #[arg(long)]
        max_quota: Option<String>,
        #[arg(long)]
        vacation_responder_is_enabled: Option<bool>,
        #[arg(long)]
        vacation_responder_start_date: Option<String>,
        #[arg(long)]
        vacation_responder_end_date: Option<String>,
        #[arg(long)]
        vacation_responder_subject: Option<String>,
        #[arg(long)]
        vacation_responder_message: Option<String>,
    },
    /// Delete an alias
    Delete {
        domain: String,
        alias_id: String,
    },
    /// Generate alias password
    #[command(name = "generate-password")]
    GeneratePassword {
        domain: String,
        alias_id: String,
        #[arg(long)]
        new_password: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        is_override: Option<bool>,
        #[arg(long)]
        emailed_instructions: Option<String>,
    },
}
```

API paths: `/v1/domains/{domain}/aliases[/{alias_id}]` and `/v1/domains/{domain}/aliases/{alias_id}/generate-password`. Follow the same dispatch/handler pattern as domains. List handler includes pagination header parsing. Create/update build `HashMap` body from flags. Use a shared `AliasBodyParams` struct (like updown-io's `CheckBodyParams`) to DRY up the create/update body building.

The `AliasesAction` enum should have:

- `List` with `domain` positional + `q`, `name`, `recipient`, `sort`, `page`, `limit`
- `Create` with `domain` positional + all optional flags (name, recipients, description, labels, protections, imap, pgp, vacation responder fields, etc.)
- `Get` with `domain` + `alias_id` positionals
- `Update` with `domain` + `alias_id` positionals + same optional flags as create
- `Delete` with `domain` + `alias_id` positionals
- `GeneratePassword` with `domain` + `alias_id` positionals + `new_password`, `password`, `is_override`, `emailed_instructions` options

API paths: `/v1/domains/{domain}/aliases[/{alias_id}]` and `/v1/domains/{domain}/aliases/{alias_id}/generate-password`.

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p forwardemail`
Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add forwardemail/src/cmd/aliases.rs
git commit -m "feat: implement alias commands (list, CRUD, generate-password)"
```

---

## Chunk 4: CLI Commands Part 2

### Task 14: Email Commands

**Files:**
- Modify: `forwardemail/src/cmd/emails.rs`

- [ ] **Step 1: Implement email commands**

`EmailsAction` enum:
- `List` with `q`, `domain`, `sort`, `page`, `limit`
- `Send` with `from`, `to`, `cc`, `bcc`, `subject`, `text`, `html`, `reply_to`, `priority`
- `Get` with `id` positional
- `Delete` with `id` positional
- `Limit` (no args)

API paths: `/v1/emails[/{id}]`, `/v1/emails/limit`.

For `Limit`, display as KV: `"Sent today: 5"`, `"Daily limit: 300"`.

- [ ] **Step 2: Verify compilation and commit**

```bash
cargo check -p forwardemail && git add forwardemail/src/cmd/emails.rs && git commit -m "feat: implement email commands (list, send, get, delete, limit)"
```

### Task 15: Log Commands

**Files:**
- Modify: `forwardemail/src/cmd/logs.rs`

- [ ] **Step 1: Implement log download with gzip decompression**

`LogsAction` enum:
- `Download` with `domain`, `q`, `bounce_category`, `response_code` options

The handler calls `client.get_bytes("/v1/logs/download", &params)`, decompresses with `flate2::read::GzDecoder`, and writes CSV to stdout via `output::print_raw`. If the response is not gzipped (no gzip magic bytes `0x1f 0x8b`), write raw bytes directly.

```rust
use flate2::read::GzDecoder;
use std::io::Read;

// In the download handler:
let bytes = client.get_bytes("/v1/logs/download", &params)?;
let text = if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    decoder.read_to_string(&mut s).context("Failed to decompress gzipped log data")?;
    s
} else {
    String::from_utf8(bytes).context("Log response is not valid UTF-8")?
};
output::print_raw(&text);
```

- [ ] **Step 2: Verify compilation and commit**

```bash
cargo check -p forwardemail && git add forwardemail/src/cmd/logs.rs && git commit -m "feat: implement log download with gzip decompression"
```

### Task 16: Invite Commands

**Files:**
- Modify: `forwardemail/src/cmd/invites.rs`

- [ ] **Step 1: Implement invite commands**

`InvitesAction` enum:
- `Create` with `domain` positional, `--email`, `--group admin|user`
- `Accept` with `domain` positional
- `Remove` with `domain` positional, `--email`

API paths:
- `POST /v1/domains/{domain}/invites`
- `GET /v1/domains/{domain}/invites` (accept)
- `DELETE /v1/domains/{domain}/invites` (remove — uses `delete_with_body` since email is in body)

- [ ] **Step 2: Verify compilation and commit**

```bash
cargo check -p forwardemail && git add forwardemail/src/cmd/invites.rs && git commit -m "feat: implement invite commands (create, accept, remove)"
```

### Task 17: Member Commands

**Files:**
- Modify: `forwardemail/src/cmd/members.rs`

- [ ] **Step 1: Implement member commands**

`MembersAction` enum:
- `Update` with `domain` + `member_id` positionals, `--group admin|user`
- `Remove` with `domain` + `member_id` positionals

API paths: `/v1/domains/{domain}/members/{member_id}` (PUT, DELETE).

- [ ] **Step 2: Verify compilation and commit**

```bash
cargo check -p forwardemail && git add forwardemail/src/cmd/members.rs && git commit -m "feat: implement member commands (update, remove)"
```

### Task 18: Catch-All Password Commands

**Files:**
- Modify: `forwardemail/src/cmd/catch_all_passwords.rs`

- [ ] **Step 1: Implement catch-all password commands**

`CatchAllPasswordsAction` enum:
- `List` with `domain` positional
- `Create` with `domain` positional, `--password`, `--description`
- `Delete` with `domain` + `token_id` positionals

API paths: `/v1/domains/{domain}/catch-all-passwords[/{token_id}]`.

- [ ] **Step 2: Verify compilation and commit**

```bash
cargo check -p forwardemail && git add forwardemail/src/cmd/catch_all_passwords.rs && git commit -m "feat: implement catch-all password commands (list, create, delete)"
```

### Task 19: Encrypt Command

**Files:**
- Modify: `forwardemail/src/cmd/encrypt.rs`

- [ ] **Step 1: Implement encrypt command (unauthenticated)**

```rust
// forwardemail/src/cmd/encrypt.rs
use anyhow::Result;
use crate::output::{self, OutputMode};

pub fn run(input: &str, mode: OutputMode) -> Result<()> {
    let base_url = forwardemail_lib::config::Config::base_url_only();
    let client = forwardemail_lib::client::Client::unauthenticated(base_url)?;

    let mut body = std::collections::HashMap::new();
    body.insert("input".to_string(), serde_json::json!(input));

    let resp = client.post("/v1/encrypt", &body)?;

    match mode {
        OutputMode::Json => {
            let json: serde_json::Value = resp.json()?;
            output::print_json(&json);
        }
        OutputMode::Table => {
            let result: forwardemail_lib::models::encrypt::EncryptResponse = resp.json()?;
            if let Some(encrypted) = result.encrypted {
                println!("{}", encrypted);
            }
        }
    }

    Ok(())
}
```

- [ ] **Step 2: Verify compilation and commit**

```bash
cargo check -p forwardemail && git add forwardemail/src/cmd/encrypt.rs && git commit -m "feat: implement encrypt command with unauthenticated client"
```

---

## Chunk 5: Integration Tests

### Task 20: Test Infrastructure

**Files:**
- Create: `forwardemail/tests/common/mod.rs`
- Create: `forwardemail/tests/fixtures/` (directory)

- [ ] **Step 1: Create test helpers**

```rust
// forwardemail/tests/common/mod.rs
use std::path::PathBuf;

pub fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Missing fixture: {}", path.display()))
}

pub fn binary() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("forwardemail").unwrap()
}
```

- [ ] **Step 2: Create JSON fixtures**

Create these fixture files in `forwardemail/tests/fixtures/`:

`account_get.json`:
```json
{
  "email": "user@example.com",
  "given_name": "John",
  "family_name": "Doe",
  "plan": "enhanced_protection",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-06-01T00:00:00Z"
}
```

`domains_list.json`:
```json
[
  {
    "name": "example.com",
    "plan": "enhanced_protection",
    "has_mx_record": true,
    "has_txt_record": true,
    "created_at": "2024-01-01T00:00:00Z"
  },
  {
    "name": "test.org",
    "plan": "free",
    "has_mx_record": false,
    "has_txt_record": false,
    "created_at": "2024-02-01T00:00:00Z"
  }
]
```

`domain_get.json`:
```json
{
  "name": "example.com",
  "plan": "enhanced_protection",
  "smtp_port": "25",
  "has_mx_record": true,
  "has_txt_record": true,
  "has_adult_content_protection": true,
  "has_phishing_protection": true,
  "has_executable_protection": true,
  "has_virus_protection": true,
  "retention_days": 30,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-06-01T00:00:00Z"
}
```

`aliases_list.json`:
```json
[
  {
    "id": "alias1",
    "name": "info",
    "recipients": ["user@gmail.com"],
    "is_enabled": true,
    "has_imap": true,
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

`alias_get.json`:
```json
{
  "id": "alias1",
  "name": "info",
  "recipients": ["user@gmail.com", "other@gmail.com"],
  "is_enabled": true,
  "has_imap": true,
  "has_pgp": false,
  "description": "Info alias",
  "created_at": "2024-01-01T00:00:00Z"
}
```

`emails_list.json`:
```json
[
  {
    "id": "email1",
    "status": "sent",
    "from": "sender@example.com",
    "to": ["recipient@example.com"],
    "subject": "Hello World",
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

`email_limit.json`:
```json
{"count": 5, "limit": 300}
```

`invites_list.json`:
```json
[{"email": "invited@example.com", "group": "admin"}]
```

`members_list.json`:
```json
[{"id": "m1", "email": "member@example.com", "group": "user"}]
```

`catch_all_passwords_list.json`:
```json
[{"id": "cap1", "description": "Main", "created_at": "2024-01-01T00:00:00Z"}]
```

`email_get.json`:
```json
{
  "id": "email1",
  "status": "sent",
  "from": "sender@example.com",
  "to": ["recipient@example.com"],
  "subject": "Hello World",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T01:00:00Z"
}
```

`encrypt_response.json`:
```json
{"encrypted": "v=spf1 a mx include:spf.forwardemail.net ~all"}
```

- [ ] **Step 3: Commit**

```bash
git add forwardemail/tests/
git commit -m "feat: add test helpers and JSON fixtures for all resources"
```

### Task 21: Account Integration Tests

**Files:**
- Create: `forwardemail/tests/account_test.rs`

- [ ] **Step 1: Write account tests**

```rust
// forwardemail/tests/account_test.rs
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_account_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/account")
        .match_header("authorization", mockito::Matcher::Regex("Basic .+".to_string()))
        .with_body(common::fixture("account_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "account", "get"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("user@example.com"))
        .stdout(predicate::str::contains("John Doe"));

    mock.assert();
}

#[test]
fn test_account_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/account")
        .match_header("authorization", mockito::Matcher::Regex("Basic .+".to_string()))
        .with_body(common::fixture("account_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "test-key", "--json", "account", "get"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"email\""));

    mock.assert();
}

#[test]
fn test_account_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v1/account")
        .with_status(401)
        .with_body(r#"{"message": "Unauthorized"}"#)
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad-key", "account", "get"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Authentication failed"));

    mock.assert();
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p forwardemail --test account_test`
Expected: 3 tests pass

- [ ] **Step 3: Commit**

```bash
git add forwardemail/tests/account_test.rs
git commit -m "test: add account integration tests (get table/json, auth error)"
```

### Task 22: Domain Integration Tests

**Files:**
- Create: `forwardemail/tests/domains_test.rs`

- [ ] **Step 1: Write domain tests**

Tests to write:
1. `test_domains_list_table` — mock `GET /v1/domains`, verify markdown table has "example.com" and column headers
2. `test_domains_list_json` — same mock, `--json` flag, verify JSON output
3. `test_domains_get` — mock `GET /v1/domains/example.com`, verify KV output
4. `test_domains_create` — mock `POST /v1/domains`, verify with `Matcher::JsonString` for request body, verify confirmation output
5. `test_domains_delete` — mock `DELETE /v1/domains/example.com`, verify confirmation
6. `test_domains_not_found` — mock `GET /v1/domains/missing.com` with 404, verify error
7. `test_domains_list_pagination` — mock `GET /v1/domains` with pagination response headers (`X-Page-Count: 3`, `X-Page-Current: 1`, `X-Page-Size: 10`, `X-Item-Count: 25`), verify pagination footer "Page 1 of 3 (25 total items)" appears in output

- [ ] **Step 2: Run tests**

Run: `cargo test -p forwardemail --test domains_test`
Expected: 6 tests pass

- [ ] **Step 3: Commit**

```bash
git add forwardemail/tests/domains_test.rs
git commit -m "test: add domain integration tests (list, get, create, delete, 404)"
```

### Task 23: Alias Integration Tests

**Files:**
- Create: `forwardemail/tests/aliases_test.rs`

- [ ] **Step 1: Write alias tests**

Tests:
1. `test_aliases_list_table` — mock `GET /v1/domains/example.com/aliases`
2. `test_aliases_list_json` — same with `--json`
3. `test_aliases_get` — mock `GET /v1/domains/example.com/aliases/alias1`
4. `test_aliases_create` — mock `POST /v1/domains/example.com/aliases`
5. `test_aliases_delete` — mock `DELETE /v1/domains/example.com/aliases/alias1`

- [ ] **Step 2: Run tests and commit**

```bash
cargo test -p forwardemail --test aliases_test && git add forwardemail/tests/aliases_test.rs && git commit -m "test: add alias integration tests"
```

### Task 24: Email Integration Tests

**Files:**
- Create: `forwardemail/tests/emails_test.rs`

- [ ] **Step 1: Write email tests**

Tests:
1. `test_emails_list_table` — mock `GET /v1/emails`
2. `test_emails_get` — mock `GET /v1/emails/email1`
3. `test_emails_limit` — mock `GET /v1/emails/limit`, verify "Sent today: 5" and "Daily limit: 300"
4. `test_emails_delete` — mock `DELETE /v1/emails/email1`

- [ ] **Step 2: Run tests and commit**

```bash
cargo test -p forwardemail --test emails_test && git add forwardemail/tests/emails_test.rs && git commit -m "test: add email integration tests"
```

### Task 25: Remaining Integration Tests

**Files:**
- Create: `forwardemail/tests/invites_test.rs`
- Create: `forwardemail/tests/members_test.rs`
- Create: `forwardemail/tests/catch_all_passwords_test.rs`
- Create: `forwardemail/tests/encrypt_test.rs`
- Create: `forwardemail/tests/logs_test.rs`

- [ ] **Step 1: Write encrypt test (unauthenticated)**

```rust
// forwardemail/tests/encrypt_test.rs
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
    // No --api-key needed
    cmd.args(["encrypt", "forward-email=user@example.com"])
        .env("FORWARDEMAIL_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("spf.forwardemail.net"));

    mock.assert();
}
```

- [ ] **Step 2: Write invites test**

Test create (mock POST), accept (mock GET), remove (mock DELETE) for `/v1/domains/example.com/invites`.

- [ ] **Step 3: Write members test**

Test update (mock PUT) and remove (mock DELETE) for `/v1/domains/example.com/members/m1`.

- [ ] **Step 4: Write catch-all-passwords test**

Test list (mock GET), create (mock POST), delete (mock DELETE) for `/v1/domains/example.com/catch-all-passwords`.

- [ ] **Step 5: Write logs test**

Test download with a mock that returns plain CSV text (not gzipped, for simplicity). Verify CSV appears in stdout.

- [ ] **Step 6: Run all tests**

Run: `cargo test -p forwardemail`
Expected: all integration tests pass

- [ ] **Step 7: Commit**

```bash
git add forwardemail/tests/
git commit -m "test: add integration tests for encrypt, invites, members, catch-all-passwords, logs"
```

---

## Chunk 6: TUI

### Task 26: TUI App State Machine

**Files:**
- Create: `forwardemail/src/tui/mod.rs`
- Create: `forwardemail/src/tui/app.rs`

- [ ] **Step 1: Implement app state machine**

```rust
// forwardemail/src/tui/app.rs
use forwardemail_lib::client::Client;
use forwardemail_lib::models::account::Account;
use forwardemail_lib::models::alias::Alias;
use forwardemail_lib::models::domain::Domain;
use forwardemail_lib::models::email::Email;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Dashboard,
    DomainList,
    DomainDetail(usize),   // index into domains vec
    AliasList(String),      // domain name
    AliasDetail(String, usize), // domain name, index into aliases vec
    EmailList,
    EmailDetail(usize),    // index into emails vec
}

pub struct App {
    pub view: View,
    pub account: Option<Account>,
    pub domains: Vec<Domain>,
    pub aliases: Vec<Alias>,
    pub emails: Vec<Email>,
    pub selected: usize,
    pub should_quit: bool,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            view: View::Dashboard,
            account: None,
            domains: Vec::new(),
            aliases: Vec::new(),
            emails: Vec::new(),
            selected: 0,
            should_quit: false,
            error: None,
        }
    }

    pub fn load_dashboard(&mut self, client: &Client) {
        match client.get_json::<Account>("/v1/account") {
            Ok(account) => {
                self.account = Some(account);
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn load_domains(&mut self, client: &Client) {
        match client.get_json::<Vec<Domain>>("/v1/domains") {
            Ok(domains) => {
                self.domains = domains;
                self.selected = 0;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn load_aliases(&mut self, client: &Client, domain: &str) {
        let path = format!("/v1/domains/{}/aliases", domain);
        match client.get_json::<Vec<Alias>>(&path) {
            Ok(aliases) => {
                self.aliases = aliases;
                self.selected = 0;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn load_emails(&mut self, client: &Client) {
        match client.get_json::<Vec<Email>>("/v1/emails") {
            Ok(emails) => {
                self.emails = emails;
                self.selected = 0;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn navigate_forward(&mut self, client: &Client) {
        match &self.view {
            View::Dashboard => {
                self.view = View::DomainList;
                self.load_domains(client);
            }
            View::DomainList => {
                if !self.domains.is_empty() {
                    self.view = View::DomainDetail(self.selected);
                }
            }
            View::DomainDetail(idx) => {
                let domain = self.domains[*idx].name.clone();
                self.view = View::AliasList(domain.clone());
                self.load_aliases(client, &domain);
            }
            View::AliasList(domain) => {
                if !self.aliases.is_empty() {
                    self.view = View::AliasDetail(domain.clone(), self.selected);
                }
            }
            View::EmailList => {
                if !self.emails.is_empty() {
                    self.view = View::EmailDetail(self.selected);
                }
            }
            _ => {}
        }
    }

    pub fn navigate_back(&mut self) {
        self.selected = 0;
        match &self.view {
            View::Dashboard => self.should_quit = true,
            View::DomainList | View::EmailList => self.view = View::Dashboard,
            View::DomainDetail(_) => self.view = View::DomainList,
            View::AliasList(domain) => {
                // Go back to the domain detail by finding the domain index
                let idx = self.domains.iter().position(|d| d.name == *domain).unwrap_or(0);
                self.view = View::DomainDetail(idx);
            }
            View::AliasDetail(domain, _) => {
                self.view = View::AliasList(domain.clone());
            }
            View::EmailDetail(_) => self.view = View::EmailList,
        }
    }

    pub fn list_len(&self) -> usize {
        match &self.view {
            View::DomainList => self.domains.len(),
            View::AliasList(_) => self.aliases.len(),
            View::EmailList => self.emails.len(),
            _ => 0,
        }
    }

    pub fn select_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select_down(&mut self) {
        let len = self.list_len();
        if len > 0 && self.selected < len - 1 {
            self.selected += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let app = App::new();
        assert_eq!(app.view, View::Dashboard);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_navigate_back_from_dashboard_quits() {
        let mut app = App::new();
        app.navigate_back();
        assert!(app.should_quit);
    }

    #[test]
    fn test_select_up_down() {
        let mut app = App::new();
        app.view = View::DomainList;
        app.domains = vec![
            Domain { name: "a.com".to_string(), plan: None, max_recipients_per_alias: None, smtp_port: None, has_adult_content_protection: None, has_phishing_protection: None, has_executable_protection: None, has_virus_protection: None, has_recipient_verification: None, retention_days: None, has_mx_record: None, has_txt_record: None, created_at: None, updated_at: None },
            Domain { name: "b.com".to_string(), plan: None, max_recipients_per_alias: None, smtp_port: None, has_adult_content_protection: None, has_phishing_protection: None, has_executable_protection: None, has_virus_protection: None, has_recipient_verification: None, retention_days: None, has_mx_record: None, has_txt_record: None, created_at: None, updated_at: None },
        ];
        assert_eq!(app.selected, 0);
        app.select_down();
        assert_eq!(app.selected, 1);
        app.select_down(); // at end, stays at 1
        assert_eq!(app.selected, 1);
        app.select_up();
        assert_eq!(app.selected, 0);
        app.select_up(); // at start, stays at 0
        assert_eq!(app.selected, 0);
    }
}
```

- [ ] **Step 2: Create TUI module entry point**

```rust
// forwardemail/src/tui/mod.rs
pub mod app;
pub mod ui;
pub mod views;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::prelude::*;
use std::io;

use forwardemail_lib::client::Client;
use app::App;

pub fn run(client: &Client) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.load_dashboard(client);

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.should_quit = true;
                }
                KeyCode::Char('q') => app.should_quit = true,
                KeyCode::Esc | KeyCode::Backspace => app.navigate_back(),
                KeyCode::Enter | KeyCode::Char('l') => app.navigate_forward(client),
                KeyCode::Up | KeyCode::Char('k') => app.select_up(),
                KeyCode::Down | KeyCode::Char('j') => app.select_down(),
                KeyCode::Char('d') => {
                    app.view = app::View::DomainList;
                    app.load_domains(client);
                }
                KeyCode::Char('e') => {
                    app.view = app::View::EmailList;
                    app.load_emails(client);
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
```

- [ ] **Step 3: Run state machine tests**

Run: `cargo test -p forwardemail -- tui::app`
Expected: 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add forwardemail/src/tui/
git commit -m "feat: add TUI app state machine with navigation and event loop"
```

### Task 27: TUI Views and Rendering

**Files:**
- Create: `forwardemail/src/tui/ui.rs`
- Create: `forwardemail/src/tui/views/mod.rs`
- Create: `forwardemail/src/tui/views/dashboard.rs`
- Create: `forwardemail/src/tui/views/domains.rs`
- Create: `forwardemail/src/tui/views/aliases.rs`
- Create: `forwardemail/src/tui/views/emails.rs`

- [ ] **Step 1: Implement main draw function**

```rust
// forwardemail/src/tui/ui.rs
use ratatui::prelude::*;
use ratatui::widgets::*;
use super::app::{App, View};
use super::views;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // header
            Constraint::Min(0),     // content
            Constraint::Length(1),  // footer
        ])
        .split(f.area());

    // Header with breadcrumb
    let breadcrumb = match &app.view {
        View::Dashboard => "Forward Email > Dashboard".to_string(),
        View::DomainList => "Forward Email > Domains".to_string(),
        View::DomainDetail(idx) => format!("Forward Email > Domains > {}", app.domains.get(*idx).map(|d| d.name.as_str()).unwrap_or("?")),
        View::AliasList(domain) => format!("Forward Email > Domains > {} > Aliases", domain),
        View::AliasDetail(domain, _) => format!("Forward Email > Domains > {} > Aliases > Detail", domain),
        View::EmailList => "Forward Email > Emails".to_string(),
        View::EmailDetail(_) => "Forward Email > Emails > Detail".to_string(),
    };
    let header = Paragraph::new(breadcrumb).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Content
    if let Some(err) = &app.error {
        let error = Paragraph::new(format!("Error: {}", err)).style(Style::default().fg(Color::Red));
        f.render_widget(error, chunks[1]);
    } else {
        match &app.view {
            View::Dashboard => views::dashboard::draw(f, app, chunks[1]),
            View::DomainList => views::domains::draw_list(f, app, chunks[1]),
            View::DomainDetail(idx) => views::domains::draw_detail(f, app, *idx, chunks[1]),
            View::AliasList(_) => views::aliases::draw_list(f, app, chunks[1]),
            View::AliasDetail(_, idx) => views::aliases::draw_detail(f, app, *idx, chunks[1]),
            View::EmailList => views::emails::draw_list(f, app, chunks[1]),
            View::EmailDetail(idx) => views::emails::draw_detail(f, app, *idx, chunks[1]),
        }
    }

    // Footer with keybindings
    let footer_text = "q: quit | Esc: back | Enter: select | j/k: navigate | d: domains | e: emails";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
```

- [ ] **Step 2: Implement view modules**

Each view module renders its resource data into the content area using ratatui widgets (Table, Paragraph, List). The `draw_list` functions render a selectable table; the `draw_detail` functions render KV pairs.

Create `forwardemail/src/tui/views/mod.rs`:
```rust
pub mod dashboard;
pub mod domains;
pub mod aliases;
pub mod emails;
```

Create `views/dashboard.rs` — renders account info (email, name, plan, created) as a paragraph block.

Create `views/domains.rs` — `draw_list` renders a ratatui `Table` widget with domain rows, highlighting `app.selected`. `draw_detail` renders KV pairs for a single domain.

Create `views/aliases.rs` — same pattern, table of aliases for the current domain.

Create `views/emails.rs` — same pattern, table of emails.

All view implementations use `ratatui::widgets::Table` with `Row` items constructed from the model vectors in `App`.

- [ ] **Step 3: Wire TUI into main.rs**

Update the `Commands::Tui` arm in `main.rs`:

```rust
Commands::Tui => {
    let config = forwardemail_lib::config::Config::load(cli.api_key.as_deref())?;
    let client = forwardemail_lib::client::Client::new(config.api_key, config.base_url)?;
    crate::tui::run(&client)
}
```

Add `mod tui;` to main.rs.

Move the `Tui` match arm out of the "requires auth" block into its own arm (it needs auth but not `mode`).

- [ ] **Step 4: Verify compilation**

Run: `cargo check -p forwardemail`
Expected: compiles

- [ ] **Step 5: Commit**

```bash
git add forwardemail/src/tui/ forwardemail/src/main.rs
git commit -m "feat: add TUI views with dashboard, domain/alias/email browsing"
```

---

## Chunk 7: Build Config and Finishing

### Task 28: Nix Flake

**Files:**
- Create: `flake.nix` (workspace root)

Reference: `/home/alindsay/projects/orangerabbit-io/updown-io/flake.nix`

- [ ] **Step 1: Write flake.nix**

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "forwardemail";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            pkg-config
            openssl
          ];
        };
      });
}
```

- [ ] **Step 2: Verify nix develop works**

Run: `nix develop --command cargo check --workspace`
Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add flake.nix flake.lock
git commit -m "chore: add flake.nix with build package and dev shell"
```

### Task 29: Release Configuration

**Files:**
- Create: `.releaserc.json`
- Create: `.github/workflows/release.yml`

- [ ] **Step 0: Create package.json for semantic-release**

```json
{
  "private": true,
  "devDependencies": {
    "semantic-release": "^24",
    "@semantic-release/changelog": "^6",
    "@semantic-release/exec": "^6",
    "@semantic-release/git": "^10",
    "@semantic-release/github": "^11"
  }
}
```

Run: `npm install` to generate `package-lock.json`. Both files must be committed.

- [ ] **Step 1: Create .releaserc.json**

```json
{
  "branches": ["main"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    ["@semantic-release/changelog", {
      "changelogFile": "CHANGELOG.md"
    }],
    ["@semantic-release/exec", {
      "prepareCmd": "sed -i 's/^version = .*/version = \"${nextRelease.version}\"/' forwardemail/Cargo.toml forwardemail-lib/Cargo.toml && cargo generate-lockfile"
    }],
    ["@semantic-release/git", {
      "assets": ["CHANGELOG.md", "Cargo.lock", "forwardemail/Cargo.toml", "forwardemail-lib/Cargo.toml"],
      "message": "chore(release): ${nextRelease.version}\n\n${nextRelease.notes}"
    }],
    "@semantic-release/github"
  ]
}
```

- [ ] **Step 2: Create release workflow**

```yaml
name: Release

on:
  push:
    branches: [main]

permissions:
  contents: write
  issues: write
  pull-requests: write

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      - run: npm ci
      - run: npx semantic-release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

- [ ] **Step 3: Commit**

```bash
git add package.json package-lock.json .releaserc.json .github/workflows/release.yml
git commit -m "ci: add semantic-release config and GitHub Actions release workflow"
```

### Task 30: CLAUDE.md

**Files:**
- Create: `CLAUDE.md`

- [ ] **Step 1: Write project CLAUDE.md**

```markdown
# Forward Email CLI + TUI

Rust CLI and TUI for the Forward Email API. Cargo workspace with two crates.

## Structure

- `forwardemail-lib/` — Library crate: API client, config, models
- `forwardemail/` — Binary crate: CLI commands, output formatting, TUI

## Building

```bash
cargo build --workspace
cargo test --workspace
```

## Testing

- Unit tests in library crate: `cargo test -p forwardemail-lib`
- Integration tests in binary crate: `cargo test -p forwardemail`
- Integration tests use `mockito` for HTTP mocking — no live API calls
- Use `FORWARDEMAIL_BASE_URL` env var to point at mock server

## API

- Base URL: `https://api.forwardemail.net`
- Auth: HTTP Basic Auth, API token as username, empty password
- Config resolution: `--api-key` flag > `FORWARDEMAIL_API_KEY` env > `~/.config/forwardemail/config.toml`

## Patterns

- Follow updown-io project patterns (noun-verb CLI, blocking reqwest, tabled output)
- Output: markdown tables via `tabled` with `Style::markdown()`, or `--json` for raw JSON
- Models: serde structs + Row types with `From<&Model>` conversion
- Commands: clap derive enums with `run()` dispatch functions
- Tests: mockito server per test, assert_cmd for binary invocation, fixture JSON files
```

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add CLAUDE.md with project instructions"
```

### Task 31: Run Full Test Suite

- [ ] **Step 1: Run all tests**

Run: `cargo test --workspace`
Expected: all library unit tests + binary integration tests pass

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: no warnings

- [ ] **Step 3: Fix any clippy issues**

If clippy reports issues, fix them and commit:
```bash
git add -A && git commit -m "fix: address clippy warnings"
```

- [ ] **Step 4: Run fmt check**

Run: `cargo fmt --all -- --check`
Expected: no formatting issues

- [ ] **Step 5: Final verification commit if needed**

Only if changes were required in steps 2-4.
