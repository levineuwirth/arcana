//! `cargo run -p arcana-gen --bin bakeoff` — bake-off CLI.
//!
//! Minimal argument parser (no `clap` dep; the flag set is small and
//! stable). Run with:
//!
//! ```text
//! cargo run -p arcana-gen --bin bakeoff --release -- \
//!     --model qwen3:235b-instruct-q4_K_M \
//!     --model glm-4.5:latest \
//!     --sample-size-per-tier 30 \
//!     --max-attempts 3 \
//!     --seed 0
//! ```
//!
//! Ollama models via `--model`. Anthropic models via
//! `--anthropic-model`; the API key is read from the
//! `ANTHROPIC_API_KEY` env var, and pricing is derived from the
//! model id (flagship Claude 4.x table built in) or overridden with
//! `--anthropic-pricing IN/OUT` (USD per million tokens).

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{anyhow, Context, Result};
use arcana_gen::bakeoff::{self, format_report, BakeoffConfig};
use arcana_gen::classifier::Tier;
use arcana_gen::llm::{AnthropicClient, AnthropicPricing, LlmClient, OllamaClient};
use arcana_gen::scryfall::ScryfallPool;

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("bakeoff: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn real_main() -> Result<()> {
    let args = parse_args(std::env::args().skip(1).collect())?;

    let mut config = BakeoffConfig {
        sample_size_per_tier: args.sample_size_per_tier,
        tiers: args.tiers.clone(),
        max_attempts: args.max_attempts,
        seed: args.seed,
        output_path: args.output.clone().unwrap_or_else(default_output_path),
        t4_control_sample: args.t4_control,
        preflight_card_names: vec![
            "Grizzly Bears".to_string(),
            "Lightning Bolt".to_string(),
            "Shock".to_string(),
        ],
    };
    if args.no_preflight {
        config.preflight_card_names.clear();
    }

    // Print header: seed + sample size + model list. Anyone reading
    // output two months from now needs this to reproduce or compare.
    eprintln!("===============================");
    eprintln!("bakeoff run");
    eprintln!("  seed:               {}", config.seed);
    eprintln!("  sample size/tier:   {}", config.sample_size_per_tier);
    eprintln!("  max attempts:       {}", config.max_attempts);
    eprintln!(
        "  tiers:              {}",
        config
            .tiers
            .iter()
            .map(|t| format!("T{}", t.as_number()))
            .collect::<Vec<_>>()
            .join(", ")
    );
    eprintln!("  T4 control sample:  {}", config.t4_control_sample);
    eprintln!(
        "  ollama models:      {}",
        if args.models.is_empty() {
            "(none)".to_string()
        } else {
            args.models.join(", ")
        }
    );
    eprintln!(
        "  anthropic models:   {}",
        if args.anthropic_models.is_empty() {
            "(none)".to_string()
        } else {
            args.anthropic_models.join(", ")
        }
    );
    eprintln!("  output:             {}", config.output_path.display());
    eprintln!("===============================");

    // Build clients.
    let mut clients: Vec<Box<dyn LlmClient>> = Vec::new();
    for m in &args.models {
        clients.push(Box::new(
            OllamaClient::new(m, &args.ollama_endpoint).with_seed(config.seed),
        ));
    }
    if !args.anthropic_models.is_empty() {
        let api_key = std::env::var("ANTHROPIC_API_KEY").context(
            "ANTHROPIC_API_KEY env var is required when --anthropic-model is set",
        )?;
        for m in &args.anthropic_models {
            let pricing = args
                .anthropic_pricing_override
                .unwrap_or_else(|| default_anthropic_pricing(m));
            eprintln!(
                "  anthropic pricing:  {} — ${}/MTok in, ${}/MTok out",
                m, pricing.input_per_mtok, pricing.output_per_mtok,
            );
            clients.push(Box::new(AnthropicClient::new(m, &api_key, pricing)));
        }
    }
    if clients.is_empty() {
        return Err(anyhow!(
            "at least one --model or --anthropic-model is required"
        ));
    }
    let client_refs: Vec<&dyn LlmClient> =
        clients.iter().map(|c| c.as_ref()).collect();

    // Load pool.
    let pool = ScryfallPool::load_default()
        .context("loading Scryfall oracle-cards pool (cache or download)")?;
    eprintln!("bakeoff: loaded {} cards from pool", pool.len());

    // Run.
    let report = bakeoff::run(&pool, &client_refs, &config)?;

    // Print compact table.
    println!("{}", format_report(&report));
    Ok(())
}

