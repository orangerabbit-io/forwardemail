use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::catch_all_password::{CatchAllPassword, CatchAllPasswordRow};

use crate::output::{self, OutputMode};

#[derive(Subcommand)]
pub enum CatchAllPasswordsAction {
    /// List catch-all passwords for a domain
    List { domain: String },
    /// Create a catch-all password
    Create {
        domain: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a catch-all password
    Delete { domain: String, token_id: String },
}

pub fn run(action: CatchAllPasswordsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        CatchAllPasswordsAction::List { domain } => {
            let path = format!("/v1/domains/{}/catch-all-passwords", domain);
            let resp = client.get(&path)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let passwords: Vec<CatchAllPassword> = resp.json()?;
                    let rows: Vec<CatchAllPasswordRow> =
                        passwords.iter().map(CatchAllPasswordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        CatchAllPasswordsAction::Create {
            domain,
            password,
            description,
        } => {
            let mut body = std::collections::HashMap::new();
            body.insert("password".to_string(), serde_json::json!(password));
            if let Some(v) = description {
                body.insert("description".to_string(), serde_json::json!(v));
            }
            let path = format!("/v1/domains/{}/catch-all-passwords", domain);
            let resp = client.post(&path, &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let cap: CatchAllPassword = resp.json()?;
                    output::print_confirm(&format!(
                        "Catch-all password created: {}",
                        cap.id.unwrap_or("-".to_string())
                    ));
                }
            }
            Ok(())
        }
        CatchAllPasswordsAction::Delete { domain, token_id } => {
            let path = format!("/v1/domains/{}/catch-all-passwords/{}", domain, token_id);
            client.delete(&path)?;
            output::print_confirm(&format!("Catch-all password deleted: {}", token_id));
            Ok(())
        }
    }
}
