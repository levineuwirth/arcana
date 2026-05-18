//! Bake-off driver: sample cards, call models, verify, record.
//!
//! Given a [`ScryfallPool`] and a list of [`LlmClient`]s, walks a
//! seeded sample of cards through the full pipeline — render
//! prompt → call model → verify → retry on failure — recording
//! every attempt to a JSONL file and returning a [`BakeoffReport`]
//! with per-model aggregates.
//!
//! # Design choices
//!
//! * **Retry-with-errors is default, not opt-in.** Default
//!   `max_attempts = 3`. One-shot-only measurement systematically
//!   underreports frontier models' error-recovery advantage; the
//!   full-pipeline number is the one that informs the Phase-3
//!   model decision. Set `max_attempts = 1` for pure one-shot
//!   studies.
//!
//! * **Unsupported cards bucket at the report level, not per-model.**
//!   `render_prompt` returning [`Unsupported`] is a property of the
//!   (card, classifier, prompt-coverage) triple, identical for
//!   every model in a run. Reporting it under a model would conflate
//!   pipeline gaps with model capability. The denominator for each
//!   `ModelSummary` excludes Unsupported cards.
//!
//! * **JSONL streams one row per (card, model) pair, flushed
//!   immediately.** A run that crashes at card 60 of 120 still has
//!   59 rows of data. `target/bakeoff-runs/` is gitignored.
//!
//! * **Precheck + preflight before the full sweep.** Precheck
//!   (Grizzly Bears through the verify pipeline) catches "arcana-
//!   cards or verify is broken" before the first real card.
//!   Preflight (a handful of known-easy cards against each model)
//!   catches "this model's endpoint is unreachable or misbehaving"
//!   before 90 minutes of real work.
//!
//! * **Seeded sampling with [`ChaCha8Rng`].** Same seed + same pool
//!   = same sample. Re-runs are reproducible. Sampled card names
//!   are logged to stderr so "what actually ran?" is always
//!   recoverable.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::classifier::{classify, Tier};
use crate::llm::LlmClient;
use crate::prompt::{
    render_prompt, render_retry_prompt, PreviousAttempt, PromptShape,
};
use crate::scryfall::{Card, ScryfallPool};
use crate::verify::{check, precheck, CompileError, VerifyConfig, VerifyResult};

// =============================================================================
// Config
// =============================================================================

#[derive(Debug, Clone)]
pub struct BakeoffConfig {
    pub sample_size_per_tier: usize,
    pub tiers: Vec<Tier>,
    /// Max generation attempts per (card, model). 3 is the default
    /// to match typical production retry budgets. Set to 1 for
    /// pure one-shot comparisons.
    pub max_attempts: usize,
    /// Seed for the card sampler. Determines which cards land in
    /// the sample. Hold this constant across shards / replicate
    /// jobs so every shard sees the same card set.
    pub card_seed: u64,
    /// Seed forwarded to model clients (Ollama / OpenAI-compatible).
    /// Vary this across shards / replicate jobs to get independent
    /// trials of the same cards. Anthropic ignores it (the
    /// Messages API doesn't accept a seed).
    pub model_seed: u64,
    pub output_path: PathBuf,
    /// Number of T4 cards sampled as a classifier sanity anchor.
    /// Expected to all route to Unsupported; a surprise passes-
    /// through means the classifier's T4 gate over- or under-catches.
    pub t4_control_sample: usize,
    /// Restrict sampling to Standard-legal cards. Default `true`
    /// (the bake-off measures current-format generation). Set
    /// `false` (`--all-sets`) to sample the whole oracle pool —
    /// needed for large runs of a structurally-safe class (vanilla
    /// / french-vanilla creatures) where Standard alone is too thin.
    pub restrict_standard: bool,
    /// Preflight: a small handful of known-easy cards each model
    /// is asked to generate before the full sweep begins. Catches
    /// "endpoint unreachable" or "model hangs" before hours of
    /// work sink into it. Empty to skip.
    pub preflight_card_names: Vec<String>,
}

impl Default for BakeoffConfig {
    fn default() -> Self {
        Self {
            sample_size_per_tier: 30,
            tiers: vec![Tier::One, Tier::Two, Tier::Three],
            max_attempts: 3,
            card_seed: 0,
            model_seed: 0,
            output_path: default_output_path(),
            t4_control_sample: 10,
            restrict_standard: true,
            preflight_card_names: vec![
                "Grizzly Bears".to_string(),
                "Lightning Bolt".to_string(),
                "Shock".to_string(),
            ],
        }
    }
}

fn default_output_path() -> PathBuf {
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H%M%SZ");
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
        .join("..")
        .join("target")
        .join("bakeoff-runs")
        .join(format!("{timestamp}.jsonl"))
}

// =============================================================================
// Outcome + report types
// =============================================================================

