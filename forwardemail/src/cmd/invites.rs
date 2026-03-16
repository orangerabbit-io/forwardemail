use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::invite::Invite;

use crate::output::{self, OutputMode};

#[derive(Subcommand)]
pub enum InvitesAction {
    /// Create a domain invite
    Create {
        domain: String,
        #[arg(long)]
        email: String,
        #[arg(long, value_parser = ["admin", "user"])]
        group: String,
    },
    /// Accept a domain invite
    Accept { domain: String },
    /// Remove a domain invite
    Remove {
        domain: String,
        #[arg(long)]
        email: String,
    },
}

pub fn run(action: InvitesAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        InvitesAction::Create {
            domain,
            email,
            group,
        } => {
            let mut body = std::collections::HashMap::new();
            body.insert("email".to_string(), serde_json::json!(email));
            body.insert("group".to_string(), serde_json::json!(group));
            let path = format!("/v1/domains/{}/invites", domain);
            let resp = client.post(&path, &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let invite: Invite = resp.json()?;
                    output::print_confirm(&format!(
                        "Invite created for {} ({})",
                        invite.email.unwrap_or("-".to_string()),
                        invite.group.unwrap_or("-".to_string())
                    ));
                }
            }
            Ok(())
        }
        InvitesAction::Accept { domain } => {
            let path = format!("/v1/domains/{}/invites", domain);
            let resp = client.get(&path)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    output::print_confirm(&format!("Invite accepted for domain: {}", domain));
                }
            }
            Ok(())
        }
        InvitesAction::Remove { domain, email } => {
            let mut body = std::collections::HashMap::new();
            body.insert("email".to_string(), serde_json::json!(email));
            let path = format!("/v1/domains/{}/invites", domain);
            client.delete_with_body(&path, &body)?;
            output::print_confirm(&format!("Invite removed for {}", email));
            Ok(())
        }
    }
}
