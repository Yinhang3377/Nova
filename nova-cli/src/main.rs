use anyhow::Result;
use clap::{Parser, Subcommand};
mod simulate;

#[derive(Parser)]
#[command(name = "nova-cli")]
#[command(about = "CLI for Nova", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Wallet {
        action: String,
    },
    Tx {
        to: Option<String>,
        amount: Option<u64>,
    },
    Gov {
        action: String,
    },
    Simulate {
        #[arg(long, action = clap::ArgAction::SetTrue)]
        storm: bool,
        #[arg(long, default_value_t = 5)]
        count: usize,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        json: bool,
        /// storage backend to use. Valid values: "mem" (default) or "none".
        ///
        /// "mem" stores generated blocks in an in-memory backend (used for testing).
        /// "none" disables persistence (backward-compatible behavior).
        #[arg(long, default_value = "mem", value_parser = ["mem", "none"])]
        backend: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Wallet { action } => {
            println!("wallet action: {}", action);
        }
        Commands::Tx { to, amount } => {
            println!("tx to={:?} amount={:?}", to, amount);
        }
        Commands::Gov { action } => {
            println!("gov action: {}", action);
        }
        Commands::Simulate {
            storm,
            count,
            json,
            backend,
        } => {
            simulate::run(count, storm, json, &backend)?;
        }
    }
    Ok(())
}
