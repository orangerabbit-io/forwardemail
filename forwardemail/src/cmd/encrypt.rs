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
