//! Compilation + smoke test runner for generated card code.
//!
//! v1 is **layer 1 only** — a `cargo check` runner against the
//! `arcana-cards/src/generated/_scratch/candidate.rs` slot, with
//! structured diagnostics. Given a candidate source string, returns
//! a [`VerifyReport`] containing pass/fail outcome, parsed compile
//! errors (when applicable), wall-clock duration, and (on failure)
//! the raw cargo stderr/stdout for parser-debugging.
//!
//! # Outcome shape
//!
//! [`VerifyResult`] distinguishes four states that look identical
//! on a naive pass/fail view but tell very different stories:
//!
//! * [`VerifyResult::Passed`] — candidate compiles clean.
//! * [`VerifyResult::FailedInCandidate`] — the candidate has
//!   errors. Actionable; feeds retry-with-errors prompts.
//! * [`VerifyResult::FailedElsewhere`] — `cargo check` failed but
//!   no errors are in the candidate file. Means `arcana-cards`
//!   itself has a pre-existing issue and the verify result is
//!   **not a signal about the candidate**. Don't count it as a
//!   failure in bake-off stats.
//! * [`VerifyResult::InfrastructureError`] — cargo spawn crashed,
//!   JSON parse failed, scratch file write failed, etc. Orthogonal
//!   to candidate correctness.
//!
//! # Precheck
//!
//! Always call [`precheck`] once before a bake-off run. It writes
//! the Grizzly Bears source into the scratch slot and confirms the
//! whole pipeline (write, cargo check, parse, filter) is healthy.
//! If the precheck returns anything other than `Passed`, the
//! pipeline is broken and per-card results will be garbage.
//!
//! # Layer-2 deferral
//!
//! Layer 2 (construct a `CardRegistry`, call the candidate's
//! `register()`, assert no panic + inspect the resulting
//! `CardDefinition`) is the next commit. The non-obvious design
//! question is **fixture shape**: most cards need richer context
//! than an empty registry to exercise meaningfully (e.g., any
//! trigger needs a game state with specific objects on the
//! battlefield). The layer-2 harness decides whether fixtures are
//! per-card-generated or generic-by-shape. Not a v1 problem —
//! noted here so it surfaces when layer 2 is picked up.
//!
//! # Parallelism
//!
//! v1 serializes: one `candidate.rs` slot, one `cargo check` at a
//! time. [`VerifyConfig::scratch_slug`] is parameterized so that
//! future parallel callers can point at `candidate_1.rs`,
//! `candidate_2.rs`, etc., but today only the default `"candidate"`
//! slug has a declared module in
//! `arcana-cards/src/generated/_scratch/mod.rs`. A non-default
//! slug produces a `cargo check` failure (missing module) — the
//! migration to parallelism is (1) preallocate N slots as
//! `pub mod candidate_N` in that mod.rs, (2) add a lease pool on
//! top of `check`. No breaking API change.

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// Configuration for a verify run. Currently only the scratch slot
/// is parameterized; other knobs (cargo flags, timeout) are
/// expected to grow with real bake-off demand.
#[derive(Debug, Clone)]
pub struct VerifyConfig {
    /// Which scratch module to write into. Defaults to `"candidate"`.
    /// Non-default values require the corresponding `pub mod` in
    /// `arcana-cards/src/generated/_scratch/mod.rs` — see module docs.
    pub scratch_slug: String,
}

impl Default for VerifyConfig {
    fn default() -> Self {
        Self { scratch_slug: "candidate".to_string() }
    }
}

/// The outcome bucket for a verify run. See module docs for
/// variant semantics.
#[derive(Debug, Clone)]
pub enum VerifyResult {
    Passed,
    FailedInCandidate(Vec<CompileError>),
    FailedElsewhere(Vec<CompileError>),
    InfrastructureError(String),
}

/// One compile diagnostic extracted from `cargo check`'s JSON
/// output. Rolls up a single primary span on a single message;
/// messages with multiple primary spans yield multiple
/// `CompileError` rows.
#[derive(Debug, Clone, Serialize)]
pub struct CompileError {
    /// Span file as reported by rustc. May be workspace-relative,
    /// manifest-relative, or absolute — filter via `ends_with`.
    pub file: String,
    pub line: u32,
    pub column: u32,
    /// "error" / "warning" / "note" / "help".
    pub level: String,
    /// Rustc error code if present (`"E0425"`, `"E0308"`, …).
    pub code: Option<String>,
    pub message: String,
}

