//! `cargo run -p arcana-gen --bin verify_dir` — batch two-layer
//! verification of candidate card sources generated out-of-process
//! (the subagent backend; see `bakeoff --dump-prompts`).
//!
//! Input: a dump dir containing `manifest.jsonl` (rows from
//! `bakeoff::dump_prompts`) and, for each supported row, a candidate
//! `<idx:03>_<slug>.rs` the generator wrote next to its prompt.
//!
//! For each candidate:
//!   * **Layer 1** — `verify::check` (`cargo check -p arcana-cards`
//!     against the scratch slot). Pass/fail + structured errors.
//!   * **Layer 2** — only if layer 1 passed: append the codegen'd
//!     structural harness (`arcana_gen::structural`) to the source,
//!     `cargo test` it, and diff the registered `CardDefinition`
//!     against the Scryfall row. Pass/fail + the mismatch list.
//!
//! Serialized by construction: one scratch slot, one cargo
//! invocation at a time (cargo holds a workspace lock anyway).
//! Generation is the parallel/expensive step; verification is a
//! sequential funnel. The scratch slot is restored to the
//! known-good stub on exit, including on abort.
//!
//! ```text
//! cargo run -p arcana-gen --bin verify_dir --release -- \
//!     --dir target/cardgen/pilot01
//! ```

use std::path::PathBuf;
use std::process::{Command, ExitCode};

use anyhow::{anyhow, Context, Result};
use arcana_gen::bakeoff::DumpRow;
use arcana_gen::semantic::stub_reason;
use arcana_gen::structural::{render_harness, Expected};
use arcana_gen::verify::{
    check, restore_known_good, scratch_path, workspace_root_path, VerifyConfig,
    VerifyResult,
};
use serde::Serialize;

fn main() -> ExitCode {
    match real_main() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("verify_dir: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
enum Outcome {
    /// No candidate `.rs` file was written for this supported row.
    NotGenerated,
    /// Layer 1 failed: candidate doesn't compile.
    Layer1Failed { errors: Vec<String> },
    /// Compiled, but the structural fingerprint diverged.
    Layer2Failed { mismatches: String },
    /// Compiled & structurally fine, but the resolver is a stub —
    /// rules text not implemented (no `Effect::` constructed).
    Layer3Failed { reason: String },
    /// Compiled, structurally matches Scryfall, not a stub.
    Passed { layer2_checked: bool },
}

#[derive(Debug, Serialize)]
struct ReportRow<'a> {
    idx: usize,
    slug: &'a str,
    name: &'a str,
    tier: u8,
    shape: Option<&'a str>,
    #[serde(flatten)]
    outcome: Outcome,
}

struct Args {
    dir: PathBuf,
    cards_dir: PathBuf,
    output: PathBuf,
    layer1_only: bool,
}

fn parse_args(raw: Vec<String>) -> Result<Args> {
    let mut dir: Option<PathBuf> = None;
    let mut cards_dir: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut layer1_only = false;
    let mut it = raw.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--dir" => {
                dir = Some(PathBuf::from(
                    it.next().ok_or_else(|| anyhow!("--dir needs a value"))?,
                ))
            }
            "--cards-dir" => {
                cards_dir = Some(PathBuf::from(
                    it.next().ok_or_else(|| anyhow!("--cards-dir needs a value"))?,
                ))
            }
            "--output" => {
                output = Some(PathBuf::from(
                    it.next().ok_or_else(|| anyhow!("--output needs a value"))?,
                ))
            }
            "--layer1-only" => layer1_only = true,
            "-h" | "--help" => {
                eprintln!(
                    "Usage: verify_dir --dir <dump-dir> [--cards-dir <dir>] \
                     [--output <jsonl>] [--layer1-only]\n\n\
                     --dir          dump dir with manifest.jsonl (default cards location)\n\
                     --cards-dir    where the <idx>_<slug>.rs candidates live (default: --dir)\n\
                     --output       report JSONL (default: <dir>/verify-report.jsonl)\n\
                     --layer1-only  skip the structural (layer-2) check"
                );
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown argument: {other}")),
        }
    }
    let dir = dir.ok_or_else(|| anyhow!("--dir is required"))?;
    let cards_dir = cards_dir.unwrap_or_else(|| dir.clone());
    let output = output.unwrap_or_else(|| dir.join("verify-report.jsonl"));
    Ok(Args { dir, cards_dir, output, layer1_only })
}

