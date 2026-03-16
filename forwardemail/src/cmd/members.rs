use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::member::Member;

use crate::output::{self, OutputMode};

#[derive(Subcommand)]
pub enum MembersAction {
    /// Update a domain member
    Update {
        domain: String,
        member_id: String,
        #[arg(long, value_parser = ["admin", "user"])]
        group: String,
    },
    /// Remove a domain member
    Remove { domain: String, member_id: String },
}

pub fn run(action: MembersAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        MembersAction::Update {
            domain,
            member_id,
            group,
        } => {
            let mut body = std::collections::HashMap::new();
            body.insert("group".to_string(), serde_json::json!(group));
            let path = format!("/v1/domains/{}/members/{}", domain, member_id);
            let resp = client.put(&path, &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let member: Member = resp.json()?;
                    output::print_confirm(&format!(
                        "Member updated: {} ({})",
                        member.email.unwrap_or("-".to_string()),
                        member.group.unwrap_or("-".to_string())
                    ));
                }
            }
            Ok(())
        }
        MembersAction::Remove { domain, member_id } => {
            let path = format!("/v1/domains/{}/members/{}", domain, member_id);
            client.delete(&path)?;
            output::print_confirm(&format!("Member removed: {}", member_id));
            Ok(())
        }
    }
}
