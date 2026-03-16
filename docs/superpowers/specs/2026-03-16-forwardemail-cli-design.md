# Forward Email CLI + TUI Design Spec

## Overview

A Rust CLI and TUI for the Forward Email API (forwardemail.net). Single binary (`forwardemail`) provides both a command-line interface covering the complete API and a read-only TUI for browsing. The project is structured as a Cargo workspace with a reusable library crate.

## API Coverage

All currently available Forward Email API endpoints:

| Resource | Endpoints |
|---|---|
| Account | create, retrieve, update |
| Domains | list, create, get, update, delete, verify-records, verify-smtp |
| Aliases | list, create, get, update, delete, generate-password |
| Emails | list, send, get, delete, limit |
| Logs | download |
| Invites | create, accept, remove |
| Members | update, remove |
| Catch-all passwords | create |
| Encrypt | encrypt TXT records |

Excludes "coming soon" endpoints (contacts, calendars, messages, folders).

## Workspace Structure

```
forwardemail/
├── Cargo.toml              # workspace root
├── forwardemail-lib/       # library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── client.rs       # HTTP client (auth, error handling)
│       ├── config.rs       # 3-tier config resolution
│       └── models/
│           ├── mod.rs
│           ├── account.rs
│           ├── domain.rs
│           ├── alias.rs
│           ├── email.rs
│           ├── log.rs
│           ├── invite.rs
│           ├── member.rs
│           ├── catch_all_password.rs
│           └── encrypt.rs
├── forwardemail/           # binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # clap CLI + dispatch
│       ├── output.rs       # markdown table, JSON, KV formatting
│       ├── tui/
│       │   ├── mod.rs      # TUI entry point
│       │   ├── app.rs      # App state machine
│       │   ├── ui.rs       # Rendering
│       │   └── views/      # Per-resource views
│       └── cmd/
│           ├── mod.rs
│           ├── account.rs
│           ├── domains.rs
│           ├── aliases.rs
│           ├── emails.rs
│           ├── logs.rs
│           ├── invites.rs
│           ├── members.rs
│           ├── catch_all_passwords.rs
│           └── encrypt.rs
└── tests/                  # integration tests for binary
    ├── common/mod.rs
    ├── fixtures/           # JSON response fixtures
    ├── account_test.rs
    ├── domains_test.rs
    ├── aliases_test.rs
    ├── emails_test.rs
    ├── logs_test.rs
    ├── invites_test.rs
    ├── members_test.rs
    ├── catch_all_passwords_test.rs
    └── encrypt_test.rs
```

## Library Crate: `forwardemail-lib`

### Client (`client.rs`)

Wraps `reqwest::blocking::Client`. Handles authentication, request construction, and error mapping.

- **Authentication**: HTTP Basic Auth. API token as username, empty password. Matches the Forward Email API convention: `curl -u API_TOKEN:`.
- **Base URL**: Defaults to `https://api.forwardemail.net`. Overridable via `FORWARDEMAIL_BASE_URL` env var (used in testing with mockito).
- **Methods**:
  - `get(path) -> Result<Response>`
  - `get_with_params(path, params) -> Result<Response>`
  - `get_json<T: DeserializeOwned>(path) -> Result<T>`
  - `get_json_with_params<T: DeserializeOwned>(path, params) -> Result<T>`
  - `post(path, body) -> Result<Response>`
  - `put(path, body) -> Result<Response>`
  - `delete(path) -> Result<Response>`
  - `get_bytes(path) -> Result<Vec<u8>>` (for gzipped log downloads)
- **Error handling**: `check_status()` maps HTTP status codes to descriptive errors:
  - 401/403 -> "Authentication failed"
  - 404 -> "Not found"
  - 422 -> "Validation error" (includes response body)
  - 429 -> "Rate limited"
  - 5xx -> "Server error"
- **Headers**: Sends `Accept-Encoding: gzip`, `Content-Type: application/json`.

### Config (`config.rs`)

3-tier resolution, highest priority first:

1. `--api-key` CLI flag
2. `FORWARDEMAIL_API_KEY` environment variable
3. `~/.config/forwardemail/config.toml`

Config file format:
```toml
api_key = "your-api-token"
```

Error messages name the config file path. Exit code 2 for config errors, exit code 1 for API errors.

### Models (`models/`)

Each resource gets a module with:

1. **API model struct** — `#[derive(Debug, Deserialize, Serialize)]` with serde field mappings. Optional/nullable fields use `Option<T>`.
2. **Row struct** — implements `tabled::Tabled` for table display. `#[tabled(rename = "...")]` for column headers.
3. **`From<&Model> for Row`** — conversion with formatting (booleans to yes/no, dates to readable format, None to "-").

#### Account (`account.rs`)

```rust
pub struct Account {
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub avatar_url: Option<String>,
    pub plan: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct CreateAccount {
    pub email: String,
    pub password: String,
}

pub struct UpdateAccount {
    pub email: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub avatar_url: Option<String>,
}
```

#### Domain (`domain.rs`)

```rust
pub struct Domain {
    pub name: String,
    pub plan: String,
    pub max_recipients_per_alias: Option<u32>,
    pub smtp_port: Option<String>,
    pub has_adult_content_protection: Option<bool>,
    pub has_phishing_protection: Option<bool>,
    pub has_executable_protection: Option<bool>,
    pub has_virus_protection: Option<bool>,
    pub has_recipient_verification: Option<bool>,
    pub retention_days: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
    // verification fields from verify-records/verify-smtp
    pub has_mx_record: Option<bool>,
    pub has_txt_record: Option<bool>,
}

pub struct CreateDomain {
    pub domain: String,
    pub plan: Option<String>,         // "free", "enhanced_protection", "team"
    pub catchall: Option<String>,
    pub has_adult_content_protection: Option<bool>,
    pub has_phishing_protection: Option<bool>,
    pub has_executable_protection: Option<bool>,
    pub has_virus_protection: Option<bool>,
    pub has_recipient_verification: Option<bool>,
    pub ignore_mx_check: Option<bool>,
    pub retention_days: Option<u32>,
    pub bounce_webhook: Option<String>,
    pub max_quota_per_alias: Option<String>,
}

pub struct UpdateDomain {
    pub smtp_port: Option<String>,
    pub has_adult_content_protection: Option<bool>,
    pub has_phishing_protection: Option<bool>,
    pub has_executable_protection: Option<bool>,
    pub has_virus_protection: Option<bool>,
    pub has_recipient_verification: Option<bool>,
    pub ignore_mx_check: Option<bool>,
    pub retention_days: Option<u32>,
    pub bounce_webhook: Option<String>,
    pub max_quota_per_alias: Option<String>,
}
```

#### Alias (`alias.rs`)

```rust
pub struct Alias {
    pub id: String,
    pub name: String,
    pub domain: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub is_enabled: Option<bool>,
    pub has_recipient_verification: Option<bool>,
    pub has_imap: Option<bool>,
    pub has_pgp: Option<bool>,
    pub error_code_if_disabled: Option<u32>,
    pub vacation_responder_is_enabled: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct CreateAlias {
    pub name: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub has_recipient_verification: Option<bool>,
    pub is_enabled: Option<bool>,
    pub error_code_if_disabled: Option<u32>,
    pub has_imap: Option<bool>,
    pub has_pgp: Option<bool>,
    pub public_key: Option<String>,
    pub max_quota: Option<String>,
    pub vacation_responder_is_enabled: Option<bool>,
    pub vacation_responder_start_date: Option<String>,
    pub vacation_responder_end_date: Option<String>,
    pub vacation_responder_subject: Option<String>,
    pub vacation_responder_message: Option<String>,
}

pub struct GeneratePassword {
    pub new_password: Option<String>,
    pub password: Option<String>,
    pub is_override: Option<bool>,
    pub emailed_instructions: Option<String>,
}
```

#### Email (`email.rs`)

