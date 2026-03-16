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
