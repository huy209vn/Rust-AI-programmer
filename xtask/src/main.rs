use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use schemars::{schema_for, JsonSchema};
use serde::de::DeserializeOwned;
use serde_json::{to_writer_pretty, Value};
use std::fs::{create_dir_all, File};
use std::io::{BufRead, Read};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    Schema,
    Validate { path: PathBuf },
    Conformance,
    Roundtrip,
    SchemaGuard,
    Report,
    Badge,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Schema => schema()?,
        Cmd::Validate { path } => validate(path)?,
        Cmd::Conformance => conformance()?,
        Cmd::Roundtrip => roundtrip()?,
        Cmd::SchemaGuard => schema_guard()?,
        Cmd::Report => report()?,
        Cmd::Badge => badge()?,
    }
    Ok(())
}

fn schema() -> Result<()> {
    create_dir_all("schemas")?;
    write_schema::<protocol::rpc::ToolRequest>("schemas/protocol_request.json")?;
    write_schema::<traceserver::trace::TraceEnvelope>("schemas/trace_envelope.json")?;
    write_schema::<traceserver::patch::PatchDoc>("schemas/patch_doc.json")?;
    write_schema::<traceserver::memory::Manifest>("schemas/memory_manifest.json")?;
    Ok(())
}

fn write_schema<T: JsonSchema>(path: &str) -> Result<()> {
    let schema = schema_for!(T);
    let file = File::create(path)?;
    to_writer_pretty(file, &schema)?;
    println!("wrote {path}");
    Ok(())
}

fn read_jsonl<T: DeserializeOwned, R: std::io::Read>(rdr: R) -> Result<Vec<T>> {
    let reader = std::io::BufReader::new(rdr);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        let v: T = serde_json::from_str(&line)?;
        out.push(v);
    }
    Ok(out)
}

fn validate(path: PathBuf) -> Result<()> {
    let file = File::open(&path)?;
    let envs: Vec<traceserver::trace::TraceEnvelope> = read_jsonl(file)?;
    for env in &envs { traceserver::validator::validate(env)?; }
    println!("validated {} trace lines", envs.len());
    Ok(())
}

fn conformance() -> Result<()> {
    schema()?;
    let status = Command::new("git").args(["diff", "--exit-code"]).status()?;
    if !status.success() { return Err(anyhow!("schema drift detected — commit updated schemas")); }
    println!("conformance ok (no schema drift)");
    Ok(())
}

fn roundtrip() -> Result<()> {
    let mut schema_str = String::new();
    File::open("schemas/trace_envelope.json")?.read_to_string(&mut schema_str)?;
    let schema_json: Value = serde_json::from_str(&schema_str)?;
    let compiled = jsonschema::JSONSchema::options().with_draft(jsonschema::Draft::Draft7).compile(&schema_json)?;
    let file = File::open("crates/traceserver/examples/sample_trace.jsonl")?;
    let vals: Vec<Value> = read_jsonl(file)?;
    for (i, v) in vals.iter().enumerate() {
        if let Err(errors) = compiled.validate(v) {
            let msgs: Vec<String> = errors.map(|e| e.to_string()).collect();
            return Err(anyhow!(format!("roundtrip schema validation failed on line {}: {}", i + 1, msgs.join(" | "))));
        }
    }
    println!("roundtrip ok: {} lines validated against schema", vals.len());
    Ok(())
}

fn schema_guard() -> Result<()> {
    for p in [
        "schemas/protocol_request.json",
        "schemas/trace_envelope.json",
        "schemas/patch_doc.json",
        "schemas/memory_manifest.json",
    ] {
        let v: Value = serde_json::from_reader(File::open(p)?)?;
        let _compiled = jsonschema::JSONSchema::options().with_draft(jsonschema::Draft::Draft7).compile(&v)?;
        println!("schema_guard ok: {p}");
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ConformanceJson { schema_drift: bool, roundtrip_lines: usize, schemas_ok: bool }

fn report() -> Result<()> {
    let drift_ok = Command::new("git").args(["diff", "--exit-code"]).status()?.success();
    let file = File::open("crates/traceserver/examples/sample_trace.jsonl").ok();
    let lines = if let Some(f) = file { read_jsonl::<Value,_>(f)?.len() } else { 0 };
    let schemas_ok = schema_guard().map(|_| true).unwrap_or(false);
    create_dir_all("schemas")?;
    let out = File::create("schemas/conformance.json")?;
    serde_json::to_writer_pretty(out, &ConformanceJson { schema_drift: drift_ok, roundtrip_lines: lines, schemas_ok })?;
    println!("wrote schemas/conformance.json");
    Ok(())
}

#[derive(serde::Serialize)]
struct ShieldsBadge<'a> { schemaVersion: u8, label: &'a str, message: String, color: &'a str }

fn badge() -> Result<()> {
    let conf: ConformanceJson = serde_json::from_reader(File::open("schemas/conformance.json")?)?;
    let passing = conf.schema_drift && conf.schemas_ok && conf.roundtrip_lines > 0;
    let badge = ShieldsBadge { schemaVersion: 1, label: "phase-0", message: if passing { "passing".into() } else { "failing".into() }, color: if passing { "brightgreen" } else { "red" } };
    let out = File::create("schemas/badge.json")?;
    serde_json::to_writer_pretty(out, &badge)?;
    println!("wrote schemas/badge.json");
    Ok(())
}