/// A verify run's full output.
#[derive(Debug, Clone)]
pub struct VerifyReport {
    pub result: VerifyResult,
    /// Wall-clock time from scratch-write through cargo exit.
    /// Useful for flagging cold-compile outliers during a
    /// bake-off sweep.
    pub duration: Duration,
    /// Raw `stdout` + `stderr` from cargo, captured when the run
    /// isn't a clean Pass. Escape hatch for "the parser disagrees
    /// with reality"; don't rely on this for normal flow.
    pub raw_output: Option<String>,
}

/// Grizzly Bears source — the canary for [`precheck`]. Loaded via
/// `include_str!` so it tracks refactors of the seed card.
const KNOWN_GOOD_CANDIDATE: &str =
    include_str!("../../arcana-cards/src/lea/grizzly_bears.rs");

/// Write a known-good (Grizzly Bears) candidate, run verify, return
/// the report. If this yields anything other than
/// [`VerifyResult::Passed`], the pipeline is broken — either
/// `arcana-cards` has a pre-existing compile error, or verify
/// itself is. Real candidate results are invalid until this
/// returns Passed.
pub fn precheck(config: &VerifyConfig) -> VerifyReport {
    check(KNOWN_GOOD_CANDIDATE, config)
}

/// Number of batch scratch slots (`candidate_0..N-1`). Single
/// source of truth lives in `arcana-cards/build.rs`; re-exposed
/// here so the batched driver can size chunks without a direct
/// dep path. The batch chunk size must be `<= n_scratch_slots()`.
pub fn n_scratch_slots() -> usize {
    arcana_cards::generated::_scratch::N_SCRATCH_SLOTS
}

/// Slot slug for batch index `i` (`candidate_0`, `candidate_1`, …).
pub fn batch_slot_slug(i: usize) -> String {
    format!("candidate_{i}")
}

/// Per-chunk result of [`check_batch`].
#[derive(Debug, Clone)]
pub struct BatchReport {
    /// One result per input source, in order. `FailedElsewhere` /
    /// `InfrastructureError` are batch-wide: if the chunk hit one,
    /// every entry carries it (the chunk's per-slot signal is
    /// invalid).
    pub per_slot: Vec<VerifyResult>,
    pub duration: Duration,
}

/// Layer-1 verify a whole chunk in ONE `cargo check`. Writes
/// `sources[i]` to `candidate_{i}.rs` and the known-good stub to
/// every higher slot (so a previous chunk's broken code can't
/// poison this compile), then runs a single `cargo check` and
/// attributes each error to its slot by file path.
///
/// This is the throughput lever for large runs: cargo's
/// per-invocation cost (process spawn, freshness check, crate
/// metadata) is paid once per chunk instead of once per card.
/// `sources.len()` must be `<= n_scratch_slots()`.
pub fn check_batch(sources: &[String]) -> BatchReport {
    let start = Instant::now();
    let n = n_scratch_slots();
    assert!(
        sources.len() <= n,
        "chunk of {} exceeds {n} scratch slots",
        sources.len()
    );

    // Write the chunk; reset all remaining slots to known-good so
    // stale candidates from a prior chunk don't add phantom errors.
    for i in 0..n {
        let path = scratch_path_for(&batch_slot_slug(i));
        let body = sources.get(i).map(|s| s.as_str()).unwrap_or(KNOWN_GOOD_CANDIDATE);
        if let Err(e) = std::fs::write(&path, body) {
            let err = VerifyResult::InfrastructureError(format!(
                "writing batch slot {}: {e}",
                path.display()
            ));
            return BatchReport {
                per_slot: vec![err; sources.len()],
                duration: start.elapsed(),
            };
        }
    }

    let output = match Command::new("cargo")
        .args(["check", "-p", "arcana-cards", "--message-format=json"])
        .current_dir(workspace_root())
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            let err =
                VerifyResult::InfrastructureError(format!("spawning cargo: {e}"));
            return BatchReport {
                per_slot: vec![err; sources.len()],
                duration: start.elapsed(),
            };
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let errors: Vec<CompileError> = parse_diagnostics(&stdout)
        .into_iter()
        .filter(|d| d.level == "error")
        .collect();

    // Bucket errors by slot. An error whose file isn't any
    // `candidate*` slot means arcana-cards itself is broken —
    // batch-wide FailedElsewhere (per-slot signal is meaningless).
    let elsewhere: Vec<CompileError> = errors
        .iter()
        .filter(|e| !e.file.contains("generated/_scratch/candidate"))
        .cloned()
        .collect();
    if !elsewhere.is_empty() && output.status.code() != Some(0) {
        return BatchReport {
            per_slot: vec![
                VerifyResult::FailedElsewhere(elsewhere);
                sources.len()
            ],
            duration: start.elapsed(),
        };
    }

    let per_slot = (0..sources.len())
        .map(|i| {
            let suffix = format!("generated/_scratch/candidate_{i}.rs");
            let mine: Vec<CompileError> = errors
                .iter()
                .filter(|e| e.file.ends_with(&suffix))
                .cloned()
                .collect();
            if mine.is_empty() {
                VerifyResult::Passed
            } else {
                VerifyResult::FailedInCandidate(mine)
            }
        })
        .collect();

    // `stderr` retained only for debugging a parser disagreement;
    // not surfaced per-slot.
    let _ = stderr;
    BatchReport { per_slot, duration: start.elapsed() }
}