#[derive(Debug)]
struct Args {
    models: Vec<String>,
    anthropic_models: Vec<String>,
    anthropic_pricing_override: Option<AnthropicPricing>,
    sample_size_per_tier: usize,
    max_attempts: usize,
    seed: u64,
    tiers: Vec<Tier>,
    t4_control: usize,
    no_preflight: bool,
    ollama_endpoint: String,
    output: Option<PathBuf>,
}

fn parse_args(raw: Vec<String>) -> Result<Args> {
    let mut models = Vec::new();
    let mut anthropic_models: Vec<String> = Vec::new();
    let mut anthropic_pricing_override: Option<AnthropicPricing> = None;
    let mut sample_size_per_tier: usize = 30;
    let mut max_attempts: usize = 3;
    let mut seed: u64 = 0;
    let mut tiers: Vec<Tier> =
        vec![Tier::One, Tier::Two, Tier::Three];
    let mut t4_control: usize = 10;
    let mut no_preflight = false;
    let mut ollama_endpoint: String = "http://localhost:11434".to_string();
    let mut output: Option<PathBuf> = None;

    let mut it = raw.into_iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--model" => {
                models.push(
                    it.next().ok_or_else(|| anyhow!("--model needs a value"))?,
                );
            }
            "--anthropic-model" => {
                anthropic_models.push(
                    it.next()
                        .ok_or_else(|| anyhow!("--anthropic-model needs a value"))?,
                );
            }
            "--anthropic-pricing" => {
                let spec = it
                    .next()
                    .ok_or_else(|| anyhow!("--anthropic-pricing needs a value"))?;
                anthropic_pricing_override = Some(parse_pricing(&spec)?);
            }
            "--sample-size-per-tier" => {
                sample_size_per_tier = it
                    .next()
                    .ok_or_else(|| anyhow!("--sample-size-per-tier needs a value"))?
                    .parse()
                    .context("--sample-size-per-tier")?;
            }
            "--max-attempts" => {
                max_attempts = it
                    .next()
                    .ok_or_else(|| anyhow!("--max-attempts needs a value"))?
                    .parse()
                    .context("--max-attempts")?;
            }
            "--seed" => {
                seed = it
                    .next()
                    .ok_or_else(|| anyhow!("--seed needs a value"))?
                    .parse()
                    .context("--seed")?;
            }
            "--tiers" => {
                let spec = it.next().ok_or_else(|| anyhow!("--tiers needs a value"))?;
                tiers = parse_tier_list(&spec)?;
            }
            "--t4-control" => {
                t4_control = it
                    .next()
                    .ok_or_else(|| anyhow!("--t4-control needs a value"))?
                    .parse()
                    .context("--t4-control")?;
            }
            "--no-preflight" => {
                no_preflight = true;
            }
            "--ollama-endpoint" => {
                ollama_endpoint = it
                    .next()
                    .ok_or_else(|| anyhow!("--ollama-endpoint needs a value"))?;
            }
            "--output" => {
                output = Some(PathBuf::from(
                    it.next().ok_or_else(|| anyhow!("--output needs a value"))?,
                ));
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown argument: {other}")),
        }
    }

    if models.is_empty() && anthropic_models.is_empty() {
        return Err(anyhow!(
            "at least one --model or --anthropic-model is required"
        ));
    }
    Ok(Args {
        models,
        anthropic_models,
        anthropic_pricing_override,
        sample_size_per_tier,
        max_attempts,
        seed,
        tiers,
        t4_control,
        no_preflight,
        ollama_endpoint,
        output,
    })
}

