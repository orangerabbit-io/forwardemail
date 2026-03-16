use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::alias::{Alias, AliasRow, GeneratedPassword};

use crate::cmd::domains::parse_pagination;
use crate::output::{self, OutputMode};

#[derive(Subcommand)]
pub enum AliasesAction {
    /// List aliases for a domain
    List {
        domain: String,
        #[arg(short, long)]
        q: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        recipient: Option<String>,
        #[arg(long)]
        sort: Option<String>,
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Create an alias
    Create {
        domain: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_delimiter = ',')]
        recipients: Option<Vec<String>>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_delimiter = ',')]
        labels: Option<Vec<String>>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        is_enabled: Option<bool>,
        #[arg(long, value_parser = ["250", "421", "550"])]
        error_code_if_disabled: Option<String>,
        #[arg(long)]
        has_imap: Option<bool>,
        #[arg(long)]
        has_pgp: Option<bool>,
        #[arg(long)]
        public_key: Option<String>,
        #[arg(long)]
        max_quota: Option<String>,
        #[arg(long)]
        vacation_responder_is_enabled: Option<bool>,
        #[arg(long)]
        vacation_responder_start_date: Option<String>,
        #[arg(long)]
        vacation_responder_end_date: Option<String>,
        #[arg(long)]
        vacation_responder_subject: Option<String>,
        #[arg(long)]
        vacation_responder_message: Option<String>,
    },
    /// Get alias details
    Get { domain: String, alias_id: String },
    /// Update an alias
    Update {
        domain: String,
        alias_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_delimiter = ',')]
        recipients: Option<Vec<String>>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_delimiter = ',')]
        labels: Option<Vec<String>>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        is_enabled: Option<bool>,
        #[arg(long, value_parser = ["250", "421", "550"])]
        error_code_if_disabled: Option<String>,
        #[arg(long)]
        has_imap: Option<bool>,
        #[arg(long)]
        has_pgp: Option<bool>,
        #[arg(long)]
        public_key: Option<String>,
        #[arg(long)]
        max_quota: Option<String>,
        #[arg(long)]
        vacation_responder_is_enabled: Option<bool>,
        #[arg(long)]
        vacation_responder_start_date: Option<String>,
        #[arg(long)]
        vacation_responder_end_date: Option<String>,
        #[arg(long)]
        vacation_responder_subject: Option<String>,
        #[arg(long)]
        vacation_responder_message: Option<String>,
    },
    /// Delete an alias
    Delete { domain: String, alias_id: String },
    /// Generate alias password
    #[command(name = "generate-password")]
    GeneratePassword {
        domain: String,
        alias_id: String,
        #[arg(long)]
        new_password: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        is_override: Option<bool>,
        #[arg(long)]
        emailed_instructions: Option<String>,
    },
}

struct AliasBodyParams {
    name: Option<String>,
    recipients: Option<Vec<String>>,
    description: Option<String>,
    labels: Option<Vec<String>>,
    has_recipient_verification: Option<bool>,
    is_enabled: Option<bool>,
    error_code_if_disabled: Option<String>,
    has_imap: Option<bool>,
    has_pgp: Option<bool>,
    public_key: Option<String>,
    max_quota: Option<String>,
    vacation_responder_is_enabled: Option<bool>,
    vacation_responder_start_date: Option<String>,
    vacation_responder_end_date: Option<String>,
    vacation_responder_subject: Option<String>,
    vacation_responder_message: Option<String>,
}

