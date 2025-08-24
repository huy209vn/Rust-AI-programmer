use anyhow::Result;
use clap::{Parser, Subcommand};

/// Ops-only CLI. No chat.
#[derive(Parser)]
#[command(name = "arp", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Index/update local repo (placeholder)
    Index { path: String },
    /// Run mini eval suite (placeholder)
    EvalMini,
    /// Validate traces (placeholder)
    TraceValidate { path: String },
    /// Start daemon (convenience)
    Daemon,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Index { path } => {
            println!("Indexing repo at {path} (placeholder)");
        }
        Command::EvalMini => {
            println!("Running mini eval (placeholder)");
        }
        Command::TraceValidate { path } => {
            println!("Validating traces in {path} (placeholder)");
        }
        Command::Daemon => {
            println!("Starting daemon (placeholder). Use `ai-rust-programmer-daemon` directly.");
        }
    }
    Ok(())
}
