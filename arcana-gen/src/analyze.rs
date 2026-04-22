//! JSONL analyzer for bake-off runs.
//!
//! Reads a file written by [`crate::bakeoff`] and produces an
//! [`Analysis`]: pass-rate matrix, error-code histograms with
//! variant sub-buckets, corpus-hard / corpus-brittle / model-split
//! cards, latency distributions, and costs.
//!
//! Pure aggregation. No cargo spawn, no network, no I/O beyond the
//! one input file read. The binary wrapper at `src/bin/bakeoff_analyze.rs`
//! handles CLI parsing and output formatting.
//!
//! # Design choices
//!
//! * **Mirror structs for deserialization.** The analyzer re-declares
//!   the JSONL row types rather than importing [`crate::bakeoff`]'s
//!   private types. Decouples the two modules: bake-off internal
//!   changes are free as long as the on-the-wire JSONL shape is
//!   preserved, and vice versa.
//!
//! * **corpus_hard AND corpus_brittle, not just one.** corpus_hard
//!   (0% pass across the whole matrix) is the "manual intervention"
//!   signal. corpus_brittle (>0% but <50%) is the "prompt-tweak
//!   might rescue this" signal. Different interventions; both deserve
//!   surfacing.
//!
//! * **Error histograms bucket by rustc code with variant sub-buckets
//!   for E0599 / E0433.** "E0599: 42" tells you nothing actionable;
//!   "E0599: 42, of which 18 are `KeywordAbility::Flash` and 12 are
//!   `Effect::MultiplyPower`" tells you exactly what to add to the
//!   CRITICAL section of the system prompt. E0308 extraction is
//!   deferred — the type-mismatch message is harder to regex.
//!
//! * **Latency split into all / passing.** Failed attempts include
//!   timeouts that skew the tail; "how long does a *successful*
//!   generation take" is the decision-relevant number.
//!
//! * **Infrastructure warnings first in the terminal output.** If the
//!   classifier leaked T4 cards into a model, every downstream number
//!   is contaminated — the reader needs to see that before they form
//!   an opinion on the data.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// =============================================================================
// Mirror structs — deserialize the JSONL row shapes written by bakeoff.rs
// =============================================================================

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum JsonlRow {
    Unsupported(UnsupportedRow),
    Attempted(AttemptedRow),
}

