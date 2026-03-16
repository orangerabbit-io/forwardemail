use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

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
    let _ = (action, client, mode);
    todo!()
}
