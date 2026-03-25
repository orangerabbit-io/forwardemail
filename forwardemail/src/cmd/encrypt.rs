use anyhow::{Context, Result};

use crate::output::{self, OutputMode};

pub fn run(input: &str, mode: OutputMode) -> Result<()> {
    let base_url = forwardemail_lib::config::Config::base_url_only();
    let client = forwardemail_lib::client::Client::unauthenticated(base_url)?;

    let mut body = std::collections::HashMap::new();
    body.insert("input".to_string(), serde_json::json!(input));

    let resp = client.post("/v1/encrypt", &body)?;
    let text = resp.text().context("Failed to read encrypt response")?;

    match mode {
        OutputMode::Json => {
            let json = serde_json::json!({ "encrypted": text });
            output::print_json(&json);
        }
        OutputMode::Table => {
            println!("{}", text);
        }
    }

    Ok(())
}