/// Final state of a (card, model) run after all attempts complete.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FinalOutcome {
    /// Candidate passed verify on attempt `at_attempt` (0-indexed).
    Passed { at_attempt: usize },
    /// Every attempt failed verify in the candidate itself.
    FailedAllAttempts,
    /// Structural abort: model HTTP call errored, verify flagged
    /// `FailedElsewhere` or `InfrastructureError`. These are not
    /// model-quality signals.
    Aborted { reason: String },
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TierSummary {
    /// Index = attempt number (0 = first try). A card that passed
    /// on retry #2 increments `passed_at_attempt[2]`.
    pub passed_at_attempt: Vec<usize>,
    pub failed_all_attempts: usize,
    pub aborted: usize,
    /// Denominator: how many cards the model was actually asked
    /// to generate. Excludes Unsupported (those never reach the
    /// model).
    pub attempted: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ModelSummary {
    pub per_tier: HashMap<Tier, TierSummary>,
    /// Flattened across tiers; same semantics as TierSummary.
    pub passed_at_attempt: Vec<usize>,
    pub failed_all_attempts: usize,
    pub aborted: usize,
    pub attempted: usize,
    pub mean_completion_duration: Duration,
    pub mean_verify_duration: Duration,
    /// Sum of `Completion.cost_usd` across all attempts. `None` if
    /// no attempt had a cost (local-only models).
    pub total_cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BakeoffReport {
    pub total_cards_sampled: usize,
    /// Seed used for card sampling.
    pub card_seed: u64,
    /// Seed used for model RNG. Different from `card_seed` lets
    /// you run replicate trials of the same card set with varied
    /// model behavior (cluster-shard pattern).
    pub model_seed: u64,
    /// Unsupported reasons keyed by their `Display` string, with
    /// counts. Identical across all models in a run — surfaces
    /// pipeline-coverage failures distinctly from model failures.
    pub unsupported: HashMap<String, usize>,
    pub per_model: HashMap<String, ModelSummary>,
    pub output_path: PathBuf,
}

// =============================================================================
// JSONL row shapes
// =============================================================================

#[derive(Serialize)]
struct UnsupportedRow<'a> {
    kind: &'static str,
    timestamp: String,
    card: CardMeta<'a>,
    tier: u8,
    reason: String,
}

#[derive(Serialize)]
struct AttemptedRow<'a> {
    kind: &'static str,
    timestamp: String,
    card: CardMeta<'a>,
    tier: u8,
    shape: &'a str,
    model: &'a str,
    prompt_render_duration_ms: u128,
    attempts: Vec<AttemptRow>,
    final_outcome: FinalOutcome,
}

#[derive(Serialize)]
struct CardMeta<'a> {
    name: &'a str,
    oracle_id: &'a str,
    set: &'a str,
}

#[derive(Serialize)]
struct AttemptRow {
    completion: CompletionRow,
    verify: VerifyRow,
}

#[derive(Serialize, Clone)]
struct CompletionRow {
    text: String,
    duration_ms: u128,
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    cost_usd: Option<f64>,
}

#[derive(Serialize, Clone)]
struct VerifyRow {
    result: VerifyResultRow,
    duration_ms: u128,
    error_count: usize,
}

#[derive(Serialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum VerifyResultRow {
    Passed,
    FailedInCandidate { errors: Vec<CompileError> },
    FailedElsewhere { errors: Vec<CompileError> },
    InfrastructureError { message: String },
}

impl From<&VerifyResult> for VerifyResultRow {
    fn from(v: &VerifyResult) -> Self {
        match v {
            VerifyResult::Passed => VerifyResultRow::Passed,
            VerifyResult::FailedInCandidate(errors) => {
                VerifyResultRow::FailedInCandidate { errors: errors.clone() }
            }
            VerifyResult::FailedElsewhere(errors) => {
                VerifyResultRow::FailedElsewhere { errors: errors.clone() }
            }
            VerifyResult::InfrastructureError(msg) => {
                VerifyResultRow::InfrastructureError { message: msg.clone() }
            }
        }
    }
}

// =============================================================================
// Driver
// =============================================================================

