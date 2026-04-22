//! `cargo run -p arcana-gen --bin bakeoff_analyze` — analyzer CLI.
//!
//! Reads a JSONL run file written by the `bakeoff` binary and emits
//! a terminal report (default) or a JSON document wrapped with a
//! `schema_version` field for downstream tooling.
//!
//! ```text
//! cargo run -p arcana-gen --bin bakeoff_analyze --release -- \
//!     target/bakeoff-runs/2026-04-22T181500Z.jsonl \
//!     --top-errors 10 \
//!     --top-hard 20
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{anyhow, Context, Result};
use arcana_gen::analyze::{analyze_file, format_terminal, AnalyzeConfig};
use serde_json::json;

/// Schema version for the JSON output envelope. Bump when the
/// `Analysis` struct gains or changes fields in a way that breaks
/// downstream consumers.
const SCHEMA_VERSION: &str = "1";

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("bakeoff-analyze: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn real_main() -> Result<()> {
    let args = parse_args(std::env::args().skip(1).collect())?;
    let config = AnalyzeConfig {
        top_errors: args.top_errors,
        top_hard: args.top_hard,
    };
    let analysis = analyze_file(&args.path, &config)
        .with_context(|| format!("analyzing {}", args.path.display()))?;

    match args.format {
        Format::Terminal => {
            println!("{}", format_terminal(&analysis));
        }
        Format::Json => {
            let env = json!({
                "schema_version": SCHEMA_VERSION,
                "analysis": analysis,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&env)
                    .context("serializing JSON envelope")?
            );
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Args {
    path: PathBuf,
    format: Format,
    top_errors: usize,
    top_hard: usize,
}

#[derive(Debug, Clone, Copy)]
enum Format {
    Terminal,
    Json,
}

fn parse_args(raw: Vec<String>) -> Result<Args> {
    let mut path: Option<PathBuf> = None;
    let mut format = Format::Terminal;
    let defaults = AnalyzeConfig::default();
    let mut top_errors = defaults.top_errors;
    let mut top_hard = defaults.top_hard;

    let mut it = raw.into_iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--format" => {
                let v = it
                    .next()
                    .ok_or_else(|| anyhow!("--format needs a value"))?;
                format = match v.as_str() {
                    "terminal" => Format::Terminal,
                    "json" => Format::Json,
                    other => return Err(anyhow!("unknown --format '{other}'; use terminal|json")),
                };
            }
            "--top-errors" => {
                top_errors = it
                    .next()
                    .ok_or_else(|| anyhow!("--top-errors needs a value"))?
                    .parse()
                    .context("--top-errors")?;
            }
            "--top-hard" => {
                top_hard = it
                    .next()
                    .ok_or_else(|| anyhow!("--top-hard needs a value"))?
                    .parse()
                    .context("--top-hard")?;
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other if other.starts_with("--") => {
                return Err(anyhow!("unknown argument: {other}"));
            }
            positional => {
                if path.is_some() {
                    return Err(anyhow!(
                        "unexpected second positional argument: {positional}"
                    ));
                }
                path = Some(PathBuf::from(positional));
            }
        }
    }

    let path = path.ok_or_else(|| anyhow!("positional <path.jsonl> is required"))?;
    Ok(Args { path, format, top_errors, top_hard })
}

fn print_usage() {
    eprintln!(
        r#"Usage: bakeoff_analyze <path.jsonl> [options]

Reads a bake-off JSONL run file and prints an analysis: pass-rate
matrix (marginal + cumulative per attempt), error-code histograms
with variant extraction for E0599 / E0433, corpus-hard /
corpus-brittle / model-split cards, latency distributions (all +
passing-only), and cost totals.

Options:
  --format terminal|json   Default: terminal
  --top-errors <n>         Top-N error codes per model (default 10)
  --top-hard <n>           Top-N hard/brittle/split cards (default 20)
  -h, --help               Print this message

JSON output is wrapped as {{"schema_version":"1","analysis":...}} so
downstream consumers can version-gate against future additions.
"#
    );
}
