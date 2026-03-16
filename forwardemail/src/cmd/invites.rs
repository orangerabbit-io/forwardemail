use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

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
    let _ = (action, client, mode);
    todo!()
}