/// Parse `--anthropic-pricing IN/OUT` (USD per million tokens).
fn parse_pricing(spec: &str) -> Result<AnthropicPricing> {
    let (lhs, rhs) = spec
        .split_once('/')
        .ok_or_else(|| anyhow!("--anthropic-pricing must be IN/OUT, got '{spec}'"))?;
    let input_per_mtok: f64 =
        lhs.trim().parse().context("--anthropic-pricing input")?;
    let output_per_mtok: f64 =
        rhs.trim().parse().context("--anthropic-pricing output")?;
    Ok(AnthropicPricing { input_per_mtok, output_per_mtok })
}

/// Conservative default pricing table for flagship Claude 4.x models.
/// Unknown model ids fall back to Opus pricing so cost estimates
/// overshoot rather than understate. Override with
/// `--anthropic-pricing IN/OUT` for precise numbers.
fn default_anthropic_pricing(model_id: &str) -> AnthropicPricing {
    if model_id.contains("opus") {
        AnthropicPricing { input_per_mtok: 15.0, output_per_mtok: 75.0 }
    } else if model_id.contains("sonnet") {
        AnthropicPricing { input_per_mtok: 3.0, output_per_mtok: 15.0 }
    } else if model_id.contains("haiku") {
        AnthropicPricing { input_per_mtok: 1.0, output_per_mtok: 5.0 }
    } else {
        AnthropicPricing { input_per_mtok: 15.0, output_per_mtok: 75.0 }
    }
}

fn parse_tier_list(spec: &str) -> Result<Vec<Tier>> {
    let mut out = Vec::new();
    for part in spec.split(',') {
        let n: u8 = part.trim().parse().with_context(|| format!("tier '{part}'"))?;
        let tier = match n {
            1 => Tier::One,
            2 => Tier::Two,
            3 => Tier::Three,
            4 => Tier::Four,
            5 => Tier::Five,
            _ => return Err(anyhow!("tier must be 1..=5, got {n}")),
        };
        out.push(tier);
    }
    Ok(out)
}

fn default_output_path() -> PathBuf {
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H%M%SZ");
    // Workspace root from arcana-gen/bin: up two levels.
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
        .join("..")
        .join("target")
        .join("bakeoff-runs")
        .join(format!("{timestamp}.jsonl"))
}

fn print_usage() {
    eprintln!(
        r#"Usage: bakeoff [--model <tag>] [--anthropic-model <id>] ... [options]

At least one model must be supplied. --model and --anthropic-model
are repeatable and may be mixed.

Ollama:
  --model <tag>                  Local Ollama tag (e.g. qwen3:235b-instruct-q4_K_M)
  --ollama-endpoint <url>        Default http://localhost:11434

Anthropic:
  --anthropic-model <id>         Messages API model id (e.g. claude-sonnet-4-6)
                                 Requires ANTHROPIC_API_KEY env var.
  --anthropic-pricing IN/OUT     Override per-MTok pricing (USD). Default is a
                                 flagship-tier table keyed on model id.

Sweep:
  --sample-size-per-tier <n>     Default 30
  --max-attempts <n>             Default 3 (set to 1 for one-shot-only)
  --seed <u64>                   Default 0
  --tiers <list>                 Comma-separated tier numbers; default 1,2,3
  --t4-control <n>               T4 sanity-anchor sample size; default 10
  --no-preflight                 Skip the 3-card pre-sweep smoke test
  --output <path>                Default target/bakeoff-runs/<timestamp>.jsonl

Outputs a compact summary to stdout and detailed per-(card, model)
JSONL to the output path. JSONL rows are flushed per-line for
crash-robustness.
"#
    );
}