/// Run layer-1 verification on `candidate_source`. See module docs
/// for outcome-shape detail.
pub fn check(candidate_source: &str, config: &VerifyConfig) -> VerifyReport {
    let start = Instant::now();

    let scratch_path = scratch_path_for(&config.scratch_slug);

    if let Err(e) = std::fs::write(&scratch_path, candidate_source) {
        return VerifyReport {
            result: VerifyResult::InfrastructureError(format!(
                "writing scratch file {}: {e}",
                scratch_path.display()
            )),
            duration: start.elapsed(),
            raw_output: None,
        };
    }

    let output = match Command::new("cargo")
        .args(["check", "-p", "arcana-cards", "--message-format=json"])
        .current_dir(workspace_root())
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return VerifyReport {
                result: VerifyResult::InfrastructureError(format!(
                    "spawning cargo: {e}"
                )),
                duration: start.elapsed(),
                raw_output: None,
            };
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    let diagnostics = parse_diagnostics(&stdout);
    let errors: Vec<CompileError> = diagnostics
        .iter()
        .filter(|d| d.level == "error")
        .cloned()
        .collect();

    // Cargo sometimes reports spans as workspace-relative
    // ("arcana-cards/src/..."), sometimes as manifest-relative
    // ("src/..."), and sometimes absolute. The suffix below matches
    // all three shapes.
    let scratch_suffix = format!(
        "src/generated/_scratch/{}.rs",
        config.scratch_slug
    );

    let result = if output.status.success() {
        VerifyResult::Passed
    } else if errors.is_empty() {
        VerifyResult::InfrastructureError(format!(
            "cargo check exited {:?} but produced no parseable errors; \
            first 2000 chars of stderr: {}",
            output.status.code(),
            truncate(&stderr, 2000),
        ))
    } else {
        let in_candidate: Vec<CompileError> = errors
            .iter()
            .filter(|e| e.file.ends_with(&scratch_suffix))
            .cloned()
            .collect();
        if in_candidate.is_empty() {
            VerifyResult::FailedElsewhere(errors)
        } else {
            VerifyResult::FailedInCandidate(in_candidate)
        }
    };

    let raw_output = match &result {
        VerifyResult::Passed => None,
        _ => Some(format!(
            "--- cargo exit {:?} ---\n\
            --- stdout (truncated) ---\n{}\n\
            --- stderr (truncated) ---\n{}",
            output.status.code(),
            truncate(&stdout, 8000),
            truncate(&stderr, 8000),
        )),
    };

    VerifyReport { result, duration: start.elapsed(), raw_output }
}

// =============================================================================
// JSON parsing
// =============================================================================

