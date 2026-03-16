use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;
use forwardemail_lib::models::domain::{Domain, DomainRow};

use crate::output::{self, OutputMode};

#[derive(Subcommand)]
pub enum DomainsAction {
    /// List all domains
    List {
        #[arg(short, long)]
        q: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        sort: Option<String>,
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Create a new domain
    Create {
        /// Domain name (FQDN)
        domain: String,
        #[arg(long, value_parser = ["free", "enhanced_protection", "team"])]
        plan: Option<String>,
        #[arg(long)]
        team_domain: Option<String>,
        #[arg(long)]
        catchall: Option<String>,
        #[arg(long)]
        has_adult_content_protection: Option<bool>,
        #[arg(long)]
        has_phishing_protection: Option<bool>,
        #[arg(long)]
        has_executable_protection: Option<bool>,
        #[arg(long)]
        has_virus_protection: Option<bool>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        ignore_mx_check: Option<bool>,
        #[arg(long)]
        retention_days: Option<u32>,
        #[arg(long)]
        bounce_webhook: Option<String>,
        #[arg(long)]
        max_quota_per_alias: Option<String>,
    },
    /// Get domain details
    Get { domain: String },
    /// Update domain settings
    Update {
        domain: String,
        #[arg(long)]
        smtp_port: Option<String>,
        #[arg(long)]
        has_adult_content_protection: Option<bool>,
        #[arg(long)]
        has_phishing_protection: Option<bool>,
        #[arg(long)]
        has_executable_protection: Option<bool>,
        #[arg(long)]
        has_virus_protection: Option<bool>,
        #[arg(long)]
        has_recipient_verification: Option<bool>,
        #[arg(long)]
        ignore_mx_check: Option<bool>,
        #[arg(long)]
        retention_days: Option<u32>,
        #[arg(long)]
        bounce_webhook: Option<String>,
        #[arg(long)]
        max_quota_per_alias: Option<String>,
    },
    /// Delete a domain
    Delete { domain: String },
    /// Verify DNS records
    #[command(name = "verify-records")]
    VerifyRecords { domain: String },
    /// Verify SMTP
    #[command(name = "verify-smtp")]
    VerifySmtp { domain: String },
}

pub(crate) fn parse_pagination(resp: &reqwest::blocking::Response) -> Option<(u32, u32, u32)> {
    let current = resp
        .headers()
        .get("X-Page-Current")?
        .to_str()
        .ok()?
        .parse()
        .ok()?;
    let total_pages = resp
        .headers()
        .get("X-Page-Count")?
        .to_str()
        .ok()?
        .parse()
        .ok()?;
    let total_items = resp
        .headers()
        .get("X-Item-Count")?
        .to_str()
        .ok()?
        .parse()
        .ok()?;
    Some((current, total_pages, total_items))
}

pub fn run(action: DomainsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DomainsAction::List {
            q,
            name,
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
            let resp = client.get_with_params("/v1/domains", &param_refs)?;
            let pagination = parse_pagination(&resp);
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let domains: Vec<Domain> = resp.json()?;
                    let rows: Vec<DomainRow> = domains.iter().map(DomainRow::from).collect();
                    output::print_table(&rows);
                    if let Some((current, total, items)) = pagination {
                        output::print_pagination(current, total, items);
                    }
                }
            }
            Ok(())
        }
        DomainsAction::Create {
            domain,
            plan,
            team_domain,
            catchall,
            has_adult_content_protection,
            has_phishing_protection,
            has_executable_protection,
            has_virus_protection,
            has_recipient_verification,
            ignore_mx_check,
            retention_days,
            bounce_webhook,
            max_quota_per_alias,
        } => {
            let mut body = std::collections::HashMap::new();
            body.insert("domain".to_string(), serde_json::json!(domain));
            if let Some(v) = plan {
                body.insert("plan".to_string(), serde_json::json!(v));
            }
            if let Some(v) = team_domain {
                body.insert("team_domain".to_string(), serde_json::json!(v));
            }
            if let Some(v) = catchall {
                body.insert("catchall".to_string(), serde_json::json!(v));
            }
            if let Some(v) = has_adult_content_protection {
                body.insert(
                    "has_adult_content_protection".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = has_phishing_protection {
                body.insert("has_phishing_protection".to_string(), serde_json::json!(v));
            }
            if let Some(v) = has_executable_protection {
                body.insert(
                    "has_executable_protection".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = has_virus_protection {
                body.insert("has_virus_protection".to_string(), serde_json::json!(v));
            }
            if let Some(v) = has_recipient_verification {
                body.insert(
                    "has_recipient_verification".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = ignore_mx_check {
                body.insert("ignore_mx_check".to_string(), serde_json::json!(v));
            }
            if let Some(v) = retention_days {
                body.insert("retention_days".to_string(), serde_json::json!(v));
            }
            if let Some(v) = bounce_webhook {
                body.insert("bounce_webhook".to_string(), serde_json::json!(v));
            }
            if let Some(v) = max_quota_per_alias {
                body.insert("max_quota_per_alias".to_string(), serde_json::json!(v));
            }
            let resp = client.post("/v1/domains", &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    output::print_confirm(&format!("Domain created: {}", domain));
                }
            }
            Ok(())
        }
        DomainsAction::Get { domain } => {
            let resp = client.get(&format!("/v1/domains/{}", domain))?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let d: Domain = resp.json()?;
                    let pairs = vec![
                        ("Domain", d.name.clone()),
                        ("Plan", d.plan.clone().unwrap_or("-".to_string())),
                        ("SMTP Port", d.smtp_port.clone().unwrap_or("-".to_string())),
                        (
                            "MX Record",
                            d.has_mx_record
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                        (
                            "TXT Record",
                            d.has_txt_record
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                        (
                            "Retention Days",
                            d.retention_days
                                .map(|v| v.to_string())
                                .unwrap_or("-".to_string()),
                        ),
                        ("Created", d.created_at.clone().unwrap_or("-".to_string())),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
        DomainsAction::Update {
            domain,
            smtp_port,
            has_adult_content_protection,
            has_phishing_protection,
            has_executable_protection,
            has_virus_protection,
            has_recipient_verification,
            ignore_mx_check,
            retention_days,
            bounce_webhook,
            max_quota_per_alias,
        } => {
            let mut body = std::collections::HashMap::new();
            if let Some(v) = smtp_port {
                body.insert("smtp_port".to_string(), serde_json::json!(v));
            }
            if let Some(v) = has_adult_content_protection {
                body.insert(
                    "has_adult_content_protection".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = has_phishing_protection {
                body.insert("has_phishing_protection".to_string(), serde_json::json!(v));
            }
            if let Some(v) = has_executable_protection {
                body.insert(
                    "has_executable_protection".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = has_virus_protection {
                body.insert("has_virus_protection".to_string(), serde_json::json!(v));
            }
            if let Some(v) = has_recipient_verification {
                body.insert(
                    "has_recipient_verification".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = ignore_mx_check {
                body.insert("ignore_mx_check".to_string(), serde_json::json!(v));
            }
            if let Some(v) = retention_days {
                body.insert("retention_days".to_string(), serde_json::json!(v));
            }
            if let Some(v) = bounce_webhook {
                body.insert("bounce_webhook".to_string(), serde_json::json!(v));
            }
            if let Some(v) = max_quota_per_alias {
                body.insert("max_quota_per_alias".to_string(), serde_json::json!(v));
            }
            let resp = client.put(&format!("/v1/domains/{}", domain), &body)?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    output::print_confirm(&format!("Domain updated: {}", domain));
                }
            }
            Ok(())
        }
        DomainsAction::Delete { domain } => {
            client.delete(&format!("/v1/domains/{}", domain))?;
            output::print_confirm(&format!("Domain deleted: {}", domain));
            Ok(())
        }
        DomainsAction::VerifyRecords { domain } => {
            let resp = client.get(&format!("/v1/domains/{}/verify-records", domain))?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let d: Domain = resp.json()?;
                    let pairs = vec![
                        ("Domain", d.name),
                        (
                            "MX Record",
                            d.has_mx_record
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                        (
                            "TXT Record",
                            d.has_txt_record
                                .map(|b| if b { "yes" } else { "no" })
                                .unwrap_or("-")
                                .to_string(),
                        ),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
        DomainsAction::VerifySmtp { domain } => {
            let resp = client.get(&format!("/v1/domains/{}/verify-smtp", domain))?;
            match mode {
                OutputMode::Json => {
                    let json: serde_json::Value = resp.json()?;
                    output::print_json(&json);
                }
                OutputMode::Table => {
                    let d: Domain = resp.json()?;
                    let pairs = vec![
                        ("Domain", d.name),
                        ("SMTP Port", d.smtp_port.clone().unwrap_or("-".to_string())),
                    ];
                    output::print_kv(&pairs);
                }
            }
            Ok(())
        }
    }
}
