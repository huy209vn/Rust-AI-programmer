#![deny(warnings)]
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask", about = "Workspace tasks (doctor, train, eval, data)")]
struct XTask {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Environment checks (toolchain, paths, datasets)
    Doctor,
    /// Snapshot data & validate ledgers
    Data {
        #[command(subcommand)]
        cmd: DataCmd,
    },
    /// Training entrypoints
    Train {
        /// Stage: S0|S1|S2|S3
        #[arg(long)]
        stage: String,
        /// Model size: tiny|base
        #[arg(long, default_value="tiny")]
        model: String,
    },
    /// Evaluation harness (slices, canary)
    Eval {
        #[command(subcommand)]
        cmd: EvalCmd,
    },
    /// Golden entries replay
    Golden {
        #[command(subcommand)]
        cmd: GoldenCmd,
    },
    /// Validate DevLog entries against schemas
    Entries {
        #[command(subcommand)]
        cmd: EntriesCmd,
    },
}

#[derive(Subcommand)]
enum DataCmd {
    Snapshot,
}

#[derive(Subcommand)]
enum EvalCmd {
    Slices,
    Canary,
    Smoke,
}

#[derive(Subcommand)]
enum GoldenCmd {
    Replay,
}

#[derive(Subcommand)]
enum EntriesCmd {
    Validate,
}

fn main() -> Result<()> {
    let args = XTask::parse();
    match args.cmd {
        Cmd::Doctor => doctor()?,
        Cmd::Data { cmd } => match cmd {
            DataCmd::Snapshot => println!("[xtask] data:snapshot — TODO (hash & ledger validate)"),
        },
        Cmd::Train { stage, model } => {
            println!("[xtask] train:{} --model {} — stub", stage, model);
        }
        Cmd::Eval { cmd } => match cmd {
            EvalCmd::Slices => println!("[xtask] eval:slices — TODO (taxonomy dashboard)"),
            EvalCmd::Canary => println!("[xtask] eval:canary — TODO"),
            EvalCmd::Smoke => println!("[xtask] eval:smoke — TODO"),
        },
        Cmd::Golden { cmd } => match cmd {
            GoldenCmd::Replay => println!("[xtask] golden:replay — TODO (determinism K±2)"),
        },
        Cmd::Entries { cmd } => match cmd {
            EntriesCmd::Validate => println!("[xtask] entries:validate — TODO (schema checks)"),
        },
    }
    Ok(())
}

fn doctor() -> Result<()> {
    println!("[xtask] doctor — checking workspace (stub)");
    println!(" - rust toolchain: stable (expected)");
    println!(" - components: rustfmt, clippy (expected)");
    println!(" - schemas present: schemas/*.schema.json");
    println!(" - datasets ledger: datasets/ledger/*.yml");
    println!("OK (stubs) — fill in checks as you implement.");
    Ok(())
}