/// Run the bake-off end-to-end. Precheck + preflight + sweep +
/// JSONL emission + aggregated report. See module docs for
/// design-choice rationale.
pub fn run(
    pool: &ScryfallPool,
    models: &[&dyn LlmClient],
    config: &BakeoffConfig,
) -> Result<BakeoffReport> {
    if models.is_empty() {
        return Err(anyhow!("bakeoff needs at least one model"));
    }
    if config.max_attempts == 0 {
        return Err(anyhow!("max_attempts must be >= 1"));
    }

    // 1. Precheck
    eprintln!("bakeoff: running precheck (Grizzly Bears canary)…");
    let pre = precheck(&VerifyConfig::default());
    match &pre.result {
        VerifyResult::Passed => {
            eprintln!("  precheck passed ({}ms)", pre.duration.as_millis());
        }
        other => {
            return Err(anyhow!(
                "precheck did not pass — pipeline is broken, bake-off results would be invalid: {other:?}"
            ));
        }
    }

    // 2. Preflight per-model
    if !config.preflight_card_names.is_empty() {
        eprintln!(
            "bakeoff: preflight {} cards × {} models",
            config.preflight_card_names.len(),
            models.len()
        );
        for name in &config.preflight_card_names {
            let Some(card) = pool.find_by_name(name) else {
                eprintln!("  warn: preflight card '{name}' not in pool, skipping");
                continue;
            };
            let tier = classify(card).tier;
            for model in models {
                let outcome = run_card_for_model(card, tier, *model, config, 1);
                eprintln!(
                    "  preflight {}/{}: {:?}",
                    model.model_id(),
                    card.name,
                    outcome.final_outcome,
                );
            }
        }
    }

    // 3. Sample
    let sampled = sample_cards(pool, config);
    eprintln!(
        "bakeoff: sampled {} cards (card_seed={}, model_seed={}, size_per_tier={}, t4_control={})",
        sampled.len(),
        config.card_seed,
        config.model_seed,
        config.sample_size_per_tier,
        config.t4_control_sample,
    );
    for (card, tier) in &sampled {
        eprintln!("  T{} {}", tier.as_number(), card.name);
    }

    // 4. Open JSONL
    if let Some(parent) = config.output_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut jsonl = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.output_path)
        .with_context(|| format!("opening {}", config.output_path.display()))?;
    eprintln!(
        "bakeoff: writing JSONL to {}",
        config.output_path.display()
    );

    // 5. Main sweep
    let mut unsupported: HashMap<String, usize> = HashMap::new();
    let mut per_model: HashMap<String, ModelAccum> = HashMap::new();
    for model in models {
        per_model.insert(model.model_id().to_string(), ModelAccum::new(config.max_attempts));
    }

    for (card, tier) in &sampled {
        // Resolve prompt once per card. Unsupported result is the
        // same across models — record at card scope and skip the
        // model loop.
        match render_prompt(card, *tier) {
            Err(u) => {
                let reason = u.to_string();
                *unsupported.entry(reason.clone()).or_insert(0) += 1;
                write_jsonl_row(
                    &mut jsonl,
                    &UnsupportedRow {
                        kind: "unsupported",
                        timestamp: iso8601_now(),
                        card: CardMeta {
                            name: &card.name,
                            oracle_id: &card.oracle_id,
                            set: &card.set,
                        },
                        tier: tier.as_number(),
                        reason,
                    },
                )?;
                continue;
            }
            Ok(_) => {}
        }

        for model in models {
            let outcome = run_card_for_model(card, *tier, *model, config, config.max_attempts);
            let acc = per_model
                .get_mut(model.model_id())
                .expect("accumulator was seeded above");
            acc.record(*tier, &outcome);
            write_jsonl_row(&mut jsonl, &outcome.row(card, *tier, model.model_id()))?;
        }
    }

    jsonl.flush().context("final JSONL flush")?;

    // 6. Finalize report
    let mut per_model_out: HashMap<String, ModelSummary> = HashMap::new();
    for (id, acc) in per_model {
        per_model_out.insert(id, acc.finalize(config.max_attempts));
    }
    Ok(BakeoffReport {
        total_cards_sampled: sampled.len(),
        card_seed: config.card_seed,
        model_seed: config.model_seed,
        unsupported,
        per_model: per_model_out,
        output_path: config.output_path.clone(),
    })
}

// =============================================================================
// Per-(card, model) attempt loop
// =============================================================================

struct CardRunOutcome {
    shape: PromptShape,
    prompt_render_duration: Duration,
    attempts: Vec<(CompletionRow, VerifyRow, Duration /*completion*/, Duration /*verify*/, Option<f64> /*cost*/)>,
    final_outcome: FinalOutcome,
}

impl CardRunOutcome {
    fn row<'a>(
        &'a self,
        card: &'a Card,
        tier: Tier,
        model_id: &'a str,
    ) -> AttemptedRow<'a> {
        AttemptedRow {
            kind: "attempted",
            timestamp: iso8601_now(),
            card: CardMeta {
                name: &card.name,
                oracle_id: &card.oracle_id,
                set: &card.set,
            },
            tier: tier.as_number(),
            shape: prompt_shape_name(self.shape),
            model: model_id,
            prompt_render_duration_ms: self.prompt_render_duration.as_millis(),
            attempts: self
                .attempts
                .iter()
                .map(|(c, v, _, _, _)| AttemptRow {
                    completion: c.clone(),
                    verify: v.clone(),
                })
                .collect(),
            final_outcome: self.final_outcome.clone(),
        }
    }
}