#[derive(Debug, Deserialize)]
struct UnsupportedRow {
    #[allow(dead_code)]
    timestamp: String,
    card: CardMeta,
    tier: u8,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct AttemptedRow {
    #[allow(dead_code)]
    timestamp: String,
    card: CardMeta,
    tier: u8,
    shape: String,
    model: String,
    // Note: bakeoff writes these as u128 (Duration::as_millis returns
    // u128), but serde_json can't deserialize u128 without the
    // arbitrary_precision feature. ms values fit a u64 by many orders
    // of magnitude, so we take them at u64 here.
    #[allow(dead_code)]
    prompt_render_duration_ms: u64,
    attempts: Vec<AttemptEntry>,
    final_outcome: FinalOutcomeRow,
}

#[derive(Debug, Deserialize, Clone)]
struct CardMeta {
    name: String,
    oracle_id: String,
    #[allow(dead_code)]
    set: String,
}

#[derive(Debug, Deserialize)]
struct AttemptEntry {
    completion: CompletionEntry,
    verify: VerifyEntry,
}

#[derive(Debug, Deserialize)]
struct CompletionEntry {
    #[allow(dead_code)]
    text: String,
    duration_ms: u64,
    #[allow(dead_code)]
    prompt_tokens: Option<u32>,
    #[allow(dead_code)]
    completion_tokens: Option<u32>,
    cost_usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct VerifyEntry {
    result: VerifyResultEntry,
    duration_ms: u64,
    #[allow(dead_code)]
    error_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum VerifyResultEntry {
    Passed,
    FailedInCandidate { errors: Vec<CompileErrorEntry> },
    FailedElsewhere {
        #[allow(dead_code)]
        errors: Vec<CompileErrorEntry>,
    },
    InfrastructureError {
        #[allow(dead_code)]
        message: String,
    },
}

#[derive(Debug, Deserialize, Clone)]
struct CompileErrorEntry {
    #[allow(dead_code)]
    file: String,
    #[allow(dead_code)]
    line: u32,
    #[allow(dead_code)]
    column: u32,
    #[allow(dead_code)]
    level: String,
    code: Option<String>,
    message: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FinalOutcomeRow {
    Passed { at_attempt: usize },
    FailedAllAttempts,
    Aborted {
        #[allow(dead_code)]
        reason: String,
    },
}

// =============================================================================
// Analysis — public output shape
// =============================================================================

/// Top-level analysis for a single JSONL run file.
#[derive(Debug, Serialize, Clone)]
pub struct Analysis {
    pub run_meta: RunMeta,
    /// Infrastructure warnings. If non-empty, downstream numbers are
    /// partially contaminated — the reader should fix the pipeline
    /// before interpreting pass rates.
    pub t4_anomalies: Vec<T4Anomaly>,
    pub pass_rate_matrix: Vec<PassRateCell>,
    pub error_histograms: Vec<ModelErrorHistogram>,
    /// Cards where every model failed every attempt. Ranked by
    /// models_failed_count DESC, then tier DESC, then name.
    pub corpus_hard: Vec<CardFailureFacet>,
    /// Cards with >0% but <50% pass rate across (model, attempt).
    /// Prompt-tweak candidates.
    pub corpus_brittle: Vec<CardFailureFacet>,
    /// Cards where ≥1 model passed AND ≥1 different model failed all
    /// attempts. The differentiation signal between models.
    pub model_split: Vec<CardFailureFacet>,
    pub latency: Vec<ModelLatency>,
    pub cost_totals: Vec<ModelCost>,
    pub unsupported_by_reason: Vec<(String, usize)>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct RunMeta {
    pub total_rows: usize,
    pub malformed_rows: usize,
    pub unsupported_rows: usize,
    pub attempted_rows: usize,
    /// Unique oracle_ids across all rows.
    pub cards_sampled: usize,
    /// (tier, unique-card-count) sorted by tier ASC.
    pub tier_counts: Vec<(u8, usize)>,
    pub models: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct T4Anomaly {
    pub card_name: String,
    pub card_oracle_id: String,
    /// The tier recorded in the JSONL row — what the classifier
    /// assigned. If 4, the driver leaked a T4 into the model loop
    /// (prompt-coverage / driver-filter bug). If not 4 but this
    /// card is actually T4, the classifier misrouted — but that's
    /// not detectable from JSONL alone, requires a re-classify pass.
    pub classified_tier: u8,
    pub shape: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PassRateCell {
    pub model: String,
    pub tier: u8,
    pub attempt_idx: usize,
    /// Cards whose FIRST successful attempt was this one. Marginal
    /// view — answers "is the retry loop earning its keep."
    pub n_passed_at_this_attempt: usize,
    /// Cards passed at this attempt_idx or any earlier one.
    /// Cumulative view — answers "with a K-attempt budget, what's
    /// the effective pass rate."
    pub n_passed_cumulative: usize,
    /// Total cards this (model, tier) slot saw.
    pub n_attempted: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModelErrorHistogram {
    pub model: String,
    /// Denominator for `pct_of_model_errors`. Sum of all
    /// FailedInCandidate diagnostics across every attempt by this
    /// model.
    pub total_errors: usize,
    /// Top-N error codes by count, DESC.
    pub buckets: Vec<ErrorBucket>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorBucket {
    /// Rustc error code. `None` displays as "other".
    pub code: Option<String>,
    pub count: usize,
    pub pct_of_model_errors: f32,
    /// Sub-buckets populated for E0599 (enum::variant / struct::method)
    /// and E0433 (undeclared type/module). Other codes yield an empty
    /// vec. TODO: E0308 type-mismatch variant extraction — needs
    /// richer message parsing.
    pub top_variants: Vec<(String, usize)>,
}

/// Unified facet used for `corpus_hard`, `corpus_brittle`, and
/// `model_split`. The three categories differ only in filter
/// predicate, not shape:
///
/// * `corpus_hard`    — `models_passed.is_empty()` AND
///                      `!models_failed_all.is_empty()`.
/// * `corpus_brittle` — `0 < pass_rate < 0.5`.
/// * `model_split`    — `!models_passed.is_empty()` AND
///                      `!models_failed_all.is_empty()`.
///
/// A single card can legitimately appear in both `corpus_brittle`
/// and `model_split` (e.g., A passes 1/3, B fails 3/3).
#[derive(Debug, Serialize, Clone)]
pub struct CardFailureFacet {
    pub name: String,
    pub oracle_id: String,
    pub tier: u8,
    pub shape: String,
    pub models_passed: Vec<String>,
    pub models_failed_all: Vec<String>,
    pub models_aborted: Vec<String>,
    /// Fraction of (model, attempt) cells that passed. Denominator
    /// excludes attempts inside aborted runs. In [0.0, 1.0].
    pub pass_rate: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModelLatency {
    pub model: String,
    pub completion_all: LatencyStats,
    /// Completion durations for attempts that ended in `Passed`. The
    /// honest "how long does a working generation take" number.
    pub completion_passing: LatencyStats,
    pub verify_all: LatencyStats,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct LatencyStats {
    pub n: usize,
    pub p50_ms: u64,
    pub p90_ms: u64,
    pub mean_ms: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModelCost {
    pub model: String,
    pub total_usd: f64,
    pub attempts_with_cost: usize,
}

// =============================================================================
// Entry points
// =============================================================================

#[derive(Debug, Clone)]
pub struct AnalyzeConfig {
    pub top_errors: usize,
    pub top_hard: usize,
}

impl Default for AnalyzeConfig {
    fn default() -> Self {
        Self { top_errors: 10, top_hard: 20 }
    }
}

/// Read a JSONL file and compute its [`Analysis`].
pub fn analyze_file(path: &Path, config: &AnalyzeConfig) -> Result<Analysis> {
    let f = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let reader = BufReader::new(f);
    let mut rows: Vec<JsonlRow> = Vec::new();
    let mut malformed = 0usize;
    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result.with_context(|| format!("reading line {}", i + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<JsonlRow>(&line) {
            Ok(r) => rows.push(r),
            Err(e) => {
                eprintln!(
                    "bakeoff-analyze: skipping malformed line {}: {e}",
                    i + 1
                );
                malformed += 1;
            }
        }
    }
    Ok(analyze_rows(rows, malformed, config))
}

fn analyze_rows(
    rows: Vec<JsonlRow>,
    malformed_rows: usize,
    config: &AnalyzeConfig,
) -> Analysis {
    let mut unsupported: Vec<UnsupportedRow> = Vec::new();
    let mut attempted: Vec<AttemptedRow> = Vec::new();
    for row in rows {
        match row {
            JsonlRow::Unsupported(u) => unsupported.push(u),
            JsonlRow::Attempted(a) => attempted.push(a),
        }
    }

    let run_meta = build_run_meta(&unsupported, &attempted, malformed_rows);
    let t4_anomalies = detect_t4_anomalies(&attempted);
    let pass_rate_matrix = build_pass_rate_matrix(&attempted);
    let error_histograms = build_error_histograms(&attempted, config.top_errors);
    let (corpus_hard, corpus_brittle, model_split) =
        bucket_card_failures(&attempted, config.top_hard);
    let latency = build_latency(&attempted);
    let cost_totals = build_cost_totals(&attempted);
    let unsupported_by_reason = tally_unsupported(&unsupported);

    Analysis {
        run_meta,
        t4_anomalies,
        pass_rate_matrix,
        error_histograms,
        corpus_hard,
        corpus_brittle,
        model_split,
        latency,
        cost_totals,
        unsupported_by_reason,
    }
}

// =============================================================================
// Aggregators
// =============================================================================

fn build_run_meta(
    unsupported: &[UnsupportedRow],
    attempted: &[AttemptedRow],
    malformed_rows: usize,
) -> RunMeta {
    let mut oracle_ids: HashSet<&str> = HashSet::new();
    let mut tier_to_oracles: BTreeMap<u8, HashSet<&str>> = BTreeMap::new();
    let mut models: HashSet<&str> = HashSet::new();

    for r in unsupported {
        oracle_ids.insert(&r.card.oracle_id);
        tier_to_oracles.entry(r.tier).or_default().insert(&r.card.oracle_id);
    }
    for r in attempted {
        oracle_ids.insert(&r.card.oracle_id);
        tier_to_oracles.entry(r.tier).or_default().insert(&r.card.oracle_id);
        models.insert(&r.model);
    }

    let mut models_out: Vec<String> = models.iter().map(|s| s.to_string()).collect();
    models_out.sort();
    let tier_counts: Vec<(u8, usize)> =
        tier_to_oracles.iter().map(|(t, s)| (*t, s.len())).collect();

    RunMeta {
        total_rows: unsupported.len() + attempted.len(),
        malformed_rows,
        unsupported_rows: unsupported.len(),
        attempted_rows: attempted.len(),
        cards_sampled: oracle_ids.len(),
        tier_counts,
        models: models_out,
    }
}

fn detect_t4_anomalies(attempted: &[AttemptedRow]) -> Vec<T4Anomaly> {
    // A T4 card reaching a model = render_prompt didn't filter it out.
    // Dedupe by oracle_id so the same card isn't listed N times (once
    // per model).
    let mut seen: HashSet<&str> = HashSet::new();
    let mut out = Vec::new();
    for r in attempted {
        if r.tier != 4 {
            continue;
        }
        if !seen.insert(&r.card.oracle_id) {
            continue;
        }
        out.push(T4Anomaly {
            card_name: r.card.name.clone(),
            card_oracle_id: r.card.oracle_id.clone(),
            classified_tier: r.tier,
            shape: r.shape.clone(),
        });
    }
    out.sort_by(|a, b| a.card_name.cmp(&b.card_name));
    out
}

fn build_pass_rate_matrix(attempted: &[AttemptedRow]) -> Vec<PassRateCell> {
    // (model, tier) -> (n_attempted, marginal[attempt_idx])
    let mut buckets: HashMap<(String, u8), MatrixBucket> = HashMap::new();
    let mut max_attempts: usize = 0;

    for r in attempted {
        let entry = buckets
            .entry((r.model.clone(), r.tier))
            .or_insert_with(MatrixBucket::default);
        entry.n_attempted += 1;
        if let FinalOutcomeRow::Passed { at_attempt } = r.final_outcome {
            while entry.marginal.len() <= at_attempt {
                entry.marginal.push(0);
            }
            entry.marginal[at_attempt] += 1;
            if at_attempt + 1 > max_attempts {
                max_attempts = at_attempt + 1;
            }
        }
        if r.attempts.len() > max_attempts {
            max_attempts = r.attempts.len();
        }
    }
    // max_attempts is at least 1 so we emit a row per (model, tier).
    if max_attempts == 0 {
        max_attempts = 1;
    }

    let mut out: Vec<PassRateCell> = Vec::new();
    for ((model, tier), bucket) in buckets {
        let mut cumulative: usize = 0;
        for attempt_idx in 0..max_attempts {
            let marginal = bucket.marginal.get(attempt_idx).copied().unwrap_or(0);
            cumulative += marginal;
            out.push(PassRateCell {
                model: model.clone(),
                tier,
                attempt_idx,
                n_passed_at_this_attempt: marginal,
                n_passed_cumulative: cumulative,
                n_attempted: bucket.n_attempted,
            });
        }
    }
    out.sort_by(|a, b| {
        a.model
            .cmp(&b.model)
            .then(a.tier.cmp(&b.tier))
            .then(a.attempt_idx.cmp(&b.attempt_idx))
    });
    out
}

#[derive(Default)]
struct MatrixBucket {
    n_attempted: usize,
    marginal: Vec<usize>,
}

fn build_error_histograms(
    attempted: &[AttemptedRow],
    top_n: usize,
) -> Vec<ModelErrorHistogram> {
    // model -> code -> (count, variant -> count)
    let mut per_model: HashMap<String, HashMap<Option<String>, CodeBucket>> =
        HashMap::new();

    for r in attempted {
        let bucket = per_model.entry(r.model.clone()).or_default();
        for attempt in &r.attempts {
            let errors = match &attempt.verify.result {
                VerifyResultEntry::FailedInCandidate { errors } => errors,
                // FailedElsewhere = arcana-cards broken, not a
                // candidate error. InfrastructureError = no errors.
                // Passed = no errors. Skip.
                _ => continue,
            };
            for e in errors {
                let code_key = e.code.clone();
                let entry = bucket.entry(code_key).or_default();
                entry.count += 1;
                if let Some(v) = extract_variant(&e.code, &e.message) {
                    *entry.variants.entry(v).or_insert(0) += 1;
                }
            }
        }
    }

    let mut out: Vec<ModelErrorHistogram> = Vec::new();
    for (model, by_code) in per_model {
        let total_errors: usize = by_code.values().map(|b| b.count).sum();
        let mut buckets: Vec<ErrorBucket> = by_code
            .into_iter()
            .map(|(code, bucket)| {
                let pct = if total_errors == 0 {
                    0.0
                } else {
                    100.0 * bucket.count as f32 / total_errors as f32
                };
                let mut variants: Vec<(String, usize)> =
                    bucket.variants.into_iter().collect();
                variants.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                variants.truncate(top_n);
                ErrorBucket {
                    code,
                    count: bucket.count,
                    pct_of_model_errors: pct,
                    top_variants: variants,
                }
            })
            .collect();
        buckets.sort_by(|a, b| b.count.cmp(&a.count));
        buckets.truncate(top_n);
        out.push(ModelErrorHistogram { model, total_errors, buckets });
    }
    out.sort_by(|a, b| a.model.cmp(&b.model));
    out
}

#[derive(Default)]
struct CodeBucket {
    count: usize,
    variants: HashMap<String, usize>,
}

fn bucket_card_failures(
    attempted: &[AttemptedRow],
    top_hard: usize,
) -> (Vec<CardFailureFacet>, Vec<CardFailureFacet>, Vec<CardFailureFacet>) {
    // Group by oracle_id.
    let mut by_card: HashMap<String, CardAgg> = HashMap::new();
    for r in attempted {
        let agg = by_card
            .entry(r.card.oracle_id.clone())
            .or_insert_with(|| CardAgg::new(&r.card.name, r.tier, &r.shape));
        agg.record(&r.model, &r.final_outcome, &r.attempts);
    }

    let mut all_facets: Vec<CardFailureFacet> = by_card
        .into_iter()
        .map(|(oracle_id, agg)| agg.finalize(oracle_id))
        .collect();

    let mut corpus_hard: Vec<CardFailureFacet> = all_facets
        .iter()
        .filter(|f| f.models_passed.is_empty() && !f.models_failed_all.is_empty())
        .cloned()
        .collect();
    // Ranking: models_failed_all.len() DESC, tier DESC, name ASC.
    corpus_hard.sort_by(|a, b| {
        b.models_failed_all
            .len()
            .cmp(&a.models_failed_all.len())
            .then(b.tier.cmp(&a.tier))
            .then(a.name.cmp(&b.name))
    });
    corpus_hard.truncate(top_hard);

    let mut corpus_brittle: Vec<CardFailureFacet> = all_facets
        .iter()
        .filter(|f| f.pass_rate > 0.0 && f.pass_rate < 0.5)
        .cloned()
        .collect();
    // Ranking: pass_rate ASC (most-broken first), then tier DESC, name ASC.
    corpus_brittle.sort_by(|a, b| {
        a.pass_rate
            .partial_cmp(&b.pass_rate)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.tier.cmp(&a.tier))
            .then(a.name.cmp(&b.name))
    });
    corpus_brittle.truncate(top_hard);

    let mut model_split: Vec<CardFailureFacet> = all_facets
        .drain(..)
        .filter(|f| !f.models_passed.is_empty() && !f.models_failed_all.is_empty())
        .collect();
    // Ranking: models_failed_all.len() DESC, tier DESC, name ASC. The
    // "how many models got stuck" signal is the actionable one here too.
    model_split.sort_by(|a, b| {
        b.models_failed_all
            .len()
            .cmp(&a.models_failed_all.len())
            .then(b.tier.cmp(&a.tier))
            .then(a.name.cmp(&b.name))
    });
    model_split.truncate(top_hard);

    (corpus_hard, corpus_brittle, model_split)
}

struct CardAgg {
    name: String,
    tier: u8,
    shape: String,
    models_passed: Vec<String>,
    models_failed_all: Vec<String>,
    models_aborted: Vec<String>,
    /// Count of attempts that produced a Passed verify, across all models.
    n_passed_attempts: usize,
    /// Count of attempts in non-aborted runs. Denominator for pass_rate.
    n_accounted_attempts: usize,
}

impl CardAgg {
    fn new(name: &str, tier: u8, shape: &str) -> Self {
        Self {
            name: name.to_string(),
            tier,
            shape: shape.to_string(),
            models_passed: Vec::new(),
            models_failed_all: Vec::new(),
            models_aborted: Vec::new(),
            n_passed_attempts: 0,
            n_accounted_attempts: 0,
        }
    }

    fn record(
        &mut self,
        model: &str,
        final_outcome: &FinalOutcomeRow,
        attempts: &[AttemptEntry],
    ) {
        match final_outcome {
            FinalOutcomeRow::Passed { .. } => {
                self.models_passed.push(model.to_string());
                self.n_accounted_attempts += attempts.len();
                for a in attempts {
                    if matches!(a.verify.result, VerifyResultEntry::Passed) {
                        self.n_passed_attempts += 1;
                    }
                }
            }
            FinalOutcomeRow::FailedAllAttempts => {
                self.models_failed_all.push(model.to_string());
                self.n_accounted_attempts += attempts.len();
            }
            FinalOutcomeRow::Aborted { .. } => {
                self.models_aborted.push(model.to_string());
                // Aborted attempts contribute neither to numerator
                // nor denominator — they're infra noise.
            }
        }
    }

    fn finalize(self, oracle_id: String) -> CardFailureFacet {
        let pass_rate = if self.n_accounted_attempts == 0 {
            0.0
        } else {
            self.n_passed_attempts as f32 / self.n_accounted_attempts as f32
        };
        let mut mp = self.models_passed;
        mp.sort();
        let mut mf = self.models_failed_all;
        mf.sort();
        let mut ma = self.models_aborted;
        ma.sort();
        CardFailureFacet {
            name: self.name,
            oracle_id,
            tier: self.tier,
            shape: self.shape,
            models_passed: mp,
            models_failed_all: mf,
            models_aborted: ma,
            pass_rate,
        }
    }
}

fn build_latency(attempted: &[AttemptedRow]) -> Vec<ModelLatency> {
    let mut per_model: HashMap<String, LatencyAccum> = HashMap::new();
    for r in attempted {
        let acc = per_model.entry(r.model.clone()).or_default();
        for a in &r.attempts {
            acc.completion_all.push(a.completion.duration_ms);
            acc.verify_all.push(a.verify.duration_ms);
            if matches!(a.verify.result, VerifyResultEntry::Passed) {
                acc.completion_passing.push(a.completion.duration_ms);
            }
        }
    }

    let mut out: Vec<ModelLatency> = per_model
        .into_iter()
        .map(|(model, mut acc)| ModelLatency {
            model,
            completion_all: latency_stats(&mut acc.completion_all),
            completion_passing: latency_stats(&mut acc.completion_passing),
            verify_all: latency_stats(&mut acc.verify_all),
        })
        .collect();
    out.sort_by(|a, b| a.model.cmp(&b.model));
    out
}

#[derive(Default)]
struct LatencyAccum {
    completion_all: Vec<u64>,
    completion_passing: Vec<u64>,
    verify_all: Vec<u64>,
}

fn latency_stats(durations_ms: &mut Vec<u64>) -> LatencyStats {
    if durations_ms.is_empty() {
        return LatencyStats::default();
    }
    durations_ms.sort_unstable();
    let n = durations_ms.len();
    let p50 = durations_ms[n / 2];
    let p90_idx = ((n * 9) / 10).min(n - 1);
    let p90 = durations_ms[p90_idx];
    let mean = (durations_ms.iter().map(|&d| d as u128).sum::<u128>() / n as u128) as u64;
    LatencyStats { n, p50_ms: p50, p90_ms: p90, mean_ms: mean }
}

fn build_cost_totals(attempted: &[AttemptedRow]) -> Vec<ModelCost> {
    let mut per_model: HashMap<String, (f64, usize)> = HashMap::new();
    for r in attempted {
        for a in &r.attempts {
            if let Some(c) = a.completion.cost_usd {
                let entry = per_model.entry(r.model.clone()).or_insert((0.0, 0));
                entry.0 += c;
                entry.1 += 1;
            }
        }
    }
    let mut out: Vec<ModelCost> = per_model
        .into_iter()
        .map(|(model, (total, n))| ModelCost {
            model,
            total_usd: total,
            attempts_with_cost: n,
        })
        .collect();
    out.sort_by(|a, b| a.model.cmp(&b.model));
    out
}

fn tally_unsupported(unsupported: &[UnsupportedRow]) -> Vec<(String, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for r in unsupported {
        *counts.entry(r.reason.as_str()).or_insert(0) += 1;
    }
    let mut out: Vec<(String, usize)> =
        counts.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    // DESC by count, then ASC by reason for ties.
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

// =============================================================================
// Error variant extraction
// =============================================================================
//
// Rustc error messages have stable English phrasing for the common
// codes. Extracting the invented identifier lets the histogram report
// *which* enums the model hallucinates on, not just that E0599 fired N
// times. The helpers below use plain-string scanning (no regex dep) and
// return None if the phrasing doesn't match a known shape — the error
// still counts in its top-level bucket.

fn extract_variant(code: &Option<String>, message: &str) -> Option<String> {
    match code.as_deref() {
        Some("E0599") => extract_e0599(message),
        Some("E0433") => extract_e0433(message),
        // TODO: E0308 (mismatched types) would need parsing the
        // "expected `X`, found `Y`" form into a facet like
        // "field::expected_actual". Deferred — the message shape is
        // richer and the data isn't worth v1 complexity yet.
        _ => None,
    }
}

/// E0599 phrasings we target:
///   - "no variant or associated item named `Foo` found for enum `Bar` in the current scope"
///   - "no method named `foo` found for struct `Bar` in the current scope"
///   - "no associated item named `Foo` found for struct `Bar`"
/// Produces `"Bar::Foo"`.
fn extract_e0599(message: &str) -> Option<String> {
    let named_anchor = "named `";
    let pos = message.find(named_anchor)?;
    let rest = &message[pos + named_anchor.len()..];
    let end = rest.find('`')?;
    let ident = &rest[..end];
    let rest = &rest[end + 1..];

    // "...found for <kind> `Owner`..." — the kind word ("enum",
    // "struct", "trait", "primitive type", ...) is between "for " and
    // the opening backtick of the owner identifier.
    let for_anchor = "found for ";
    let for_pos = rest.find(for_anchor)?;
    let after_for = &rest[for_pos + for_anchor.len()..];
    let tick_pos = after_for.find('`')?;
    let owner_rest = &after_for[tick_pos + 1..];
    let owner_end = owner_rest.find('`')?;
    let owner = &owner_rest[..owner_end];

    Some(format!("{owner}::{ident}"))
}

/// E0433 phrasings:
///   - "failed to resolve: use of undeclared type `Foo`"
///   - "failed to resolve: use of undeclared crate or module `foo`"
/// Produces the backticked identifier.
fn extract_e0433(message: &str) -> Option<String> {
    for anchor in ["undeclared type `", "undeclared crate or module `"] {
        if let Some(pos) = message.find(anchor) {
            let rest = &message[pos + anchor.len()..];
            if let Some(end) = rest.find('`') {
                return Some(rest[..end].to_string());
            }
        }
    }
    None
}

// =============================================================================
// Terminal formatting
// =============================================================================

/// Render an [`Analysis`] for terminal display. Section order
/// surfaces infrastructure issues first: a reader with a broken
/// pipeline should see that before forming opinions on pass rates.
pub fn format_terminal(a: &Analysis) -> String {
    let mut s = String::new();

    // 1. Run meta (with malformed-line count).
    s.push_str("=== Bake-off analysis ===\n");
    s.push_str(&format!(
        "rows: {total}  (unsupported: {unsup}, attempted: {att}, malformed: {mal})\n",
        total = a.run_meta.total_rows,
        unsup = a.run_meta.unsupported_rows,
        att = a.run_meta.attempted_rows,
        mal = a.run_meta.malformed_rows,
    ));
    s.push_str(&format!(
        "unique cards sampled: {}\n",
        a.run_meta.cards_sampled
    ));
    if !a.run_meta.tier_counts.is_empty() {
        let parts: Vec<String> = a
            .run_meta
            .tier_counts
            .iter()
            .map(|(t, n)| format!("T{t}={n}"))
            .collect();
        s.push_str(&format!("tier buckets: {}\n", parts.join(", ")));
    }
    if !a.run_meta.models.is_empty() {
        s.push_str(&format!("models: {}\n", a.run_meta.models.join(", ")));
    }

    // 2. Infrastructure warnings (T4 anomalies).
    if !a.t4_anomalies.is_empty() {
        s.push_str("\n-- INFRA WARNINGS --\n");
        s.push_str(&format!(
            "T4 leaked into model loop ({} cards). Driver filter or prompt-coverage bug.\n",
            a.t4_anomalies.len()
        ));
        for an in &a.t4_anomalies {
            s.push_str(&format!(
                "  • {} (oracle_id={}, shape={})\n",
                an.card_name, an.card_oracle_id, an.shape
            ));
        }
    }

    // 3. Pass-rate matrix.
    s.push_str("\n-- Pass-rate matrix --\n");
    if a.pass_rate_matrix.is_empty() {
        s.push_str("  (no attempted rows)\n");
    } else {
        // Group by (model, tier), collapse attempts into one row
        // with cumulative per attempt.
        let mut grouped: BTreeMap<(String, u8), Vec<&PassRateCell>> = BTreeMap::new();
        for cell in &a.pass_rate_matrix {
            grouped
                .entry((cell.model.clone(), cell.tier))
                .or_default()
                .push(cell);
        }
        for ((model, tier), cells) in grouped {
            let denom = cells.first().map(|c| c.n_attempted).unwrap_or(0).max(1);
            let final_cum = cells
                .iter()
                .map(|c| c.n_passed_cumulative)
                .max()
                .unwrap_or(0);
            s.push_str(&format!(
                "  [{model}] T{tier}: {final_cum}/{denom} ({:.1}%) cumulative\n",
                100.0 * final_cum as f32 / denom as f32
            ));
            for cell in cells {
                s.push_str(&format!(
                    "      attempt {}: +{} this try, {} total ({:.1}% cum)\n",
                    cell.attempt_idx,
                    cell.n_passed_at_this_attempt,
                    cell.n_passed_cumulative,
                    100.0 * cell.n_passed_cumulative as f32 / denom as f32,
                ));
            }
        }
    }

    // 4. Error histogram.
    s.push_str("\n-- Error histograms --\n");
    if a.error_histograms.is_empty() {
        s.push_str("  (no candidate errors recorded)\n");
    } else {
        for h in &a.error_histograms {
            s.push_str(&format!(
                "  [{}] total errors: {}\n",
                h.model, h.total_errors
            ));
            for b in &h.buckets {
                let code = b.code.as_deref().unwrap_or("other");
                s.push_str(&format!(
                    "    {code}: {} ({:.1}%)\n",
                    b.count, b.pct_of_model_errors
                ));
                for (v, n) in &b.top_variants {
                    s.push_str(&format!("      · {v}: {n}\n"));
                }
            }
        }
    }

    // 5. Corpus-hard.
    s.push_str("\n-- Corpus-hard (0% pass, every model failed every attempt) --\n");
    if a.corpus_hard.is_empty() {
        s.push_str("  (none)\n");
    } else {
        for f in &a.corpus_hard {
            s.push_str(&format!(
                "  T{}  {}  [{} models failed-all]\n",
                f.tier,
                f.name,
                f.models_failed_all.len(),
            ));
        }
    }

    // 6. Corpus-brittle.
    s.push_str("\n-- Corpus-brittle (<50% pass, >0%) --\n");
    if a.corpus_brittle.is_empty() {
        s.push_str("  (none)\n");
    } else {
        for f in &a.corpus_brittle {
            s.push_str(&format!(
                "  T{}  {}  pass_rate={:.2}  passed={:?}  failed_all={:?}\n",
                f.tier, f.name, f.pass_rate, f.models_passed, f.models_failed_all,
            ));
        }
    }

    // 7. Model-split.
    s.push_str("\n-- Model-split (≥1 model passed, ≥1 different model failed all) --\n");
    if a.model_split.is_empty() {
        s.push_str("  (none)\n");
    } else {
        for f in &a.model_split {
            s.push_str(&format!(
                "  T{}  {}  passed={:?}  failed_all={:?}\n",
                f.tier, f.name, f.models_passed, f.models_failed_all,
            ));
        }
    }

    // 8. Latency.
    s.push_str("\n-- Latency --\n");
    for l in &a.latency {
        s.push_str(&format!(
            "  [{}] completion all: n={}, p50={}ms, p90={}ms, mean={}ms\n",
            l.model,
            l.completion_all.n,
            l.completion_all.p50_ms,
            l.completion_all.p90_ms,
            l.completion_all.mean_ms,
        ));
        s.push_str(&format!(
            "         completion passing-only: n={}, p50={}ms, p90={}ms, mean={}ms\n",
            l.completion_passing.n,
            l.completion_passing.p50_ms,
            l.completion_passing.p90_ms,
            l.completion_passing.mean_ms,
        ));
        s.push_str(&format!(
            "         verify    all: n={}, p50={}ms, p90={}ms, mean={}ms\n",
            l.verify_all.n,
            l.verify_all.p50_ms,
            l.verify_all.p90_ms,
            l.verify_all.mean_ms,
        ));
    }

    // 9. Cost.
    if !a.cost_totals.is_empty() {
        s.push_str("\n-- Cost --\n");
        for c in &a.cost_totals {
            s.push_str(&format!(
                "  [{}] total ${:.4} over {} attempts\n",
                c.model, c.total_usd, c.attempts_with_cost
            ));
        }
    }

    // 10. Unsupported.
    if !a.unsupported_by_reason.is_empty() {
        s.push_str("\n-- Unsupported (pipeline-coverage) --\n");
        for (reason, n) in &a.unsupported_by_reason {
            s.push_str(&format!("  {n:>4}  {reason}\n"));
        }
    }

    s
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn row_unsupported(
        oracle_id: &str,
        name: &str,
        tier: u8,
        reason: &str,
    ) -> String {
        format!(
            r#"{{"kind":"unsupported","timestamp":"2026-01-01T00:00:00Z","card":{{"name":"{name}","oracle_id":"{oracle_id}","set":"tst"}},"tier":{tier},"reason":"{reason}"}}"#
        )
    }

    /// Compose an AttemptedRow with fully-realized attempt entries.
    /// `attempts` is a list of (completion_ms, verify_ms, verify_result_json).
    /// `final_outcome_json` is a JSON fragment like
    /// `{"kind":"passed","at_attempt":0}`.
    fn row_attempted(
        oracle_id: &str,
        name: &str,
        tier: u8,
        shape: &str,
        model: &str,
        attempts: &[(u64, u64, &str)],
        final_outcome_json: &str,
    ) -> String {
        let atts: Vec<String> = attempts
            .iter()
            .map(|(c_ms, v_ms, vr)| {
                format!(
                    r#"{{"completion":{{"text":"","duration_ms":{c_ms},"prompt_tokens":null,"completion_tokens":null,"cost_usd":null}},"verify":{{"result":{vr},"duration_ms":{v_ms},"error_count":0}}}}"#
                )
            })
            .collect();
        format!(
            r#"{{"kind":"attempted","timestamp":"2026-01-01T00:00:00Z","card":{{"name":"{name}","oracle_id":"{oracle_id}","set":"tst"}},"tier":{tier},"shape":"{shape}","model":"{model}","prompt_render_duration_ms":1,"attempts":[{}],"final_outcome":{final_outcome_json}}}"#,
            atts.join(",")
        )
    }

    fn passed_verify() -> &'static str {
        r#"{"kind":"passed"}"#
    }

    fn failed_verify(errors_json: &str) -> String {
        format!(
            r#"{{"kind":"failed_in_candidate","errors":{errors_json}}}"#
        )
    }

    fn err_json(code: &str, message: &str) -> String {
        let escaped = message.replace('\\', "\\\\").replace('"', "\\\"");
        format!(
            r#"{{"file":"src/generated/_scratch/candidate.rs","line":1,"column":1,"level":"error","code":"{code}","message":"{escaped}"}}"#
        )
    }

    fn parse(lines: &[String]) -> Analysis {
        let mut rows = Vec::new();
        for l in lines {
            let r: JsonlRow = serde_json::from_str(l)
                .unwrap_or_else(|e| panic!("test JSONL must parse: {e}\nline={l}"));
            rows.push(r);
        }
        analyze_rows(rows, 0, &AnalyzeConfig::default())
    }

    // -- happy path -----------------------------------------------------

    #[test]
    fn happy_path_counts_are_correct() {
        let lines = vec![
            row_attempted(
                "c1",
                "Card One",
                1,
                "VanillaCreature",
                "qwen",
                &[(100, 50, passed_verify())],
                r#"{"kind":"passed","at_attempt":0}"#,
            ),
            row_attempted(
                "c2",
                "Card Two",
                2,
                "FrenchVanillaCreature",
                "qwen",
                &[(120, 60, passed_verify())],
                r#"{"kind":"passed","at_attempt":0}"#,
            ),
            row_attempted(
                "c3",
                "Card Three",
                2,
                "SingleEffectSpell",
                "qwen",
                &[(130, 70, passed_verify())],
                r#"{"kind":"passed","at_attempt":0}"#,
            ),
            row_attempted(
                "c4",
                "Card Four",
                3,
                "TriggeredAbilityCreature",
                "qwen",
                &[
                    (140, 80, &failed_verify(&format!("[{}]", err_json("E0599", "no variant named `Bamboozle` found for enum `KeywordAbility`")))),
                    (150, 90, &failed_verify(&format!("[{}]", err_json("E0599", "no variant named `Bamboozle` found for enum `KeywordAbility`")))),
                ],
                r#"{"kind":"failed_all_attempts"}"#,
            ),
            row_unsupported("c5", "Card Five", 4, "tier out of scope: T4"),
        ];
        let a = parse(&lines);

        assert_eq!(a.run_meta.total_rows, 5);
        assert_eq!(a.run_meta.attempted_rows, 4);
        assert_eq!(a.run_meta.unsupported_rows, 1);
        assert_eq!(a.run_meta.cards_sampled, 5);
        assert_eq!(a.run_meta.models, vec!["qwen"]);

        // Pass-rate matrix: qwen saw 1 T1, 2 T2, 1 T3. All T1 + T2
        // passed at attempt 0; T3 failed all 2 attempts.
        let cells: HashMap<(String, u8, usize), &PassRateCell> = a
            .pass_rate_matrix
            .iter()
            .map(|c| ((c.model.clone(), c.tier, c.attempt_idx), c))
            .collect();
        assert_eq!(cells[&("qwen".to_string(), 1, 0)].n_passed_cumulative, 1);
        assert_eq!(cells[&("qwen".to_string(), 1, 0)].n_attempted, 1);
        assert_eq!(cells[&("qwen".to_string(), 2, 0)].n_passed_cumulative, 2);
        assert_eq!(cells[&("qwen".to_string(), 2, 0)].n_attempted, 2);
        assert_eq!(cells[&("qwen".to_string(), 3, 0)].n_passed_cumulative, 0);
        assert_eq!(cells[&("qwen".to_string(), 3, 1)].n_passed_cumulative, 0);
        assert_eq!(cells[&("qwen".to_string(), 3, 0)].n_attempted, 1);

        // corpus_hard includes c4.
        assert_eq!(a.corpus_hard.len(), 1);
        assert_eq!(a.corpus_hard[0].oracle_id, "c4");
        assert!(a.corpus_hard[0].models_passed.is_empty());
        assert_eq!(a.corpus_hard[0].models_failed_all, vec!["qwen"]);

        // Error histogram for qwen: 2× E0599 with the same variant.
        assert_eq!(a.error_histograms.len(), 1);
        let h = &a.error_histograms[0];
        assert_eq!(h.model, "qwen");
        assert_eq!(h.total_errors, 2);
        assert_eq!(h.buckets[0].code.as_deref(), Some("E0599"));
        assert_eq!(h.buckets[0].count, 2);
        assert_eq!(
            h.buckets[0].top_variants,
            vec![("KeywordAbility::Bamboozle".to_string(), 2)]
        );
        assert!((h.buckets[0].pct_of_model_errors - 100.0).abs() < 0.01);

        assert_eq!(a.unsupported_by_reason, vec![("tier out of scope: T4".to_string(), 1)]);
    }

    // -- retry-with-partial-success -------------------------------------

    #[test]
    fn retry_attribution_marginal_and_cumulative() {
        // 3 cards, same model, same tier:
        //   c1 passes at attempt 0
        //   c2 passes at attempt 1
        //   c3 fails all 3 attempts
        let failed_err = failed_verify(&format!(
            "[{}]",
            err_json("E0599", "no variant named `Bogus` found for enum `Zone`")
        ));
        let lines = vec![
            row_attempted(
                "c1",
                "C1",
                2,
                "SingleEffectSpell",
                "m",
                &[(10, 10, passed_verify())],
                r#"{"kind":"passed","at_attempt":0}"#,
            ),
            row_attempted(
                "c2",
                "C2",
                2,
                "SingleEffectSpell",
                "m",
                &[(10, 10, &failed_err), (20, 10, passed_verify())],
                r#"{"kind":"passed","at_attempt":1}"#,
            ),
            row_attempted(
                "c3",
                "C3",
                2,
                "SingleEffectSpell",
                "m",
                &[(10, 10, &failed_err), (20, 10, &failed_err), (30, 10, &failed_err)],
                r#"{"kind":"failed_all_attempts"}"#,
            ),
        ];
        let a = parse(&lines);

        let cells: HashMap<usize, &PassRateCell> = a
            .pass_rate_matrix
            .iter()
            .filter(|c| c.model == "m" && c.tier == 2)
            .map(|c| (c.attempt_idx, c))
            .collect();

        assert_eq!(cells[&0].n_passed_at_this_attempt, 1);
        assert_eq!(cells[&0].n_passed_cumulative, 1);
        assert_eq!(cells[&0].n_attempted, 3);

        assert_eq!(cells[&1].n_passed_at_this_attempt, 1);
        assert_eq!(cells[&1].n_passed_cumulative, 2);
        assert_eq!(cells[&1].n_attempted, 3);

        assert_eq!(cells[&2].n_passed_at_this_attempt, 0);
        assert_eq!(cells[&2].n_passed_cumulative, 2);
        assert_eq!(cells[&2].n_attempted, 3);
    }

    // -- two-model divergence on one card --------------------------------

    #[test]
    fn two_model_divergence_appears_in_model_split() {
        let failed_err = failed_verify(&format!(
            "[{}]",
            err_json("E0433", "failed to resolve: use of undeclared type `Foo`")
        ));
        let lines = vec![
            // Model A passes on attempt 0.
            row_attempted(
                "same",
                "Same Card",
                3,
                "TriggeredAbilityCreature",
                "model_a",
                &[(10, 10, passed_verify())],
                r#"{"kind":"passed","at_attempt":0}"#,
            ),
            // Model B fails all 2 attempts.
            row_attempted(
                "same",
                "Same Card",
                3,
                "TriggeredAbilityCreature",
                "model_b",
                &[(10, 10, &failed_err), (20, 10, &failed_err)],
                r#"{"kind":"failed_all_attempts"}"#,
            ),
        ];
        let a = parse(&lines);

        // Expect exactly one split facet for "same".
        assert_eq!(a.model_split.len(), 1);
        let f = &a.model_split[0];
        assert_eq!(f.oracle_id, "same");
        assert_eq!(f.models_passed, vec!["model_a"]);
        assert_eq!(f.models_failed_all, vec!["model_b"]);

        // corpus_hard must NOT include this card — model_a passed.
        assert!(
            !a.corpus_hard.iter().any(|h| h.oracle_id == "same"),
            "model-split card must not be corpus_hard"
        );

        // pass_rate = 1 passed / 3 total = ~0.333 → corpus_brittle too.
        assert!(
            a.corpus_brittle.iter().any(|h| h.oracle_id == "same"),
            "model-split card with <50% pass_rate should also be corpus_brittle"
        );
    }

    // -- corpus_hard strictness -----------------------------------------

    #[test]
    fn corpus_hard_requires_all_models_failed() {
        // c_hard: both models fail all attempts → corpus_hard.
        // c_mixed: one model passes, one fails → NOT corpus_hard.
        let fe = failed_verify(&format!("[{}]", err_json("E0599", "x")));
        let lines = vec![
            row_attempted("c_hard", "Hard", 3, "s", "a", &[(1, 1, &fe)], r#"{"kind":"failed_all_attempts"}"#),
            row_attempted("c_hard", "Hard", 3, "s", "b", &[(1, 1, &fe)], r#"{"kind":"failed_all_attempts"}"#),
            row_attempted("c_mixed", "Mixed", 3, "s", "a", &[(1, 1, passed_verify())], r#"{"kind":"passed","at_attempt":0}"#),
            row_attempted("c_mixed", "Mixed", 3, "s", "b", &[(1, 1, &fe)], r#"{"kind":"failed_all_attempts"}"#),
        ];
        let a = parse(&lines);

        let hard_ids: Vec<&str> =
            a.corpus_hard.iter().map(|f| f.oracle_id.as_str()).collect();
        assert_eq!(hard_ids, vec!["c_hard"]);

        let split_ids: Vec<&str> =
            a.model_split.iter().map(|f| f.oracle_id.as_str()).collect();
        assert_eq!(split_ids, vec!["c_mixed"]);
    }

    // -- corpus_brittle ------------------------------------------------

    #[test]
    fn corpus_brittle_window_is_exclusive_of_zero_and_half() {
        // Rate exactly 0.5 should NOT be brittle (strict <0.5).
        // Rate 0.0 should NOT be brittle (strict >0.0).
        // Rate ~0.17 (1/6 passing) SHOULD be brittle.
        let fe = failed_verify(&format!("[{}]", err_json("E0599", "x")));

        let lines = vec![
            // c_half: pass_rate == 0.5 exactly (1/2)
            row_attempted("c_half", "Half", 3, "s", "a", &[(1, 1, passed_verify())], r#"{"kind":"passed","at_attempt":0}"#),
            row_attempted("c_half", "Half", 3, "s", "b", &[(1, 1, &fe)], r#"{"kind":"failed_all_attempts"}"#),
            // c_brittle: 1 pass out of 1+3 = 0.25
            row_attempted("c_brittle", "Brittle", 3, "s", "a", &[(1, 1, passed_verify())], r#"{"kind":"passed","at_attempt":0}"#),
            row_attempted("c_brittle", "Brittle", 3, "s", "b", &[(1, 1, &fe), (1, 1, &fe), (1, 1, &fe)], r#"{"kind":"failed_all_attempts"}"#),
            // c_zero: all fail
            row_attempted("c_zero", "Zero", 3, "s", "a", &[(1, 1, &fe)], r#"{"kind":"failed_all_attempts"}"#),
        ];
        let a = parse(&lines);
        let brittle_ids: Vec<&str> =
            a.corpus_brittle.iter().map(|f| f.oracle_id.as_str()).collect();
        assert_eq!(brittle_ids, vec!["c_brittle"]);
    }

    // -- error variant extraction ---------------------------------------

    #[test]
    fn extract_e0599_variant_from_typical_message() {
        let msg = "no variant or associated item named `Bamboozle` found for enum `KeywordAbility` in the current scope";
        assert_eq!(
            extract_variant(&Some("E0599".to_string()), msg),
            Some("KeywordAbility::Bamboozle".to_string())
        );
    }

    #[test]
    fn extract_e0599_variant_from_method_form() {
        let msg = "no method named `bananas` found for struct `Characteristics` in the current scope";
        assert_eq!(
            extract_variant(&Some("E0599".to_string()), msg),
            Some("Characteristics::bananas".to_string())
        );
    }

    #[test]
    fn extract_e0433_undeclared_type() {
        let msg = "failed to resolve: use of undeclared type `TypeLine`";
        assert_eq!(
            extract_variant(&Some("E0433".to_string()), msg),
            Some("TypeLine".to_string())
        );
    }

    #[test]
    fn extract_e0433_undeclared_module() {
        let msg = "failed to resolve: use of undeclared crate or module `imaginary`";
        assert_eq!(
            extract_variant(&Some("E0433".to_string()), msg),
            Some("imaginary".to_string())
        );
    }

    #[test]
    fn extract_returns_none_for_untargeted_code() {
        assert_eq!(
            extract_variant(
                &Some("E0308".to_string()),
                "mismatched types: expected `u32`, found `String`"
            ),
            None
        );
        assert_eq!(extract_variant(&None, "syntax error"), None);
    }

    // -- error histogram shape ------------------------------------------

    #[test]
    fn error_histogram_has_percentages_and_other_bucket() {
        // 3 errors total: 2× E0599, 1× no-code (which becomes "other").
        let mixed_errors = format!(
            r#"[{},{},{{"file":"src/generated/_scratch/candidate.rs","line":1,"column":1,"level":"error","code":null,"message":"oops"}}]"#,
            err_json("E0599", "no variant named `Foo` found for enum `Bar`"),
            err_json("E0599", "no variant named `Baz` found for enum `Bar`"),
        );
        let verify = failed_verify(&mixed_errors);
        let lines = vec![row_attempted(
            "c", "C", 3, "s", "m",
            &[(1, 1, &verify)],
            r#"{"kind":"failed_all_attempts"}"#,
        )];
        let a = parse(&lines);
        let h = &a.error_histograms[0];
        assert_eq!(h.total_errors, 3);
        // Buckets sorted DESC by count. E0599 (2) before "other"/None (1).
        assert_eq!(h.buckets[0].code.as_deref(), Some("E0599"));
        assert_eq!(h.buckets[0].count, 2);
        assert!((h.buckets[0].pct_of_model_errors - (200.0 / 3.0)).abs() < 0.1);
        assert_eq!(h.buckets[1].code, None);
        assert_eq!(h.buckets[1].count, 1);
        // E0599 sub-bucket has two distinct variants.
        let variants: Vec<&str> = h.buckets[0]
            .top_variants
            .iter()
            .map(|(v, _)| v.as_str())
            .collect();
        assert!(variants.contains(&"Bar::Foo"));
        assert!(variants.contains(&"Bar::Baz"));
    }

    // -- T4 anomaly -----------------------------------------------------

    #[test]
    fn t4_row_is_flagged_as_anomaly() {
        // T4 card that reached a model = anomaly.
        let lines = vec![row_attempted(
            "t4c",
            "Leaky T4",
            4,
            "TriggeredAbilityCreature",
            "m",
            &[(1, 1, passed_verify())],
            r#"{"kind":"passed","at_attempt":0}"#,
        )];
        let a = parse(&lines);
        assert_eq!(a.t4_anomalies.len(), 1);
        assert_eq!(a.t4_anomalies[0].card_name, "Leaky T4");
        assert_eq!(a.t4_anomalies[0].classified_tier, 4);
    }

    #[test]
    fn t4_anomaly_deduped_across_models() {
        let lines = vec![
            row_attempted("t4c", "Leaky", 4, "s", "a", &[(1, 1, passed_verify())], r#"{"kind":"passed","at_attempt":0}"#),
            row_attempted("t4c", "Leaky", 4, "s", "b", &[(1, 1, passed_verify())], r#"{"kind":"passed","at_attempt":0}"#),
        ];
        let a = parse(&lines);
        assert_eq!(a.t4_anomalies.len(), 1, "deduped by oracle_id");
    }

    // -- Malformed lines -------------------------------------------------

    #[test]
    fn malformed_lines_are_counted_and_skipped() {
        use std::io::Write as _;
        let path = std::env::temp_dir().join("arcana-analyze-malformed.jsonl");
        let mut f = std::fs::File::create(&path).expect("create tmp");
        writeln!(
            f,
            "{}",
            row_unsupported("c1", "Card One", 1, "tier out of scope: T4")
        )
        .unwrap();
        writeln!(f, "{{not valid json").unwrap();
        writeln!(f, "").unwrap(); // blank, skipped silently
        writeln!(f, r#"{{"kind":"unknown","tier":1}}"#).unwrap(); // unknown kind
        drop(f);

        let a = analyze_file(&path, &AnalyzeConfig::default()).expect("analyze");
        assert_eq!(a.run_meta.total_rows, 1);
        assert_eq!(a.run_meta.malformed_rows, 2);
    }

    // -- Unsupported tally sort -----------------------------------------

    #[test]
    fn unsupported_reasons_sorted_by_count_desc() {
        let lines = vec![
            row_unsupported("c1", "c1", 1, "alpha"),
            row_unsupported("c2", "c2", 1, "beta"),
            row_unsupported("c3", "c3", 1, "beta"),
            row_unsupported("c4", "c4", 1, "beta"),
            row_unsupported("c5", "c5", 1, "alpha"),
            row_unsupported("c6", "c6", 1, "gamma"),
        ];
        let a = parse(&lines);
        // beta 3, alpha 2, gamma 1
        assert_eq!(
            a.unsupported_by_reason,
            vec![
                ("beta".to_string(), 3),
                ("alpha".to_string(), 2),
                ("gamma".to_string(), 1),
            ]
        );
    }

    // -- Latency split --------------------------------------------------

    #[test]
    fn latency_passing_excludes_failed_attempts() {
        let fe = failed_verify(&format!("[{}]", err_json("E0599", "x")));
        let lines = vec![
            // 3 attempts total; attempts 0 + 2 failed, attempt 1 passed.
            // Wait — final_outcome must be consistent. Simpler: 2 rows.
            // Row A: failed all (3 failed attempts, each 100ms)
            row_attempted(
                "a", "A", 2, "s", "m",
                &[(100, 50, &fe), (100, 50, &fe), (100, 50, &fe)],
                r#"{"kind":"failed_all_attempts"}"#,
            ),
            // Row B: passed on attempt 0 with 50ms completion.
            row_attempted(
                "b", "B", 2, "s", "m",
                &[(50, 20, passed_verify())],
                r#"{"kind":"passed","at_attempt":0}"#,
            ),
        ];
        let a = parse(&lines);
        let l = &a.latency[0];
        // completion_all: 4 samples (three 100 + one 50). Mean = 87.
        assert_eq!(l.completion_all.n, 4);
        assert_eq!(l.completion_all.mean_ms, 87);
        // completion_passing: just the one 50ms sample.
        assert_eq!(l.completion_passing.n, 1);
        assert_eq!(l.completion_passing.p50_ms, 50);
        // verify_all: 4 samples.
        assert_eq!(l.verify_all.n, 4);
    }

    // -- Ranking for corpus_hard ---------------------------------------

    #[test]
    fn corpus_hard_ranking_prefers_more_failures_then_higher_tier() {
        // Two corpus_hard cards:
        //   cA: T2, failed by 3 models (should rank first)
        //   cB: T3, failed by 2 models
        // Ties within count: higher tier ranks first.
        let fe = failed_verify(&format!("[{}]", err_json("E0599", "x")));
        let mk = |oid: &str, name: &str, tier: u8, models: &[&str]| {
            let mut lines = Vec::new();
            for m in models {
                lines.push(row_attempted(
                    oid, name, tier, "s", m,
                    &[(1, 1, &fe)],
                    r#"{"kind":"failed_all_attempts"}"#,
                ));
            }
            lines
        };
        let mut lines = Vec::new();
        lines.extend(mk("cA", "A", 2, &["m1", "m2", "m3"]));
        lines.extend(mk("cB", "B", 3, &["m1", "m2"]));
        let a = parse(&lines);
        assert_eq!(a.corpus_hard[0].oracle_id, "cA"); // 3 > 2
        assert_eq!(a.corpus_hard[1].oracle_id, "cB");
    }

    // -- latency_stats edge cases ---------------------------------------

    #[test]
    fn latency_stats_handles_empty() {
        let mut empty: Vec<u64> = Vec::new();
        let s = latency_stats(&mut empty);
        assert_eq!(s.n, 0);
        assert_eq!(s.p50_ms, 0);
    }

    #[test]
    fn latency_stats_single_sample() {
        let mut v = vec![42u64];
        let s = latency_stats(&mut v);
        assert_eq!(s.n, 1);
        assert_eq!(s.p50_ms, 42);
        assert_eq!(s.p90_ms, 42);
        assert_eq!(s.mean_ms, 42);
    }
}