```rust
pub struct Email {
    pub id: String,
    pub status: Option<String>,
    pub from: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub subject: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct EmailLimit {
    pub limit: u32,
    pub remaining: u32,
    pub reset_at: Option<String>,
}

pub struct SendEmail {
    pub from: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub subject: Option<String>,
    pub text: Option<String>,
    pub html: Option<String>,
    pub reply_to: Option<String>,
    pub priority: Option<String>,  // "high", "normal", "low"
}
```

#### Other Models

- **Invite** (`invite.rs`): `email`, `group` ("admin"/"user")
- **Member** (`member.rs`): `id`, `email`, `group`, domain association
- **CatchAllPassword** (`catch_all_password.rs`): `new_password`, `description`
- **Encrypt** (`encrypt.rs`): `input` string, response with encrypted value

### Pagination

List endpoints accept common pagination params:
- `q` (search query)
- `sort` (field name, prefix `-` for descending)
- `page` (>= 1, default 1)
- `limit` (10-50, default 10)

A shared `PaginationParams` struct serializes to query parameters.

## Binary Crate: `forwardemail`

### CLI Structure

Clap derive with noun-verb pattern. Global flags: `--json`, `--api-key`.

```
forwardemail [--json] [--api-key KEY] <command>

Commands:
  account       Account management
  domains       Domain management
  aliases       Alias management
  emails        Outbound email management
  logs          Log management
  invites       Domain invite management
  members       Domain member management
  catch-all-passwords  Catch-all password management
  encrypt       Encrypt TXT records
  tui           Launch interactive TUI
```

#### Full Command Reference

```
forwardemail account create --email EMAIL --password PASS
forwardemail account get
forwardemail account update [--email EMAIL] [--given-name NAME] [--family-name NAME] [--avatar-url URL]

forwardemail domains list [-q SEARCH] [--sort FIELD] [--page N] [--limit N]
forwardemail domains create <DOMAIN> [--plan free|enhanced_protection|team] [--catchall BOOL] [--retention-days N] [flags...]
forwardemail domains get <DOMAIN>
forwardemail domains update <DOMAIN> [--smtp-port PORT] [--retention-days N] [flags...]
forwardemail domains delete <DOMAIN>
forwardemail domains verify-records <DOMAIN>
forwardemail domains verify-smtp <DOMAIN>

forwardemail aliases list <DOMAIN> [-q SEARCH] [--sort FIELD] [--page N] [--limit N]
forwardemail aliases create <DOMAIN> [--name NAME] [--recipients EMAILS] [--description DESC] [--labels LABELS] [flags...]
forwardemail aliases get <DOMAIN> <ALIAS_ID>
forwardemail aliases update <DOMAIN> <ALIAS_ID> [flags...]
forwardemail aliases delete <DOMAIN> <ALIAS_ID>
forwardemail aliases generate-password <DOMAIN> <ALIAS_ID> [--new-password PASS] [--emailed-instructions EMAIL]

forwardemail emails list [-q SEARCH] [--sort FIELD] [--page N] [--limit N]
forwardemail emails send [--from FROM] [--to TO] [--cc CC] [--bcc BCC] [--subject SUBJ] [--text TEXT] [--html HTML] [--reply-to ADDR] [--priority high|normal|low]
forwardemail emails get <ID>
forwardemail emails delete <ID>
forwardemail emails limit

forwardemail logs download [--domain DOMAIN] [-q SEARCH] [--bounce-category CAT] [--response-code CODE]

forwardemail invites create <DOMAIN> --email EMAIL --group admin|user
forwardemail invites accept <DOMAIN>
forwardemail invites remove <DOMAIN> --email EMAIL

forwardemail members update <DOMAIN> <MEMBER_ID> --group admin|user
forwardemail members remove <DOMAIN> <MEMBER_ID>

forwardemail catch-all-passwords create <DOMAIN> [--password PASS] [--description DESC]

forwardemail encrypt <INPUT>

forwardemail tui
```