fn run_card_for_model(
    card: &Card,
    tier: Tier,
    model: &dyn LlmClient,
    _config: &BakeoffConfig,
    max_attempts: usize,
) -> CardRunOutcome {
    let render_start = Instant::now();
    let initial = match render_prompt(card, tier) {
        Ok(p) => p,
        Err(u) => {
            // Shouldn't hit this — caller filtered Unsupported
            // already. If it does, surface as an abort.
            return CardRunOutcome {
                shape: PromptShape::VanillaCreature, // placeholder
                prompt_render_duration: render_start.elapsed(),
                attempts: vec![],
                final_outcome: FinalOutcome::Aborted {
                    reason: format!("unexpected Unsupported inside attempt loop: {u}"),
                },
            };
        }
    };
    let shape = initial.shape;
    let prompt_render_duration = render_start.elapsed();

    let mut attempts = Vec::new();
    let mut previous_code: Option<String> = None;
    let mut previous_errors: Vec<CompileError> = Vec::new();

    for attempt_idx in 0..max_attempts {
        // Render the prompt for this attempt: one-shot on first,
        // retry-with-errors on subsequent.
        let prompt = if attempt_idx == 0 {
            initial.clone()
        } else {
            let code = previous_code.as_deref().unwrap_or("");
            match render_retry_prompt(
                card,
                tier,
                &PreviousAttempt {
                    code,
                    errors: &previous_errors,
                },
            ) {
                Ok(p) => p,
                Err(u) => {
                    return CardRunOutcome {
                        shape,
                        prompt_render_duration,
                        attempts,
                        final_outcome: FinalOutcome::Aborted {
                            reason: format!("retry prompt Unsupported: {u}"),
                        },
                    };
                }
            }
        };

        // Call the model.
        let completion = match model.complete(&prompt.system, &prompt.user) {
            Ok(c) => c,
            Err(e) => {
                return CardRunOutcome {
                    shape,
                    prompt_render_duration,
                    attempts,
                    final_outcome: FinalOutcome::Aborted {
                        reason: format!("model call failed: {e:#}"),
                    },
                };
            }
        };

        // Verify the candidate.
        let verify_report = check(&completion.text, &VerifyConfig::default());

        let completion_row = CompletionRow {
            text: completion.text.clone(),
            duration_ms: completion.duration.as_millis(),
            prompt_tokens: completion.prompt_tokens,
            completion_tokens: completion.completion_tokens,
            cost_usd: completion.cost_usd,
        };
        let error_count = match &verify_report.result {
            VerifyResult::Passed => 0,
            VerifyResult::FailedInCandidate(e) | VerifyResult::FailedElsewhere(e) => e.len(),
            VerifyResult::InfrastructureError(_) => 0,
        };
        let verify_row = VerifyRow {
            result: VerifyResultRow::from(&verify_report.result),
            duration_ms: verify_report.duration.as_millis(),
            error_count,
        };

        attempts.push((
            completion_row,
            verify_row,
            completion.duration,
            verify_report.duration,
            completion.cost_usd,
        ));

        match &verify_report.result {
            VerifyResult::Passed => {
                return CardRunOutcome {
                    shape,
                    prompt_render_duration,
                    attempts,
                    final_outcome: FinalOutcome::Passed { at_attempt: attempt_idx },
                };
            }
            VerifyResult::FailedInCandidate(errors) => {
                // Set up retry context and loop.
                previous_code = Some(completion.text);
                previous_errors = errors.clone();
            }
            VerifyResult::FailedElsewhere(_) => {
                return CardRunOutcome {
                    shape,
                    prompt_render_duration,
                    attempts,
                    final_outcome: FinalOutcome::Aborted {
                        reason: "verify FailedElsewhere — arcana-cards broken mid-run".into(),
                    },
                };
            }
            VerifyResult::InfrastructureError(msg) => {
                return CardRunOutcome {
                    shape,
                    prompt_render_duration,
                    attempts,
                    final_outcome: FinalOutcome::Aborted {
                        reason: format!("verify InfrastructureError: {msg}"),
                    },
                };
            }
        }
    }

    CardRunOutcome {
        shape,
        prompt_render_duration,
        attempts,
        final_outcome: FinalOutcome::FailedAllAttempts,
    }
}

// =============================================================================
// Sampling
// =============================================================================

pub fn sample_cards(pool: &ScryfallPool, config: &BakeoffConfig) -> Vec<(Card, Tier)> {
    let mut rng = ChaCha8Rng::seed_from_u64(config.card_seed);

    // Bucket cards by classifier tier. Standard-legal only by
    // default; whole pool when `restrict_standard` is false.
    let mut by_tier: HashMap<Tier, Vec<&Card>> = HashMap::new();
    let pool_iter: Box<dyn Iterator<Item = &Card>> = if config.restrict_standard {
        Box::new(pool.standard_legal())
    } else {
        Box::new(pool.iter())
    };
    for card in pool_iter {
        let tier = classify(card).tier;
        by_tier.entry(tier).or_default().push(card);
    }

    // Sort each bucket by oracle_id so the pre-shuffle state is
    // deterministic across runs with the same pool (pool iteration
    // order is insertion-ordered but verifying that is cheap).
    for bucket in by_tier.values_mut() {
        bucket.sort_by(|a, b| a.oracle_id.cmp(&b.oracle_id));
    }

    let mut out: Vec<(Card, Tier)> = Vec::new();
    for &tier in &config.tiers {
        if let Some(bucket) = by_tier.get(&tier) {
            let mut shuffled = bucket.clone();
            shuffled.shuffle(&mut rng);
            for card in shuffled.iter().take(config.sample_size_per_tier) {
                out.push(((*card).clone(), tier));
            }
        }
    }
    // T4 control sample (routes through the same pipeline and
    // should all end up Unsupported — signals classifier health).
    if config.t4_control_sample > 0 {
        if let Some(bucket) = by_tier.get(&Tier::Four) {
            let mut shuffled = bucket.clone();
            shuffled.shuffle(&mut rng);
            for card in shuffled.iter().take(config.t4_control_sample) {
                out.push(((*card).clone(), Tier::Four));
            }
        }
    }
    out
}

// =============================================================================
// Aggregation
// =============================================================================

struct ModelAccum {
    max_attempts: usize,
    passed_at_attempt: Vec<usize>,
    failed_all_attempts: usize,
    aborted: usize,
    attempted: usize,
    per_tier: HashMap<Tier, TierSummary>,
    completion_durations: Vec<Duration>,
    verify_durations: Vec<Duration>,
    total_cost_usd: Option<f64>,
}

