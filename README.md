# forwardemail

A command-line interface for the [Forward Email](https://forwardemail.net) API.

## Install

### From source

```sh
git clone https://github.com/orangerabbit-io/forwardemail.git
cd forwardemail
cargo build --release
cp target/release/forwardemail ~/.local/bin/
```

### With Nix

```sh
nix run github:orangerabbit-io/forwardemail
```

## Configuration

Create `~/.config/forwardemail/config.toml`:

```toml
api_key = "your-api-key"
```

Get your API key from [forwardemail.net/my-account/security](https://forwardemail.net/my-account/security).

Alternatively, set the `FORWARDEMAIL_API_KEY` environment variable or pass `--api-key` on every command.

Priority: `--api-key` flag > `FORWARDEMAIL_API_KEY` env > config file.

## Usage

### Account

```sh
forwardemail account get
forwardemail account create --email user@example.com --password secret
forwardemail account update --given-name John --family-name Doe
```

### Domains

```sh
forwardemail domains list
forwardemail domains create example.com --plan enhanced_protection
forwardemail domains get example.com
forwardemail domains update example.com --retention-days 30
forwardemail domains delete example.com
forwardemail domains verify-records example.com
forwardemail domains verify-smtp example.com
```

### Aliases

```sh
forwardemail aliases list example.com
forwardemail aliases create example.com --name info --recipients user@example.com
forwardemail aliases get example.com <alias_id>
forwardemail aliases update example.com <alias_id> --is-enabled false
forwardemail aliases delete example.com <alias_id>
forwardemail aliases generate-password example.com <alias_id>
```

### Emails

```sh
forwardemail emails list
forwardemail emails send --from sender@example.com --to recipient@example.com --subject "Hello" --text "Body"
forwardemail emails get <id>
forwardemail emails delete <id>
forwardemail emails limit
```

### Logs

```sh
forwardemail logs download
forwardemail logs download --domain example.com
forwardemail logs download -q "bounce" --response-code 550
```

### Invites

```sh
forwardemail invites create example.com --email user@example.com --group admin
forwardemail invites accept example.com
forwardemail invites remove example.com --email user@example.com
```

### Members

```sh
forwardemail members update example.com <member_id> --group user
forwardemail members remove example.com <member_id>
```

### Catch-All Passwords

```sh
forwardemail catch-all-passwords list example.com
forwardemail catch-all-passwords create example.com --description "Main"
forwardemail catch-all-passwords delete example.com <token_id>
```

### Encrypt

```sh
forwardemail encrypt "forward-email=user@example.com"
```

### Output

Markdown table output by default. Add `--json` to any command for JSON:

```sh
forwardemail domains list --json
forwardemail domains list --json | jq '.[].name'
```

## API Coverage

| Resource | Commands |
|----------|----------|
| Account | create, get, update |
| Domains | list, create, get, update, delete, verify-records, verify-smtp |
| Aliases | list, create, get, update, delete, generate-password |
| Emails | list, send, get, delete, limit |
| Logs | download |
| Invites | create, accept, remove |
| Members | update, remove |
| Catch-All Passwords | list, create, delete |
| Encrypt | encrypt |

## Development

```sh
cargo test --workspace   # unit + integration (mocked)
cargo clippy --workspace # lint
cargo fmt --all          # format
```

Mock tests use [mockito](https://crates.io/crates/mockito) -- no API key needed.

### Live API tests

```sh
FORWARDEMAIL_LIVE_TEST=1 cargo test --test live_test -- --test-threads=1
```

Requires `FORWARDEMAIL_API_KEY`. Exercises real endpoints including domain/alias/password CRUD lifecycles with automatic cleanup.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