fn parse_diagnostics(stdout: &str) -> Vec<CompileError> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let Ok(msg) = serde_json::from_str::<CargoMessage>(line) else {
            continue;
        };
        if msg.reason != "compiler-message" {
            continue;
        }
        let Some(m) = msg.message else { continue };
        let code = m.code.map(|c| c.code);
        // Each primary span yields one row. Some errors (syntax
        // errors, in particular) emit exactly one primary span on
        // the offending token; others (type mismatches) can emit
        // two, one on the expected type and one on the actual.
        // Fall back to non-primary spans if there are zero primary
        // ones, because syntax errors sometimes report the span
        // without is_primary set.
        let primary_spans: Vec<&CargoSpan> =
            m.spans.iter().filter(|s| s.is_primary).collect();
        let spans_to_emit: Vec<&CargoSpan> = if primary_spans.is_empty() {
            m.spans.iter().collect()
        } else {
            primary_spans
        };
        for span in spans_to_emit {
            out.push(CompileError {
                file: span.file_name.clone(),
                line: span.line_start,
                column: span.column_start,
                level: m.level.clone(),
                code: code.clone(),
                message: m.message.clone(),
            });
        }
        // Messages with zero spans are suppressed (not actionable
        // for per-span error bucketing).
    }
    out
}

#[derive(Deserialize)]
struct CargoMessage {
    reason: String,
    #[serde(default)]
    message: Option<CargoDiagnostic>,
}

#[derive(Deserialize)]
struct CargoDiagnostic {
    message: String,
    level: String,
    #[serde(default)]
    code: Option<CargoCode>,
    #[serde(default)]
    spans: Vec<CargoSpan>,
}

#[derive(Deserialize)]
struct CargoCode {
    code: String,
}

#[derive(Deserialize)]
struct CargoSpan {
    file_name: String,
    line_start: u32,
    column_start: u32,
    is_primary: bool,
}

// =============================================================================
// path helpers
// =============================================================================

/// The scratch file path for `config`'s slot. Layer-2 callers write
/// `<candidate source> + <structural harness>` here and run
/// `cargo test`; see [`crate::structural`].
pub fn scratch_path(config: &VerifyConfig) -> PathBuf {
    scratch_path_for(&config.scratch_slug)
}

/// Restore the scratch slot to the known-good (Grizzly Bears)
/// source. Callers that write candidates into the slot must call
/// this when done so an aborted run doesn't leave the workspace
/// with a broken — or `_noop`-bootstrap — scratch file. Grizzly
/// Bears (not the build.rs `_noop` stub) is the restore target
/// because it exposes `register`, keeping any later workspace build
/// that references the slot healthy.
pub fn restore_known_good(config: &VerifyConfig) -> std::io::Result<()> {
    std::fs::write(scratch_path_for(&config.scratch_slug), KNOWN_GOOD_CANDIDATE)
}

/// Workspace root (parent of `arcana-gen`). Exposed so layer-2
/// callers can spawn `cargo test` with the right `current_dir`.
pub fn workspace_root_path() -> PathBuf {
    workspace_root()
}

/// The known-good (Grizzly Bears) source. Batched callers reset
/// unused slots to this so a prior chunk's code can't add phantom
/// diagnostics, and it has a real `register` (unlike the `_noop`
/// bootstrap stub) so any later workspace build stays healthy.
pub fn known_good_source() -> &'static str {
    KNOWN_GOOD_CANDIDATE
}

/// Write `source` into batch slot `i` (`candidate_{i}.rs`). Used by
/// the batched layer-2 path to stage a chunk before one
/// `cargo test`.
pub fn write_batch_slot(i: usize, source: &str) -> std::io::Result<()> {
    std::fs::write(scratch_path_for(&batch_slot_slug(i)), source)
}

/// Restore the single slot and every batch slot to the known-good
/// stub. Batched callers must call this on exit (RAII) so an
/// aborted run never leaves a broken slot — which would break every
/// later cargo invocation in the workspace.
pub fn restore_all_slots() -> std::io::Result<()> {
    std::fs::write(scratch_path_for("candidate"), KNOWN_GOOD_CANDIDATE)?;
    for i in 0..n_scratch_slots() {
        std::fs::write(
            scratch_path_for(&batch_slot_slug(i)),
            KNOWN_GOOD_CANDIDATE,
        )?;
    }
    Ok(())
}

pub(crate) fn scratch_path_for(slug: &str) -> PathBuf {
    let mut p = workspace_root();
    p.push("arcana-cards");
    p.push("src");
    p.push("generated");
    p.push("_scratch");
    p.push(format!("{slug}.rs"));
    p
}

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is arcana-gen's own dir; workspace root is
    // its parent. Stable across release/debug and across the
    // arcana-gen binary vs its lib context.
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
        .parent()
        .expect("arcana-gen must live under a workspace root")
        .to_path_buf()
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…(truncated at {} chars)", &s[..max], max)
    }
}

