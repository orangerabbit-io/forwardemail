use anyhow::Result;
use clap::Subcommand;
use forwardemail_lib::client::Client;

use crate::output::OutputMode;

#[derive(Subcommand)]
pub enum LogsAction {
    /// Download logs as CSV
    Download {
        #[arg(long)]
        domain: Option<String>,
        #[arg(short, long)]
        q: Option<String>,
        #[arg(long)]
        bounce_category: Option<String>,
        #[arg(long)]
        response_code: Option<String>,
    },
}

pub fn run(action: LogsAction, client: &Client, mode: OutputMode) -> Result<()> {
    let _ = (action, client, mode);
    todo!()
}