fn build_alias_body(
    params: AliasBodyParams,
) -> std::collections::HashMap<String, serde_json::Value> {
    let mut body = std::collections::HashMap::new();
    if let Some(v) = params.name {
        body.insert("name".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.recipients {
        body.insert("recipients".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.description {
        body.insert("description".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.labels {
        body.insert("labels".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.has_recipient_verification {
        body.insert(
            "has_recipient_verification".to_string(),
            serde_json::json!(v),
        );
    }
    if let Some(v) = params.is_enabled {
        body.insert("is_enabled".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.error_code_if_disabled {
        body.insert(
            "error_code_if_disabled".to_string(),
            serde_json::json!(v.parse::<u32>().unwrap_or(250)),
        );
    }
    if let Some(v) = params.has_imap {
        body.insert("has_imap".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.has_pgp {
        body.insert("has_pgp".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.public_key {
        body.insert("public_key".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.max_quota {
        body.insert("max_quota".to_string(), serde_json::json!(v));
    }
    if let Some(v) = params.vacation_responder_is_enabled {
        body.insert(
            "vacation_responder_is_enabled".to_string(),
            serde_json::json!(v),
        );
    }
    if let Some(v) = params.vacation_responder_start_date {
        body.insert(
            "vacation_responder_start_date".to_string(),
            serde_json::json!(v),
        );
    }
    if let Some(v) = params.vacation_responder_end_date {
        body.insert(
            "vacation_responder_end_date".to_string(),
            serde_json::json!(v),
        );
    }
    if let Some(v) = params.vacation_responder_subject {
        body.insert(
            "vacation_responder_subject".to_string(),
            serde_json::json!(v),
        );
    }
    if let Some(v) = params.vacation_responder_message {
        body.insert(
            "vacation_responder_message".to_string(),
            serde_json::json!(v),
        );
    }
    body
}

pub fn run(action: AliasesAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        AliasesAction::List {
            domain,
            q,
            name,
            recipient,
            sort,
            page,
            limit,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = &q {
                params.push(("q", v.clone()));
            }
            if let Some(v) = &name {
                params.push(("name", v.clone()));
            }
            if let Some(v) = &recipient {
                params.push(("recipient", v.clone()));
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
            let path = format!("/v1/domains/{}/aliases", domain);
            let resp = client.get_with_params(&path, &param_refs)?;
            let pagination = parse_pagination(&resp);
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let aliases: Vec<Alias> = resp.json()?;
                    let rows: Vec<AliasRow> = aliases.iter().map(AliasRow::from).collect();
                    output::print_table(&rows);
                    if let Some((current, total, items)) = pagination {
                        output::print_pagination(current, total, items);
                    }
                }
            }
            Ok(())
        }
        AliasesAction::Create {
            domain,
            name,
            recipients,
            description,
            labels,
            has_recipient_verification,
            is_enabled,
            error_code_if_disabled,
            has_imap,
            has_pgp,
            public_key,
            max_quota,
            vacation_responder_is_enabled,
            vacation_responder_start_date,
            vacation_responder_end_date,
            vacation_responder_subject,
            vacation_responder_message,
        } => {
            let body = build_alias_body(AliasBodyParams {
                name,
                recipients,
                description,
                labels,
                has_recipient_verification,
                is_enabled,
                error_code_if_disabled,
                has_imap,
                has_pgp,
                public_key,
                max_quota,
                vacation_responder_is_enabled,
                vacation_responder_start_date,
                vacation_responder_end_date,
                vacation_responder_subject,
                vacation_responder_message,
            });
            let path = format!("/v1/domains/{}/aliases", domain);
            let resp = client.post(&path, &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let alias: Alias = resp.json()?;
                    output::print_confirm(&format!(
                        "Alias created: {} ({})",
                        alias.name.unwrap_or("-".to_string()),
                        alias.id
                    ));
                }
            }
            Ok(())
        }
        AliasesAction::Get { domain, alias_id } => {
            let path = format!("/v1/domains/{}/aliases/{}", domain, alias_id);
            let resp = client.get(&path)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let a: Alias = resp.json()?;
                    let pairs = vec![
                        ("ID", a.id.clone()),
                        ("Name", a.name.clone().unwrap_or("-".to_string())),
                        ("Domain", a.domain.clone().unwrap_or("-".to_string())),
                        (
                            "Recipients",
                            a.recipients
                                .as_ref()
                                .map(|r| r.join(", "))
                                .unwrap_or("-".to_string()),
                        ),
                        (
                            "Enabled",
                            a.is_enabled
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                        (
                            "IMAP",
                            a.has_imap
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                        (
                            "PGP",
                            a.has_pgp
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                        ("Created", a.created_at.clone().unwrap_or("-".to_string())),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
        AliasesAction::Update {
            domain,
            alias_id,
            name,
            recipients,
            description,
            labels,
            has_recipient_verification,
            is_enabled,
            error_code_if_disabled,
            has_imap,
            has_pgp,
            public_key,
            max_quota,
            vacation_responder_is_enabled,
            vacation_responder_start_date,
            vacation_responder_end_date,
            vacation_responder_subject,
            vacation_responder_message,
        } => {
            let body = build_alias_body(AliasBodyParams {
                name,
                recipients,
                description,
                labels,
                has_recipient_verification,
                is_enabled,
                error_code_if_disabled,
                has_imap,
                has_pgp,
                public_key,
                max_quota,
                vacation_responder_is_enabled,
                vacation_responder_start_date,
                vacation_responder_end_date,
                vacation_responder_subject,
                vacation_responder_message,
            });
            let path = format!("/v1/domains/{}/aliases/{}", domain, alias_id);
            let resp = client.put(&path, &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    output::print_confirm(&format!("Alias updated: {}", alias_id));
                }
            }
            Ok(())
        }
        AliasesAction::Delete { domain, alias_id } => {
            let path = format!("/v1/domains/{}/aliases/{}", domain, alias_id);
            client.delete(&path)?;
            output::print_confirm(&format!("Alias deleted: {}", alias_id));
            Ok(())
        }
        AliasesAction::GeneratePassword {
            domain,
            alias_id,
            new_password,
            password,
            is_override,
            emailed_instructions,
        } => {
            let mut body = std::collections::HashMap::new();
            if let Some(v) = new_password {
                body.insert("new_password".to_string(), serde_json::json!(v));
            }
            if let Some(v) = password {
                body.insert("password".to_string(), serde_json::json!(v));
            }
            if let Some(v) = is_override {
                body.insert("is_override".to_string(), serde_json::json!(v));
            }
            if let Some(v) = emailed_instructions {
                body.insert("emailed_instructions".to_string(), serde_json::json!(v));
            }
            let path = format!(
                "/v1/domains/{}/aliases/{}/generate-password",
                domain, alias_id
            );
            let resp = client.post(&path, &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let gp: GeneratedPassword = resp.json()?;
                    if let Some(pw) = gp.password {
                        let pairs = vec![("Password", pw)];
                        output::print_kv(&pairs);
                    } else {
                        output::print_confirm("Password generated (check email)");
                    }
                }
            }
            Ok(())
        }
    }
}