fn real_main() -> Result<bool> {
    let args = parse_args(std::env::args().skip(1).collect())?;
    let cfg = VerifyConfig::default();

    let manifest_path = args.dir.join("manifest.jsonl");
    let manifest = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("reading {}", manifest_path.display()))?;
    let rows: Vec<DumpRow> = manifest
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .context("parsing manifest.jsonl")?;

    // Precheck: known-good through layer 1. If this isn't Passed,
    // arcana-cards or verify is broken and every result is noise.
    eprintln!("verify_dir: precheck (known-good canary)…");
    match arcana_gen::verify::precheck(&cfg).result {
        VerifyResult::Passed => {}
        other => {
            let _ = restore_known_good(&cfg);
            return Err(anyhow!(
                "precheck did not pass — arcana-cards/verify broken: {other:?}"
            ));
        }
    }

    // Restore the scratch slot no matter how we leave.
    let _restore = ScratchGuard(&cfg);

    let mut report = String::new();
    let supported: Vec<&DumpRow> = rows.iter().filter(|r| r.supported).collect();
    eprintln!(
        "verify_dir: {} supported card(s) in manifest{}",
        supported.len(),
        if args.layer1_only { " (layer-1 only)" } else { "" }
    );

    let (mut not_gen, mut l1_fail, mut l2_fail, mut l3_fail, mut passed) =
        (0, 0, 0, 0, 0);

    for row in &supported {
        let cand_path = args
            .cards_dir
            .join(format!("{:03}_{}.rs", row.idx, row.slug));
        let outcome = if !cand_path.exists() {
            not_gen += 1;
            Outcome::NotGenerated
        } else {
            let source = std::fs::read_to_string(&cand_path)
                .with_context(|| format!("reading {}", cand_path.display()))?;
            match verify_one(&source, row, &cfg, args.layer1_only)? {
                o @ Outcome::Layer1Failed { .. } => {
                    l1_fail += 1;
                    o
                }
                o @ Outcome::Layer2Failed { .. } => {
                    l2_fail += 1;
                    o
                }
                o @ Outcome::Layer3Failed { .. } => {
                    l3_fail += 1;
                    o
                }
                o @ Outcome::Passed { .. } => {
                    passed += 1;
                    o
                }
                o => o,
            }
        };
        let status = match &outcome {
            Outcome::NotGenerated => "not-generated",
            Outcome::Layer1Failed { .. } => "L1-FAIL",
            Outcome::Layer2Failed { .. } => "L2-FAIL",
            Outcome::Layer3Failed { .. } => "L3-STUB",
            Outcome::Passed { .. } => "PASS",
        };
        eprintln!("  [{status:>13}] T{} {}", row.tier, row.name);
        let line = serde_json::to_string(&ReportRow {
            idx: row.idx,
            slug: &row.slug,
            name: &row.name,
            tier: row.tier,
            shape: row.shape.as_deref(),
            outcome,
        })
        .context("serializing report row")?;
        report.push_str(&line);
        report.push('\n');
    }

    std::fs::write(&args.output, &report)
        .with_context(|| format!("writing {}", args.output.display()))?;

    let n = supported.len().max(1);
    eprintln!("\n=== verify_dir summary ===");
    eprintln!("  supported cards:   {}", supported.len());
    eprintln!("  not generated:     {not_gen}");
    eprintln!(
        "  layer-1 fail:      {l1_fail}  ({:.0}% of generated)",
        pct(l1_fail, supported.len() - not_gen),
    );
    eprintln!("  layer-2 fail:      {l2_fail}");
    eprintln!(
        "  layer-3 stub:      {l3_fail}  (compiles + bones OK, rules unimplemented)"
    );
    eprintln!(
        "  passed:            {passed}  ({:.0}% of supported, {:.0}% of generated)",
        pct(passed, n),
        pct(passed, supported.len() - not_gen),
    );
    eprintln!("  report:            {}", args.output.display());

    Ok(l1_fail == 0 && l2_fail == 0 && l3_fail == 0 && not_gen == 0)
}