### Output Formatting (`output.rs`)

- **`OutputMode::Table`**: Uses `tabled` with `Style::markdown()`. Markdown table output.
- **`OutputMode::Json`**: Pretty-printed JSON passthrough.
- **`print_kv(pairs)`**: Right-aligned key-value pairs for single-item detail views.
- **`print_confirm(message)`**: Single-line confirmation for mutations.
- **`print_raw(text)`**: Passthrough for binary/text responses (log downloads).

### TUI (`tui/`)

Launched via `forwardemail tui`. Read-only browsing of all resources.

**App state machine** (`app.rs`):
- States: `Dashboard`, `DomainList`, `DomainDetail`, `AliasList`, `AliasDetail`, `EmailList`, `EmailDetail`
- Navigation: arrow keys/vim keys for list navigation, Enter to drill in, Esc/Backspace to go back, q to quit
- Data loaded on navigation (blocking HTTP calls in the library)

**Views** (`views/`):
- Dashboard: account info summary
- Domain list: table of domains with plan, verification status
- Domain detail: KV view of domain fields
- Alias list: table of aliases for selected domain
- Alias detail: KV view of alias fields
- Email list: table of outbound emails
- Email detail: KV view of email fields

**Rendering** (`ui.rs`):
- Header bar with breadcrumb navigation
- Main content area (table or KV view)
- Footer with keybinding hints

## Dependencies

### Library (`forwardemail-lib`)

| Crate | Version | Features | Purpose |
|---|---|---|---|
| reqwest | 0.12 | blocking, json, gzip | HTTP client |
| serde | 1 | derive | Serialization |
| serde_json | 1 | | JSON handling |
| anyhow | 1 | | Error handling |
| toml | 0.8 | | Config file parsing |

### Binary (`forwardemail`)

| Crate | Version | Features | Purpose |
|---|---|---|---|
| forwardemail-lib | workspace | | API client |
| clap | 4 | derive | CLI parsing |
| tabled | 0.17 | | Table output |
| ratatui | latest | | TUI framework |
| crossterm | latest | | Terminal backend |

### Dev Dependencies

| Crate | Version | Purpose |
|---|---|---|
| mockito | 1 | HTTP mock server |
| assert_cmd | 2 | CLI binary testing |
| predicates | 3 | Assertion matchers |
| serial_test | 3 | Sequential test execution |

## Testing Strategy

### Library Unit Tests

- **Model deserialization**: Round-trip JSON fixtures through serde. Verify optional fields handle null/missing.
- **Config resolution**: Test priority order (flag > env > file). Test missing API key error. Use `serial_test` for env var tests.
- **Client error mapping**: Verify HTTP status codes map to correct error messages.

### Binary Integration Tests

Each resource gets a test file. Tests use `mockito::Server` for HTTP mocking and `assert_cmd` to invoke the compiled binary. The binary is pointed at the mock server via `FORWARDEMAIL_BASE_URL` env var.

**Test patterns per resource:**
1. **List (table output)**: Mock GET, verify markdown table contains expected fields
2. **List (JSON output)**: Mock GET with `--json` flag, verify JSON structure
3. **Get (detail view)**: Mock GET, verify KV output contains expected fields
4. **Create**: Mock POST, verify request body via `Matcher`, verify confirmation output
5. **Update**: Mock PUT, verify request body and confirmation
6. **Delete**: Mock DELETE, verify confirmation
7. **Auth error**: Mock 401, verify "Authentication failed" in stderr and exit code 1
8. **Not found**: Mock 404, verify error message
9. **Pagination params**: Verify query string encoding

### TUI Tests

- State transition tests on `App` struct: navigate forward/back, verify current state
- Data loading: verify correct API calls made for each state transition
- No render/screenshot tests

## Error Handling

- `anyhow::Result<T>` throughout
- Library returns errors; binary maps to exit codes
- Exit code 0: success
- Exit code 1: API/runtime error
- Exit code 2: configuration error
- Clap handles invalid arguments before Rust code runs
