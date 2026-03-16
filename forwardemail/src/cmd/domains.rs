use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

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

pub fn run(action: DomainsAction, client: &Client, mode: OutputMode) -> Result<()> {
    let _ = (action, client, mode);
    todo!()
}
