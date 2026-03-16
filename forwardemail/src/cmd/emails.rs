use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::email::{Email, EmailLimit, EmailRow};

use crate::cmd::domains::parse_pagination;
use crate::output::{self, OutputMode};

#[derive(Subcommand)]
pub enum EmailsAction {
    /// List outbound emails
    List {
        #[arg(short, long)]
        q: Option<String>,
        #[arg(long)]
        domain: Option<String>,
        #[arg(long)]
        sort: Option<String>,
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Send an email
    Send {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        cc: Option<String>,
        #[arg(long)]
        bcc: Option<String>,
        #[arg(long)]
        subject: Option<String>,
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        html: Option<String>,
        #[arg(long)]
        reply_to: Option<String>,
        #[arg(long)]
        priority: Option<String>,
    },
    /// Get email details
    Get { id: String },
    /// Delete an email
    Delete { id: String },
    /// Show daily email send limit
    Limit,
}

pub fn run(action: EmailsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        EmailsAction::List {
            q,
            domain,
            sort,
            page,
            limit,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = &q {
                params.push(("q", v.clone()));
            }
            if let Some(v) = &domain {
                params.push(("domain", v.clone()));
            }
            if let Some(v) = &sort {
                params.push(("sort", v.clone()));
            }
            if let Some(v) = page {
                params.push(("page", v.to_string()));
            }
            if let Some(v) = limit {
                params.push(("limit", v.to_string()));
            }
            let param_refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let resp = client.get_with_params("/v1/emails", &param_refs)?;
            let pagination = parse_pagination(&resp);
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let emails: Vec<Email> = resp.json()?;
                    let rows: Vec<EmailRow> = emails.iter().map(EmailRow::from).collect();
                    output::print_table(&rows);
                    if let Some((current, total, items)) = pagination {
                        output::print_pagination(current, total, items);
                    }
                }
            }
            Ok(())
        }
        EmailsAction::Send {
            from,
            to,
            cc,
            bcc,
            subject,
            text,
            html,
            reply_to,
            priority,
        } => {
            let mut body = std::collections::HashMap::new();
            body.insert("from".to_string(), serde_json::json!(from));
            body.insert("to".to_string(), serde_json::json!(to));
            if let Some(v) = cc {
                body.insert("cc".to_string(), serde_json::json!(v));
            }
            if let Some(v) = bcc {
                body.insert("bcc".to_string(), serde_json::json!(v));
            }
            if let Some(v) = subject {
                body.insert("subject".to_string(), serde_json::json!(v));
            }
            if let Some(v) = text {
                body.insert("text".to_string(), serde_json::json!(v));
            }
            if let Some(v) = html {
                body.insert("html".to_string(), serde_json::json!(v));
            }
            if let Some(v) = reply_to {
                body.insert("reply_to".to_string(), serde_json::json!(v));
            }
            if let Some(v) = priority {
                body.insert("priority".to_string(), serde_json::json!(v));
            }
            let resp = client.post("/v1/emails", &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let email: Email = resp.json()?;
                    output::print_confirm(&format!(
                        "Email queued: {} ({})",
                        email.subject.unwrap_or("-".to_string()),
                        email.id
                    ));
                }
            }
            Ok(())
        }
        EmailsAction::Get { id } => {
            let resp = client.get(&format!("/v1/emails/{}", id))?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let e: Email = resp.json()?;
                    let pairs = vec![
                        ("ID", e.id.clone()),
                        ("Status", e.status.clone().unwrap_or("-".to_string())),
                        ("From", e.from.clone().unwrap_or("-".to_string())),
                        (
                            "To",
                            e.to.as_ref()
                                .map(|t| t.join(", "))
                                .unwrap_or("-".to_string()),
                        ),
                        ("Subject", e.subject.clone().unwrap_or("-".to_string())),
                        ("Created", e.created_at.clone().unwrap_or("-".to_string())),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
        EmailsAction::Delete { id } => {
            client.delete(&format!("/v1/emails/{}", id))?;
            output::print_confirm(&format!("Email deleted: {}", id));
            Ok(())
        }
        EmailsAction::Limit => {
            let resp = client.get("/v1/emails/limit")?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let limit: EmailLimit = resp.json()?;
                    let pairs = vec![
                        ("Sent today", limit.count.to_string()),
                        ("Daily limit", limit.limit.to_string()),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
    }
}
