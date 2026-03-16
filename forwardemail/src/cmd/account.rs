use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::account::Account;

use crate::output::{self, OutputMode};

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