impl ModelAccum {
    fn new(max_attempts: usize) -> Self {
        Self {
            max_attempts,
            passed_at_attempt: vec![0; max_attempts],
            failed_all_attempts: 0,
            aborted: 0,
            attempted: 0,
            per_tier: HashMap::new(),
            completion_durations: Vec::new(),
            verify_durations: Vec::new(),
            total_cost_usd: None,
        }
    }

    fn record(&mut self, tier: Tier, outcome: &CardRunOutcome) {
        self.attempted += 1;
        let tier_summary = self
            .per_tier
            .entry(tier)
            .or_insert_with(|| TierSummary {
                passed_at_attempt: vec![0; self.max_attempts],
                ..Default::default()
            });
        tier_summary.attempted += 1;

        match &outcome.final_outcome {
            FinalOutcome::Passed { at_attempt } => {
                if *at_attempt < self.max_attempts {
                    self.passed_at_attempt[*at_attempt] += 1;
                    tier_summary.passed_at_attempt[*at_attempt] += 1;
                }
            }
            FinalOutcome::FailedAllAttempts => {
                self.failed_all_attempts += 1;
                tier_summary.failed_all_attempts += 1;
            }
            FinalOutcome::Aborted { .. } => {
                self.aborted += 1;
                tier_summary.aborted += 1;
            }
        }

        for (_c, _v, c_dur, v_dur, cost) in &outcome.attempts {
            self.completion_durations.push(*c_dur);
            self.verify_durations.push(*v_dur);
            if let Some(c) = cost {
                *self.total_cost_usd.get_or_insert(0.0) += c;
            }
        }
    }

    fn finalize(self, _max_attempts: usize) -> ModelSummary {
        let mean_completion =
            mean_duration(&self.completion_durations).unwrap_or_default();
        let mean_verify = mean_duration(&self.verify_durations).unwrap_or_default();
        ModelSummary {
            per_tier: self.per_tier,
            passed_at_attempt: self.passed_at_attempt,
            failed_all_attempts: self.failed_all_attempts,
            aborted: self.aborted,
            attempted: self.attempted,
            mean_completion_duration: mean_completion,
            mean_verify_duration: mean_verify,
            total_cost_usd: self.total_cost_usd,
        }
    }
}

fn mean_duration(ds: &[Duration]) -> Option<Duration> {
    if ds.is_empty() {
        return None;
    }
    let total_nanos: u128 = ds.iter().map(|d| d.as_nanos()).sum();
    let mean_nanos = total_nanos / ds.len() as u128;
    Some(Duration::from_nanos(mean_nanos as u64))
}

// =============================================================================
// Helpers
// =============================================================================

fn prompt_shape_name(s: PromptShape) -> &'static str {
    match s {
        PromptShape::VanillaCreature => "VanillaCreature",
        PromptShape::FrenchVanillaCreature => "FrenchVanillaCreature",
        PromptShape::SingleEffectSpell => "SingleEffectSpell",
        PromptShape::TriggeredAbilityCreature => "TriggeredAbilityCreature",
    }
}

fn write_jsonl_row<T: Serialize>(file: &mut File, row: &T) -> Result<()> {
    let line = serde_json::to_string(row).context("serializing JSONL row")?;
    writeln!(file, "{line}").context("writing JSONL row")?;
    file.flush().context("flushing JSONL row")?;
    Ok(())
}

fn iso8601_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Format a report for terminal display. Called by the binary to
/// print a compact summary at end of run. Also used in tests.
pub fn format_report(report: &BakeoffReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "=== Bake-off Report ===\n\
         total cards sampled: {}\n\
         card_seed: {}\n\
         model_seed: {}\n\
         JSONL path: {}\n\n",
        report.total_cards_sampled,
        report.card_seed,
        report.model_seed,
        report.output_path.display(),
    ));
    if !report.unsupported.is_empty() {
        out.push_str("Unsupported (pipeline-coverage, identical across models):\n");
        let mut kvs: Vec<_> = report.unsupported.iter().collect();
        kvs.sort_by(|a, b| b.1.cmp(a.1));
        for (reason, count) in kvs {
            out.push_str(&format!("  {count:>4}  {reason}\n"));
        }
        out.push('\n');
    }
    out.push_str("Per-model summary:\n");
    let mut model_ids: Vec<_> = report.per_model.keys().collect();
    model_ids.sort();
    for id in model_ids {
        let m = &report.per_model[id];
        let passed_total: usize = m.passed_at_attempt.iter().sum();
        let denom = m.attempted.max(1);
        out.push_str(&format!(
            "\n  [{id}] attempted={}, passed={} ({:.1}%), failed={}, aborted={}\n",
            m.attempted,
            passed_total,
            100.0 * passed_total as f64 / denom as f64,
            m.failed_all_attempts,
            m.aborted,
        ));
        for (idx, n) in m.passed_at_attempt.iter().enumerate() {
            out.push_str(&format!(
                "    passed on attempt {} (0-indexed): {}\n",
                idx, n
            ));
        }
        out.push_str(&format!(
            "    mean completion: {}ms, mean verify: {}ms\n",
            m.mean_completion_duration.as_millis(),
            m.mean_verify_duration.as_millis(),
        ));
        if let Some(cost) = m.total_cost_usd {
            out.push_str(&format!("    total cost: ${cost:.4}\n"));
        }
        // Per-tier breakdown
        let mut tiers: Vec<_> = m.per_tier.keys().collect();
        tiers.sort();
        for tier in tiers {
            let ts = &m.per_tier[tier];
            let t_passed: usize = ts.passed_at_attempt.iter().sum();
            let t_denom = ts.attempted.max(1);
            out.push_str(&format!(
                "    T{}: {}/{} passed ({:.1}%), {} failed, {} aborted\n",
                tier.as_number(),
                t_passed,
                ts.attempted,
                100.0 * t_passed as f64 / t_denom as f64,
                ts.failed_all_attempts,
                ts.aborted,
            ));
        }
    }
    out
}

