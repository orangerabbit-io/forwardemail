use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

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
    Get {
        domain: String,
        alias_id: String,
    },
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
    Delete {
        domain: String,
        alias_id: String,
    },
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

pub fn run(action: AliasesAction, client: &Client, mode: OutputMode) -> Result<()> {
    let _ = (action, client, mode);
    todo!()
}
