use anyhow::*;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
struct Xtask {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Setup,
    BuildAll,
    EvalMini,
    TraceValidate,
    RunDaemon,
}

fn main() -> Result<()> {
    let xt = Xtask::parse();
    match xt.cmd {
        Cmd::Setup => {
            println!("(skeleton) setup: ensure toolchain & components present");
        }
        Cmd::BuildAll => run("cargo", &["build", "--workspace"])?,
        Cmd::EvalMini => println!("(skeleton) run eval core on fixtures"),
        Cmd::TraceValidate => println!("(skeleton) validate traces"),
        Cmd::RunDaemon => run("cargo", &["run", "-p", "ai-rust-programmer-daemon"])?,
    }
    Ok(())
}

fn run(bin: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(bin).args(args).status()?;
    ensure!(status.success(), "command failed: {bin} {}", args.join(" "));
    Ok(())
}