/// Let callers construct a `ScryfallPool`-compatible view from a
/// path to a JSON file. Re-exported here so the binary doesn't
/// need a direct dep on scryfall.
pub fn load_default_pool() -> Result<ScryfallPool> {
    ScryfallPool::load_default().context("loading default Scryfall pool")
}

/// Helper for the binary: ensures `path` has a parent and creates it.
pub fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    Ok(())
}

// =============================================================================
// Prompt dump (subagent / out-of-process generation backend)
// =============================================================================

/// One manifest row written by [`dump_prompts`]. Carries everything
/// the out-of-process generation step (a Claude Code subagent, a
/// human, a different harness) needs to produce a candidate, plus
/// the Scryfall-derived structural fields the layer-2 checker diffs
/// the generated `CardDefinition` against.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DumpRow {
    /// Stable 0-based index; also the prompt-file name prefix.
    pub idx: usize,
    /// Filesystem-safe handle derived from the card name.
    pub slug: String,
    pub tier: u8,
    /// `true` when `render_prompt` produced a prompt. `false` rows
    /// carry `unsupported_reason` and have no `prompt_file`.
    pub supported: bool,
    pub shape: Option<String>,
    pub unsupported_reason: Option<String>,
    pub name: String,
    pub oracle_id: String,
    pub set: String,
    pub mana_cost: Option<String>,
    pub cmc: f32,
    pub type_line: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub colors: Vec<String>,
    pub keywords: Vec<String>,
    /// Path (relative to the dump dir) of the prompt file, or `None`
    /// for unsupported rows.
    pub prompt_file: Option<String>,
}

/// Render the deterministic prefix of the pipeline (sample →
/// classify → render_prompt) and write it to disk instead of
/// calling a model. Produces, under `out_dir`:
///
/// * `manifest.jsonl` — one [`DumpRow`] per sampled card.
/// * `prompts/<idx>_<slug>.txt` — the system + user prompt for each
///   supported card, with a delimiter the reader can split on.
///
/// This is the seam for the subagent backend: a generator reads a
/// prompt file, writes `<idx>_<slug>.rs` next to it, and the
/// `verify_dir` driver picks the candidates up. No network, no LLM
/// clients constructed. Returns `(supported, unsupported)` counts.
pub fn dump_prompts(
    pool: &ScryfallPool,
    config: &BakeoffConfig,
    out_dir: &Path,
) -> Result<(usize, usize)> {
    let prompts_dir = out_dir.join("prompts");
    std::fs::create_dir_all(&prompts_dir)
        .with_context(|| format!("creating {}", prompts_dir.display()))?;

    let sampled = sample_cards(pool, config);
    eprintln!(
        "dump: sampled {} cards (card_seed={}, size_per_tier={}, t4_control={})",
        sampled.len(),
        config.card_seed,
        config.sample_size_per_tier,
        config.t4_control_sample,
    );

    let manifest_path = out_dir.join("manifest.jsonl");
    let mut manifest = File::create(&manifest_path)
        .with_context(|| format!("creating {}", manifest_path.display()))?;

    let mut supported = 0usize;
    let mut unsupported = 0usize;
    for (idx, (card, tier)) in sampled.iter().enumerate() {
        let slug = slugify(&card.name);
        let mut row = DumpRow {
            idx,
            slug: slug.clone(),
            tier: tier.as_number(),
            supported: false,
            shape: None,
            unsupported_reason: None,
            name: card.name.clone(),
            oracle_id: card.oracle_id.clone(),
            set: card.set.clone(),
            mana_cost: card.mana_cost.clone(),
            cmc: card.cmc,
            type_line: card.type_line.clone(),
            power: card.power.clone(),
            toughness: card.toughness.clone(),
            colors: card.colors.clone(),
            keywords: card.keywords.clone(),
            prompt_file: None,
        };
        match render_prompt(card, *tier) {
            Ok(prompt) => {
                let fname = format!("{idx:03}_{slug}.txt");
                let body = format!(
                    "# Arcana card-gen prompt — {name} (T{tier}, {shape})\n\
                     #\n\
                     # Write ONLY the Rust source for this card to a sibling file\n\
                     # named {idx:03}_{slug}.rs. No markdown fences, no prose.\n\
                     #\n\
                     ===== SYSTEM =====\n{system}\n\
                     ===== USER =====\n{user}\n",
                    name = card.name,
                    tier = tier.as_number(),
                    shape = prompt_shape_name(prompt.shape),
                    system = prompt.system,
                    user = prompt.user,
                );
                std::fs::write(prompts_dir.join(&fname), body)
                    .with_context(|| format!("writing prompt {fname}"))?;
                row.supported = true;
                row.shape = Some(prompt_shape_name(prompt.shape).to_string());
                row.prompt_file = Some(format!("prompts/{fname}"));
                supported += 1;
            }
            Err(u) => {
                row.unsupported_reason = Some(u.to_string());
                unsupported += 1;
            }
        }
        let line = serde_json::to_string(&row).context("serializing DumpRow")?;
        writeln!(manifest, "{line}").context("writing manifest row")?;
    }
    manifest.flush().context("flushing manifest")?;
    eprintln!(
        "dump: wrote {supported} prompt(s) + {unsupported} unsupported row(s) to {}",
        out_dir.display()
    );
    Ok((supported, unsupported))
}

