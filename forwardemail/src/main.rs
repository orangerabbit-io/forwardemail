mod cmd;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "forwardemail", about = "CLI for the Forward Email API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force JSON output
    #[arg(long, global = true)]
    pub json: bool,

    /// API key (overrides config file and env var)
    #[arg(long, global = true)]
    pub api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage account
    Account {
        #[command(subcommand)]
        action: cmd::account::AccountAction,
    },
    /// Manage domains
    Domains {
        #[command(subcommand)]
        action: cmd::domains::DomainsAction,
    },
    /// Manage aliases
    Aliases {
        #[command(subcommand)]
        action: cmd::aliases::AliasesAction,
    },
    /// Manage outbound emails
    Emails {
        #[command(subcommand)]
        action: cmd::emails::EmailsAction,
    },
    /// Download logs
    Logs {
        #[command(subcommand)]
        action: cmd::logs::LogsAction,
    },
    /// Manage domain invites
    Invites {
        #[command(subcommand)]
        action: cmd::invites::InvitesAction,
    },
    /// Manage domain members
    Members {
        #[command(subcommand)]
        action: cmd::members::MembersAction,
    },
    /// Manage catch-all passwords
    #[command(name = "catch-all-passwords")]
    CatchAllPasswords {
        #[command(subcommand)]
        action: cmd::catch_all_passwords::CatchAllPasswordsAction,
    },
    /// Encrypt TXT records
    Encrypt {
        /// Plaintext TXT record to encrypt
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {:#}", e);

        let exit_code = if format!("{:#}", e).contains("No API key found")
            || format!("{:#}", e).contains("Failed to parse config")
            || format!("{:#}", e).contains("HOME environment variable")
        {
            2
        } else {
            1
        };
        process::exit(exit_code);
    }
}

fn run(cli: Cli) -> Result<()> {
    let mode = output::OutputMode::from_json_flag(cli.json);

    match cli.command {
        Commands::Encrypt { input } => cmd::encrypt::run(&input, mode),
        command => {
            // All other commands require authentication
            let config = forwardemail_lib::config::Config::load(cli.api_key.as_deref())?;
            let client = forwardemail_lib::client::Client::new(config.api_key, config.base_url)?;

            match command {
                Commands::Account { action } => cmd::account::run(action, &client, mode),
                Commands::Domains { action } => cmd::domains::run(action, &client, mode),
                Commands::Aliases { action } => cmd::aliases::run(action, &client, mode),
                Commands::Emails { action } => cmd::emails::run(action, &client, mode),
                Commands::Logs { action } => cmd::logs::run(action, &client, mode),
                Commands::Invites { action } => cmd::invites::run(action, &client, mode),
                Commands::Members { action } => cmd::members::run(action, &client, mode),
                Commands::CatchAllPasswords { action } => {
                    cmd::catch_all_passwords::run(action, &client, mode)
                }
                Commands::Encrypt { .. } => unreachable!(),
            }
        }
    }
}