// =============================================================================
// round-trip tests
// =============================================================================
//
// All tests in this module spawn `cargo check`, so they're `#[ignore]`d
// to keep the default `cargo test` run fast. Run manually with:
//
//   cargo test -p arcana-gen --lib verify:: -- --ignored --test-threads=1
//
// `--test-threads=1` is load-bearing: the verify tests share the single
// `_scratch/candidate.rs` slot (see module docs on parallelism), so
// concurrent test runs step on each other. Once slot preallocation lands
// this can drop.

#[cfg(test)]
mod tests {
    use super::*;

    /// RAII guard: on drop, overwrites `_scratch/candidate.rs` with
    /// the known-good stub. Every ignored test grabs one so that a
    /// panicking test doesn't leave the worktree with a broken
    /// scratch file (which would then break every subsequent cargo
    /// invocation in the workspace — very annoying to diagnose).
    struct RestoreScratch;
    impl Drop for RestoreScratch {
        fn drop(&mut self) {
            let _ = std::fs::write(
                scratch_path_for("candidate"),
                KNOWN_GOOD_CANDIDATE,
            );
        }
    }

    #[test]
    #[ignore]
    fn precheck_passes_on_clean_workspace() {
        let _guard = RestoreScratch;
        // Canary: Grizzly Bears source written into the scratch
        // slot compiles. If this fails, either arcana-cards is
        // broken or verify's plumbing is broken — either way,
        // every real verify result in this session is invalid.
        let report = precheck(&VerifyConfig::default());
        match &report.result {
            VerifyResult::Passed => {}
            other => panic!(
                "precheck did not pass: {other:?}\nraw: {}",
                report.raw_output.as_deref().unwrap_or("<none>"),
            ),
        }
        // Sanity: duration was measured.
        assert!(report.duration.as_millis() > 0, "duration must be positive");
    }

    #[test]
    #[ignore]
    fn missing_import_reports_e0433_in_candidate() {
        let _guard = RestoreScratch;
        // TypeLine referenced without the `use` line importing it.
        let src = r#"
            use arcana_core::mana::ManaCost;
            use arcana_core::objects::Characteristics;
            use arcana_core::registry::{CardDefinition, CardRegistry};
            use arcana_core::types::{CardId, ColorSet};

            pub fn register(reg: &mut CardRegistry) -> CardId {
                let name = reg.interner_mut().intern("Test Card");
                let chars = Characteristics {
                    name,
                    mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
                    colors: ColorSet::green(),
                    types: TypeLine::CREATURE.into(),
                    ..Default::default()
                };
                reg.register(CardDefinition::new(name, chars))
            }
        "#;
        let report = check(src, &VerifyConfig::default());
        match &report.result {
            VerifyResult::FailedInCandidate(errors) => {
                assert!(
                    errors.iter().any(|e| e.code.as_deref() == Some("E0433")),
                    "expected E0433 (undeclared type) in candidate; got: {errors:#?}\nraw: {}",
                    report.raw_output.as_deref().unwrap_or("<none>"),
                );
            }
            other => panic!("expected FailedInCandidate, got {other:?}"),
        }
    }

    #[test]
    #[ignore]
    fn invented_variant_reports_e0599_in_candidate() {
        let _guard = RestoreScratch;
        // A clearly-invented variant. This is the single most common
        // failure mode we expect from generated cards: the model
        // confidently invokes variants that don't exist. `Bamboozle`
        // isn't a real MTG keyword and has no chance of sneaking
        // into the enum — if future engine work adds it, the test
        // breaks loudly and we pick a different fake.
        let src = r#"
            use arcana_core::effects::KeywordAbility;
            use arcana_core::mana::ManaCost;
            use arcana_core::objects::Characteristics;
            use arcana_core::registry::{CardDefinition, CardRegistry};
            use arcana_core::types::{CardId, ColorSet, PtValue, TypeLine};

            pub fn register(reg: &mut CardRegistry) -> CardId {
                let name = reg.interner_mut().intern("Hoax Creature");
                let chars = Characteristics {
                    name,
                    mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
                    colors: ColorSet::blue(),
                    types: TypeLine::CREATURE.into(),
                    power: Some(PtValue::Fixed(1)),
                    toughness: Some(PtValue::Fixed(1)),
                    keywords: vec![KeywordAbility::Bamboozle],
                    ..Default::default()
                };
                reg.register(CardDefinition::new(name, chars))
            }
        "#;
        let report = check(src, &VerifyConfig::default());
        match &report.result {
            VerifyResult::FailedInCandidate(errors) => {
                assert!(
                    errors.iter().any(|e| e.code.as_deref() == Some("E0599")),
                    "expected E0599 (no variant) in candidate; got: {errors:#?}\nraw: {}",
                    report.raw_output.as_deref().unwrap_or("<none>"),
                );
            }
            other => panic!("expected FailedInCandidate, got {other:?}"),
        }
    }

