use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

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
    Remove {
        domain: String,
        member_id: String,
    },
}

pub fn run(action: MembersAction, client: &Client, mode: OutputMode) -> Result<()> {
    let _ = (action, client, mode);
    todo!()
}
