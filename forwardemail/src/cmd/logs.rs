use anyhow::{Context, Result};
use clap::Subcommand;
use flate2::read::GzDecoder;
use forwardemail_lib::client::Client;
use std::io::Read;

use crate::output::{self, OutputMode};

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

pub fn run(action: LogsAction, client: &Client, _mode: OutputMode) -> Result<()> {
    match action {
        LogsAction::Download {
            domain,
            q,
            bounce_category,
            response_code,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = &domain {
                params.push(("domain", v.clone()));
            }
            if let Some(v) = &q {
                params.push(("q", v.clone()));
            }
            if let Some(v) = &bounce_category {
                params.push(("bounce_category", v.clone()));
            }
            if let Some(v) = &response_code {
                params.push(("response_code", v.clone()));
            }
            let param_refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let bytes = client.get_bytes("/v1/logs/download", &param_refs)?;
            let text = if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
                let mut decoder = GzDecoder::new(&bytes[..]);
                let mut s = String::new();
                decoder
                    .read_to_string(&mut s)
                    .context("Failed to decompress gzipped log data")?;
                s
            } else {
                String::from_utf8(bytes).context("Log response is not valid UTF-8")?
            };
            output::print_raw(&text);
            Ok(())
        }
    }
}