    #[test]
    #[ignore]
    fn wrong_field_shape_reports_e0308_in_candidate() {
        let _guard = RestoreScratch;
        // `mana_cost` field expects `Option<ManaCost>`; `5` is an
        // integer literal. Classic model-hallucination failure.
        let src = r#"
            use arcana_core::objects::Characteristics;
            use arcana_core::registry::{CardDefinition, CardRegistry};
            use arcana_core::types::{CardId, ColorSet, TypeLine};

            pub fn register(reg: &mut CardRegistry) -> CardId {
                let name = reg.interner_mut().intern("Bad Field");
                let chars = Characteristics {
                    name,
                    mana_cost: 5,
                    colors: ColorSet::white(),
                    types: TypeLine::INSTANT.into(),
                    ..Default::default()
                };
                reg.register(CardDefinition::new(name, chars))
            }
        "#;
        let report = check(src, &VerifyConfig::default());
        match &report.result {
            VerifyResult::FailedInCandidate(errors) => {
                assert!(
                    errors.iter().any(|e| e.code.as_deref() == Some("E0308")),
                    "expected E0308 (type mismatch) in candidate; got: {errors:#?}\nraw: {}",
                    report.raw_output.as_deref().unwrap_or("<none>"),
                );
            }
            other => panic!("expected FailedInCandidate, got {other:?}"),
        }
    }

    #[test]
    #[ignore]
    fn syntax_error_reports_failure_in_candidate() {
        let _guard = RestoreScratch;
        // Unbalanced braces — a fundamentally different parse path
        // than the three errors above. Surfaces as a syntax-level
        // diagnostic (no rustc error code, just "expected ..." or
        // "unexpected ..." messages). Rare in production (~1-2% of
        // failures) but high-value to know the parser handles it.
        let src = r#"
            use arcana_core::objects::Characteristics;
            use arcana_core::registry::{CardDefinition, CardRegistry};
            use arcana_core::types::CardId;

            pub fn register(reg: &mut CardRegistry) -> CardId {
                let name = reg.interner_mut().intern("Missing Brace");
                let chars = Characteristics {
                    name,
                    // intentionally unbalanced — no closing brace
                reg.register(CardDefinition::new(name, chars))
            }
        "#;
        let report = check(src, &VerifyConfig::default());
        match &report.result {
            VerifyResult::FailedInCandidate(errors) => {
                // Syntax errors don't always carry an error code
                // (many are emitted at level "error" with code:
                // null), so we assert on the file location instead.
                assert!(!errors.is_empty(), "expected at least one error");
                assert!(
                    errors.iter().all(|e| e.level == "error"),
                    "all diagnostics should be errors; got: {errors:#?}"
                );
            }
            other => panic!("expected FailedInCandidate, got {other:?}"),
        }
    }

    #[test]
    fn workspace_root_resolves() {
        // Non-ignored: no cargo spawn. Just confirms the path
        // helper's assumption (arcana-gen lives directly under the
        // workspace root) holds, so the ignored tests have a
        // coherent place to write to.
        let p = workspace_root();
        assert!(
            p.join("arcana-cards").join("Cargo.toml").exists(),
            "workspace_root() should contain arcana-cards/Cargo.toml; got {}",
            p.display()
        );
    }

    #[test]
    fn truncate_respects_max() {
        assert_eq!(truncate("short", 100), "short");
        let long = "x".repeat(1000);
        let t = truncate(&long, 100);
        assert!(t.len() > 100, "truncate appends a suffix indicator");
        assert!(t.starts_with(&"x".repeat(100)));
        assert!(t.contains("truncated"));
    }
}
