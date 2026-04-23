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
//!
//! OpenAI-compatible endpoints via `--openai-model` +
//! `--openai-endpoint` (required pairing). The API key env var name
//! defaults to `OPENAI_API_KEY` and is configurable with
//! `--openai-api-key-env`; if the named var is unset, requests go
//! without an `Authorization` header, which is what local servers
//! (vLLM, llama.cpp, LMStudio, …) want. Pricing is optional via
//! `--openai-pricing IN/OUT` — leave it unset for local inference
//! so `cost_usd` stays `None` in the JSONL.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{anyhow, Context, Result};
use arcana_gen::bakeoff::{self, format_report, BakeoffConfig};
use arcana_gen::classifier::Tier;
use arcana_gen::llm::{
    AnthropicClient, AnthropicPricing, LlmClient, OllamaClient, OpenAiCompatibleClient,
    OpenAiPricing,
};
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
    eprintln!(
        "  openai models:      {}",
        if args.openai_models.is_empty() {
            "(none)".to_string()
        } else {
            format!(
                "{} @ {}",
                args.openai_models.join(", "),
                args.openai_endpoint.as_deref().unwrap_or("<missing>")
            )
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
    if !args.openai_models.is_empty() {
        let endpoint = args.openai_endpoint.as_ref().ok_or_else(|| {
            anyhow!("--openai-endpoint is required when --openai-model is set")
        })?;
        let openai_key = std::env::var(&args.openai_api_key_env).ok();
        if openai_key.is_none() {
            eprintln!(
                "  openai auth:        ${} unset — sending requests without Authorization header",
                args.openai_api_key_env,
            );
        } else {
            eprintln!(
                "  openai auth:        Bearer token from ${}",
                args.openai_api_key_env,
            );
        }
        for m in &args.openai_models {
            let mut client = OpenAiCompatibleClient::new(m, endpoint)
                .with_seed(config.seed);
            if let Some(k) = &openai_key {
                client = client.with_api_key(k);
            }
            if let Some(p) = args.openai_pricing_override {
                eprintln!(
                    "  openai pricing:     {} — ${}/MTok in, ${}/MTok out",
                    m, p.input_per_mtok, p.output_per_mtok,
                );
                client = client.with_pricing(p);
            }
            clients.push(Box::new(client));
        }
    }
    if clients.is_empty() {
        return Err(anyhow!(
            "at least one --model, --anthropic-model, or --openai-model is required"
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
    openai_models: Vec<String>,
    openai_endpoint: Option<String>,
    openai_api_key_env: String,
    openai_pricing_override: Option<OpenAiPricing>,
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
    let mut openai_models: Vec<String> = Vec::new();
    let mut openai_endpoint: Option<String> = None;
    let mut openai_api_key_env: String = "OPENAI_API_KEY".to_string();
    let mut openai_pricing_override: Option<OpenAiPricing> = None;
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
                let (i, o) = parse_pricing(&spec, "--anthropic-pricing")?;
                anthropic_pricing_override =
                    Some(AnthropicPricing { input_per_mtok: i, output_per_mtok: o });
            }
            "--openai-model" => {
                openai_models.push(
                    it.next()
                        .ok_or_else(|| anyhow!("--openai-model needs a value"))?,
                );
            }
            "--openai-endpoint" => {
                openai_endpoint = Some(
                    it.next()
                        .ok_or_else(|| anyhow!("--openai-endpoint needs a value"))?,
                );
            }
            "--openai-api-key-env" => {
                openai_api_key_env = it
                    .next()
                    .ok_or_else(|| anyhow!("--openai-api-key-env needs a value"))?;
            }
            "--openai-pricing" => {
                let spec = it
                    .next()
                    .ok_or_else(|| anyhow!("--openai-pricing needs a value"))?;
                let (i, o) = parse_pricing(&spec, "--openai-pricing")?;
                openai_pricing_override =
                    Some(OpenAiPricing { input_per_mtok: i, output_per_mtok: o });
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

    if models.is_empty() && anthropic_models.is_empty() && openai_models.is_empty() {
        return Err(anyhow!(
            "at least one --model, --anthropic-model, or --openai-model is required"
        ));
    }
    if !openai_models.is_empty() && openai_endpoint.is_none() {
        return Err(anyhow!(
            "--openai-endpoint is required when --openai-model is set"
        ));
    }
    Ok(Args {
        models,
        anthropic_models,
        anthropic_pricing_override,
        openai_models,
        openai_endpoint,
        openai_api_key_env,
        openai_pricing_override,
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

/// Parse an `IN/OUT` pricing spec (USD per million tokens) for any of
/// the `--*-pricing` flags. `flag` is the user-facing flag name for
/// error messages.
fn parse_pricing(spec: &str, flag: &str) -> Result<(f64, f64)> {
    let (lhs, rhs) = spec
        .split_once('/')
        .ok_or_else(|| anyhow!("{flag} must be IN/OUT, got '{spec}'"))?;
    let input_per_mtok: f64 =
        lhs.trim().parse().with_context(|| format!("{flag} input"))?;
    let output_per_mtok: f64 = rhs
        .trim()
        .parse()
        .with_context(|| format!("{flag} output"))?;
    Ok((input_per_mtok, output_per_mtok))
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
        r#"Usage: bakeoff [--model <tag>] [--anthropic-model <id>] [--openai-model <id>] ... [options]

At least one model must be supplied. All three model flags are
repeatable and may be mixed in one run.

Ollama:
  --model <tag>                  Local Ollama tag (e.g. qwen3:235b-instruct-q4_K_M)
  --ollama-endpoint <url>        Default http://localhost:11434

Anthropic:
  --anthropic-model <id>         Messages API model id (e.g. claude-sonnet-4-6)
                                 Requires ANTHROPIC_API_KEY env var.
  --anthropic-pricing IN/OUT     Override per-MTok pricing (USD). Default is a
                                 flagship-tier table keyed on model id.

OpenAI-compatible (OpenAI, Together, Fireworks, Groq, DeepSeek, vLLM,
llama.cpp server, LMStudio, ...):
  --openai-model <id>            Repeatable; all share --openai-endpoint.
  --openai-endpoint <url>        Required. Base URL through /v1 (e.g.
                                 https://api.openai.com/v1, http://m3:8000/v1).
                                 No default — avoids accidental spending.
  --openai-api-key-env <VAR>     Env var holding the Bearer token. Default
                                 OPENAI_API_KEY. If unset, no Authorization
                                 header is sent (appropriate for local servers).
  --openai-pricing IN/OUT        Per-MTok USD. Leave unset for local inference
                                 so cost_usd stays None in the JSONL.

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