fn pct(n: usize, d: usize) -> f64 {
    if d == 0 {
        0.0
    } else {
        100.0 * n as f64 / d as f64
    }
}

fn verify_one(
    source: &str,
    row: &DumpRow,
    cfg: &VerifyConfig,
    layer1_only: bool,
) -> Result<Outcome> {
    // Layer 1: compile the source alone (no #[cfg(test)] harness,
    // so `cargo check` measures exactly what the model emitted).
    let l1 = check(source, cfg);
    match l1.result {
        VerifyResult::Passed => {}
        VerifyResult::FailedInCandidate(errors) => {
            return Ok(Outcome::Layer1Failed {
                errors: errors
                    .iter()
                    .map(|e| {
                        format!(
                            "{}:{} [{}] {}",
                            e.line,
                            e.column,
                            e.code.as_deref().unwrap_or("-"),
                            e.message
                        )
                    })
                    .collect(),
            });
        }
        VerifyResult::FailedElsewhere(errs) => {
            return Err(anyhow!(
                "verify FailedElsewhere — arcana-cards is broken outside the \
                 candidate; aborting (first error: {:?})",
                errs.first()
            ));
        }
        VerifyResult::InfrastructureError(m) => {
            return Err(anyhow!("verify InfrastructureError: {m}"));
        }
    }

    // Layer 3: cheap source heuristic — a shape that must implement
    // rules text but constructs no `Effect::` is a stub. Run before
    // the expensive layer-2 cargo test so we don't compile a stub.
    if let Some(reason) = stub_reason(row.shape.as_deref(), source) {
        return Ok(Outcome::Layer3Failed { reason });
    }

    if layer1_only {
        return Ok(Outcome::Passed { layer2_checked: false });
    }

    // Layer 2: append the structural harness and `cargo test` it.
    let harness = render_harness(&Expected::from_row(row));
    let combined = format!("{source}\n{harness}");
    std::fs::write(scratch_path(cfg), &combined)
        .context("writing layer-2 scratch")?;

    let out = Command::new("cargo")
        .args([
            "test",
            "-p",
            "arcana-cards",
            "--lib",
            "--quiet",
            "generated::_scratch::candidate::__structural::structural",
            "--",
            "--exact",
            "--nocapture",
        ])
        .current_dir(workspace_root_path())
        .output()
        .context("spawning cargo test for layer 2")?;

    if out.status.success() {
        Ok(Outcome::Passed { layer2_checked: true })
    } else {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        Ok(Outcome::Layer2Failed {
            mismatches: extract_mismatches(&stdout, &stderr),
        })
    }
}

/// Pull the `structural mismatches:` panic block out of the test
/// output. Falls back to a stderr tail if the panic shape changes
/// (e.g. the candidate's `register` itself panicked).
fn extract_mismatches(stdout: &str, stderr: &str) -> String {
    if let Some(i) = stdout.find("structural mismatches:") {
        let tail = &stdout[i..];
        let end = tail.find("\nnote:").unwrap_or(tail.len().min(2000));
        return tail[..end].trim().to_string();
    }
    let combined = format!("{stdout}\n{stderr}");
    let tail = combined
        .lines()
        .filter(|l| {
            l.contains("panicked")
                || l.contains("error[")
                || l.contains("structural")
        })
        .take(20)
        .collect::<Vec<_>>()
        .join("\n");
    if tail.is_empty() {
        "layer-2 cargo test failed with no recognizable panic (see report dir)"
            .to_string()
    } else {
        tail
    }
}

/// Restores the scratch slot on drop — even on `?`/panic — so an
/// aborted run never leaves a broken candidate that would break
/// every later cargo invocation in the workspace.
struct ScratchGuard<'a>(&'a VerifyConfig);
impl Drop for ScratchGuard<'_> {
    fn drop(&mut self) {
        if let Err(e) = restore_known_good(self.0) {
            eprintln!("verify_dir: warn: failed to restore scratch slot: {e}");
        } else {
            eprintln!("verify_dir: scratch slot restored to known-good.");
        }
    }
}
