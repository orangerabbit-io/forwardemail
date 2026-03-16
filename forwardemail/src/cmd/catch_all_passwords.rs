use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

#[derive(Subcommand)]
pub enum CatchAllPasswordsAction {
    /// List catch-all passwords for a domain
    List { domain: String },
    /// Create a catch-all password
    Create {
        domain: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a catch-all password
    Delete {
        domain: String,
        token_id: String,
    },
}

pub fn run(action: CatchAllPasswordsAction, client: &Client, mode: OutputMode) -> Result<()> {
    let _ = (action, client, mode);
    todo!()
}