/// Filesystem- and module-safe slug: lowercase, non-alphanumeric
/// runs collapsed to a single `_`, leading/trailing `_` trimmed.
/// `"Bonecrusher Giant // Stomp"` → `"bonecrusher_giant_stomp"`.
pub fn slugify(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut prev_us = true; // trims leading separators
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_us = false;
        } else if !prev_us {
            out.push('_');
            prev_us = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() {
        out.push_str("card");
    }
    out
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::MockClient;

    // Tiny fixture: one T1 vanilla, one T2 french-vanilla, one T4
    // X-cost. Enough to exercise every outcome path in a single
    // test run while keeping the response queue short.
    const FIXTURE_POOL: &str = r#"[
        {
            "id":"bears-1","oracle_id":"bears","name":"Test Bear",
            "mana_cost":"{1}{G}","cmc":2.0,
            "type_line":"Creature — Bear","oracle_text":"",
            "power":"2","toughness":"2",
            "colors":["G"],"color_identity":["G"],"keywords":[],
            "legalities":{"standard":"legal"},
            "rarity":"common","set":"tst","layout":"normal"
        },
        {
            "id":"fireball-1","oracle_id":"fireball","name":"Test Fireball",
            "mana_cost":"{X}{R}","cmc":1.0,
            "type_line":"Sorcery","oracle_text":"Test Fireball deals X damage to any target.",
            "colors":["R"],"color_identity":["R"],"keywords":[],
            "legalities":{"standard":"legal"},
            "rarity":"common","set":"tst","layout":"normal"
        }
    ]"#;

    const GRIZZLY_BEARS_SOURCE: &str =
        include_str!("../../arcana-cards/src/lea/grizzly_bears.rs");

    const BROKEN_SOURCE: &str =
        "use arcana_core::DefinitelyNonexistentType;\n\
         pub fn register() -> () {}\n";

    /// RAII guard: restores the scratch file on drop. Same pattern
    /// as verify::tests::RestoreScratch — shared here via include.
    struct RestoreScratch;
    impl Drop for RestoreScratch {
        fn drop(&mut self) {
            let _ = std::fs::write(
                crate::verify::scratch_path_for("candidate"),
                GRIZZLY_BEARS_SOURCE,
            );
        }
    }

    fn mk_config(output: PathBuf) -> BakeoffConfig {
        BakeoffConfig {
            sample_size_per_tier: 10,
            tiers: vec![Tier::One, Tier::Two, Tier::Three],
            max_attempts: 2,
            card_seed: 42,
            model_seed: 42,
            output_path: output,
            t4_control_sample: 5,
            restrict_standard: true,
            preflight_card_names: vec![], // skip preflight in tests
        }
    }

    fn tmp_jsonl_path(tag: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("arcana-gen-bakeoff-{tag}.jsonl"));
        let _ = std::fs::remove_file(&p);
        p
    }

    #[test]
    #[ignore]
    fn end_to_end_passes_one_shot_when_good_candidate() {
        let _guard = RestoreScratch;
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let mock = MockClient::new(
            "mock",
            vec![GRIZZLY_BEARS_SOURCE.to_string()],
        );
        let config = mk_config(tmp_jsonl_path("one_shot"));
        let models: Vec<&dyn LlmClient> = vec![&mock];
        let report = run(&pool, &models, &config).expect("run");

        // One T1 vanilla + one T4-routed-to-Unsupported.
        assert_eq!(report.total_cards_sampled, 2, "one T1 + one T4 control");
        assert_eq!(report.unsupported.len(), 1, "T4 routes to one Unsupported bucket");

        let mock_summary = &report.per_model["mock"];
        assert_eq!(mock_summary.attempted, 1, "only the T1 card reaches the model");
        assert_eq!(mock_summary.passed_at_attempt[0], 1, "passed on first try");
        assert_eq!(mock_summary.passed_at_attempt[1], 0);
        assert_eq!(mock_summary.failed_all_attempts, 0);

        // JSONL file exists and has the expected row count:
        // 1 attempted + 1 unsupported = 2 rows.
        let lines = std::fs::read_to_string(&config.output_path)
            .expect("JSONL readable")
            .lines()
            .count();
        assert_eq!(lines, 2);
    }

    #[test]
    #[ignore]
    fn end_to_end_passes_on_retry_after_compile_error() {
        let _guard = RestoreScratch;
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let mock = MockClient::new(
            "mock",
            vec![
                BROKEN_SOURCE.to_string(),
                GRIZZLY_BEARS_SOURCE.to_string(),
            ],
        );
        let config = mk_config(tmp_jsonl_path("retry"));
        let models: Vec<&dyn LlmClient> = vec![&mock];
        let report = run(&pool, &models, &config).expect("run");

        let mock_summary = &report.per_model["mock"];
        assert_eq!(mock_summary.attempted, 1);
        assert_eq!(mock_summary.passed_at_attempt[0], 0, "did not pass first try");
        assert_eq!(mock_summary.passed_at_attempt[1], 1, "passed on retry");
        assert_eq!(mock_summary.failed_all_attempts, 0);
    }

    #[test]
    #[ignore]
    fn end_to_end_fails_all_attempts_when_model_cannot_produce_valid_code() {
        let _guard = RestoreScratch;
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let mock = MockClient::new(
            "mock",
            vec![BROKEN_SOURCE.to_string(), BROKEN_SOURCE.to_string()],
        );
        let config = mk_config(tmp_jsonl_path("fails_all"));
        let models: Vec<&dyn LlmClient> = vec![&mock];
        let report = run(&pool, &models, &config).expect("run");

        let mock_summary = &report.per_model["mock"];
        assert_eq!(mock_summary.attempted, 1);
        assert_eq!(
            mock_summary.passed_at_attempt.iter().sum::<usize>(),
            0,
            "never passed"
        );
        assert_eq!(mock_summary.failed_all_attempts, 1);
    }

    // Note: a "precheck failure aborts run" test would require
    // corrupting arcana-cards outside the scratch file, since
    // precheck overwrites the scratch with Grizzly Bears before
    // running cargo check. That's messier than it's worth; the
    // abort path is visible in `run()` and gets exercised if any
    // real arcana-cards-level breakage occurs during a bake-off.

    #[test]
    fn sample_cards_uses_card_seed_not_model_seed() {
        // Cluster-shard contract: different model_seeds with the
        // same card_seed must produce identical card samples. This
        // is what lets shards run independent trials of the same
        // card set in parallel.
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let mut config_a = mk_config(PathBuf::from("/tmp/unused_a"));
        let mut config_b = mk_config(PathBuf::from("/tmp/unused_b"));
        config_a.card_seed = 7;
        config_a.model_seed = 100;
        config_b.card_seed = 7;
        config_b.model_seed = 999;
        let a = sample_cards(&pool, &config_a);
        let b = sample_cards(&pool, &config_b);
        let a_names: Vec<_> = a.iter().map(|(c, _)| c.name.clone()).collect();
        let b_names: Vec<_> = b.iter().map(|(c, _)| c.name.clone()).collect();
        assert_eq!(a_names, b_names, "model_seed must not affect card sampling");

        // Different card_seeds DO yield different samples (assuming
        // the pool is large enough — fixture is small but the seed
        // change should still permute order).
        config_b.card_seed = 99;
        let c = sample_cards(&pool, &config_b);
        let c_names: Vec<_> = c.iter().map(|(c, _)| c.name.clone()).collect();
        // With a 2-card pool, different shuffle seeds may still
        // produce the same order half the time. Skip the strict
        // inequality assertion; the constancy assertion above is
        // the load-bearing one.
        let _ = c_names;
    }

    #[test]
    fn sample_cards_is_deterministic_under_seed() {
        // Non-ignored — pure function, no cargo spawn. Same seed
        // must yield the same sample, run to run.
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let config = mk_config(PathBuf::from("/tmp/unused"));
        let a = sample_cards(&pool, &config);
        let b = sample_cards(&pool, &config);
        let a_names: Vec<_> = a.iter().map(|(c, _)| c.name.clone()).collect();
        let b_names: Vec<_> = b.iter().map(|(c, _)| c.name.clone()).collect();
        assert_eq!(a_names, b_names);
    }

    #[test]
    fn format_report_includes_seed_and_output_path() {
        let report = BakeoffReport {
            total_cards_sampled: 42,
            card_seed: 123,
            model_seed: 456,
            unsupported: HashMap::new(),
            per_model: HashMap::new(),
            output_path: PathBuf::from("/some/path.jsonl"),
        };
        let s = format_report(&report);
        assert!(s.contains("42"), "total cards must appear");
        assert!(s.contains("123"), "card_seed must appear");
        assert!(s.contains("456"), "model_seed must appear");
        assert!(s.contains("/some/path.jsonl"), "jsonl path must appear");
    }

    #[test]
    fn empty_models_list_is_rejected() {
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let config = mk_config(tmp_jsonl_path("empty_models"));
        let models: Vec<&dyn LlmClient> = vec![];
        let err = run(&pool, &models, &config).unwrap_err();
        assert!(err.to_string().contains("at least one model"));
    }

    #[test]
    fn zero_max_attempts_is_rejected() {
        let pool = ScryfallPool::from_json_str(FIXTURE_POOL).expect("fixture parses");
        let mock = MockClient::new("m", vec![]);
        let mut config = mk_config(tmp_jsonl_path("zero_attempts"));
        config.max_attempts = 0;
        let models: Vec<&dyn LlmClient> = vec![&mock];
        let err = run(&pool, &models, &config).unwrap_err();
        assert!(err.to_string().contains("max_attempts"));
    }
}
