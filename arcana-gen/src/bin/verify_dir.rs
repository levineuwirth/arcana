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
    check, check_batch, known_good_source, n_scratch_slots, restore_known_good,
    scratch_path, workspace_root_path, write_batch_slot, VerifyConfig,
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

#[derive(Debug, Clone, Serialize)]
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
    /// `Some(k)` → batched mode, chunk size `k` (capped at the
    /// compiled slot count). One `cargo check` + one `cargo test`
    /// per chunk instead of per card.
    batch: Option<usize>,
}

fn parse_args(raw: Vec<String>) -> Result<Args> {
    let mut dir: Option<PathBuf> = None;
    let mut cards_dir: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut layer1_only = false;
    let mut batch: Option<usize> = None;
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
            "--batch" => {
                // Optional value: `--batch` (default chunk) or
                // `--batch 64`. Peek; if the next arg parses as a
                // number consume it, else use the default.
                batch = Some(match it.clone().next() {
                    Some(v) if v.parse::<usize>().is_ok() => {
                        it.next();
                        v.parse().unwrap()
                    }
                    _ => arcana_gen::verify::n_scratch_slots(),
                });
            }
            "-h" | "--help" => {
                eprintln!(
                    "Usage: verify_dir --dir <dump-dir> [--cards-dir <dir>] \
                     [--output <jsonl>] [--layer1-only]\n\n\
                     --dir          dump dir with manifest.jsonl (default cards location)\n\
                     --cards-dir    where the <idx>_<slug>.rs candidates live (default: --dir)\n\
                     --output       report JSONL (default: <dir>/verify-report.jsonl)\n\
                     --layer1-only  skip the structural (layer-2) check\n\
                     --batch [K]    batched mode: one cargo check + one cargo\n\
                     \x20              test per K-card chunk (default K = slot count).\n\
                     \x20              The throughput lever for large runs."
                );
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown argument: {other}")),
        }
    }
    let dir = dir.ok_or_else(|| anyhow!("--dir is required"))?;
    let cards_dir = cards_dir.unwrap_or_else(|| dir.clone());
    let output = output.unwrap_or_else(|| dir.join("verify-report.jsonl"));
    Ok(Args { dir, cards_dir, output, layer1_only, batch })
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
    let _restore = ScratchGuard;

    let supported: Vec<&DumpRow> = rows.iter().filter(|r| r.supported).collect();
    eprintln!(
        "verify_dir: {} supported card(s){}{}",
        supported.len(),
        if args.layer1_only { " (layer-1 only)" } else { "" },
        match args.batch {
            Some(k) => format!(" (batched, chunk={k})"),
            None => String::new(),
        },
    );

    // Both modes produce outcomes in manifest order; the tally /
    // print / serialize tail is shared.
    let outcomes: Vec<(&DumpRow, Outcome)> = match args.batch {
        Some(k) => outcomes_batched(&supported, &args, &cfg, k)?,
        None => outcomes_single(&supported, &args, &cfg)?,
    };

    let (mut not_gen, mut l1_fail, mut l2_fail, mut l3_fail, mut passed) =
        (0, 0, 0, 0, 0);
    let mut report = String::new();
    for (row, outcome) in &outcomes {
        let status = match outcome {
            Outcome::NotGenerated => {
                not_gen += 1;
                "not-generated"
            }
            Outcome::Layer1Failed { .. } => {
                l1_fail += 1;
                "L1-FAIL"
            }
            Outcome::Layer2Failed { .. } => {
                l2_fail += 1;
                "L2-FAIL"
            }
            Outcome::Layer3Failed { .. } => {
                l3_fail += 1;
                "L3-STUB"
            }
            Outcome::Passed { .. } => {
                passed += 1;
                "PASS"
            }
        };
        eprintln!("  [{status:>13}] T{} {}", row.tier, row.name);
        let line = serde_json::to_string(&ReportRow {
            idx: row.idx,
            slug: &row.slug,
            name: &row.name,
            tier: row.tier,
            shape: row.shape.as_deref(),
            outcome: outcome.clone(),
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

/// Single-slot path: one cargo invocation per card (unchanged
/// behaviour; correct but slow, fine for small runs).
fn outcomes_single<'a>(
    supported: &[&'a DumpRow],
    args: &Args,
    cfg: &VerifyConfig,
) -> Result<Vec<(&'a DumpRow, Outcome)>> {
    let mut out = Vec::with_capacity(supported.len());
    for row in supported {
        let p = args.cards_dir.join(format!("{:03}_{}.rs", row.idx, row.slug));
        let o = if !p.exists() {
            Outcome::NotGenerated
        } else {
            let src = std::fs::read_to_string(&p)
                .with_context(|| format!("reading {}", p.display()))?;
            verify_one(&src, row, cfg, args.layer1_only)?
        };
        out.push((*row, o));
    }
    Ok(out)
}

/// Batched path: resolve NotGenerated + layer-3 stubs cheaply, then
/// per chunk run ONE `cargo check` (layer 1) and ONE `cargo test`
/// (layer 2) across the whole chunk. Cargo's per-invocation cost is
/// paid once per chunk instead of once per card.
fn outcomes_batched<'a>(
    supported: &[&'a DumpRow],
    args: &Args,
    _cfg: &VerifyConfig,
    k: usize,
) -> Result<Vec<(&'a DumpRow, Outcome)>> {
    let k = k.min(n_scratch_slots()).max(1);

    // (row, Some(source) if it still needs L1/L2, resolved Outcome)
    let mut staged: Vec<(&DumpRow, Option<String>, Option<Outcome>)> =
        Vec::with_capacity(supported.len());
    for row in supported {
        let p = args.cards_dir.join(format!("{:03}_{}.rs", row.idx, row.slug));
        if !p.exists() {
            staged.push((*row, None, Some(Outcome::NotGenerated)));
            continue;
        }
        let src = std::fs::read_to_string(&p)
            .with_context(|| format!("reading {}", p.display()))?;
        if let Some(reason) = stub_reason(row.shape.as_deref(), &src) {
            staged.push((*row, None, Some(Outcome::Layer3Failed { reason })));
        } else {
            staged.push((*row, Some(src), None));
        }
    }

    let pending: Vec<usize> = staged
        .iter()
        .enumerate()
        .filter(|(_, (_, s, o))| s.is_some() && o.is_none())
        .map(|(i, _)| i)
        .collect();

    let total = pending.len();
    let mut done = 0;
    for chunk in pending.chunks(k) {
        // ---- Layer 1: one cargo check for the whole chunk ----
        let sources: Vec<String> =
            chunk.iter().map(|&si| staged[si].1.clone().unwrap()).collect();
        let rep = check_batch(&sources);
        for (j, &si) in chunk.iter().enumerate() {
            match &rep.per_slot[j] {
                VerifyResult::Passed => {} // stays pending → L2
                VerifyResult::FailedInCandidate(errs) => {
                    staged[si].2 = Some(Outcome::Layer1Failed {
                        errors: errs
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
                        "verify FailedElsewhere — arcana-cards broken outside \
                         candidates; aborting (first: {:?})",
                        errs.first()
                    ));
                }
                VerifyResult::InfrastructureError(m) => {
                    return Err(anyhow!("verify InfrastructureError: {m}"));
                }
            }
        }

        // ---- Layer 2: one cargo test for the chunk's L1-passers ----
        let passers: Vec<usize> = chunk
            .iter()
            .copied()
            .filter(|&si| staged[si].2.is_none())
            .collect();
        if args.layer1_only {
            for si in passers {
                staged[si].2 = Some(Outcome::Passed { layer2_checked: false });
            }
        } else if !passers.is_empty() {
            for (slot, &si) in passers.iter().enumerate() {
                let row = staged[si].0;
                let src = staged[si].1.as_ref().unwrap();
                let harness = render_harness(&Expected::from_row(row));
                write_batch_slot(slot, &format!("{src}\n{harness}"))
                    .context("writing L2 batch slot")?;
            }
            // Reset the remaining slots so a prior chunk's harness
            // can't run under a stale test name.
            for slot in passers.len()..n_scratch_slots() {
                write_batch_slot(slot, known_good_source())
                    .context("resetting L2 batch slot")?;
            }
            let l2 = run_l2_batch(passers.len())?;
            for (slot, &si) in passers.iter().enumerate() {
                staged[si].2 = Some(match &l2[slot] {
                    None => Outcome::Passed { layer2_checked: true },
                    Some(reason) => {
                        Outcome::Layer2Failed { mismatches: reason.clone() }
                    }
                });
            }
        }

        done += chunk.len();
        eprintln!("  …chunk done ({done}/{total} L1-pending cards verified)");
    }

    Ok(staged
        .into_iter()
        .map(|(r, _, o)| {
            (r, o.unwrap_or(Outcome::Passed { layer2_checked: false }))
        })
        .collect())
}

/// Run the chunk's layer-2 tests in ONE `cargo test`. Returns one
/// entry per L2 slot `0..m`: `None` = pass, `Some(reason)` = fail.
/// A chunk-wide build failure marks every slot failed (rare for
/// vanilla; the harness is uniform codegen over L1-clean source).
fn run_l2_batch(m: usize) -> Result<Vec<Option<String>>> {
    let out = Command::new("cargo")
        // NOT `--quiet`: cargo's -q makes libtest print one char
        // per test instead of `test <name> ... ok` lines, which
        // batch attribution parses. Compiler noise goes to stderr.
        .args([
            "test",
            "-p",
            "arcana-cards",
            "--lib",
            "generated::_scratch::candidate_",
            "--",
            "--nocapture",
        ])
        .current_dir(workspace_root_path())
        .output()
        .context("spawning cargo test for batched layer 2")?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // No per-test lines + compiler errors ⇒ the chunk's test binary
    // didn't build; can't attribute, fail the chunk conservatively.
    if !stdout.contains("test result:")
        && (stderr.contains("error[") || stderr.contains("error:"))
    {
        let tail: String = stderr.lines().rev().take(8).collect::<Vec<_>>()
            .into_iter().rev().collect::<Vec<_>>().join("\n");
        return Ok((0..m)
            .map(|_| Some(format!("chunk L2 build failed:\n{tail}")))
            .collect());
    }

    let mut res: Vec<Option<String>> = (0..m)
        .map(|_| Some("layer-2 test not found in output".to_string()))
        .collect();
    for line in stdout.lines() {
        // `test generated::_scratch::candidate_<i>::__structural::structural ... ok|FAILED`
        let Some(rest) = line.strip_prefix("test generated::_scratch::candidate_")
        else {
            continue;
        };
        let Some((num, tail)) = rest.split_once("::__structural::structural")
        else {
            continue;
        };
        let Ok(slot) = num.parse::<usize>() else { continue };
        if slot >= m {
            continue;
        }
        res[slot] = if tail.contains(" ok") {
            None
        } else {
            Some(extract_slot_failure(&stdout, slot))
        };
    }
    Ok(res)
}

/// Pull slot `i`'s panic block from libtest `--nocapture` output.
fn extract_slot_failure(stdout: &str, i: usize) -> String {
    let marker =
        format!("candidate_{i}::__structural::structural stdout ----");
    if let Some(p) = stdout.find(&marker) {
        let after = &stdout[p + marker.len()..];
        let end = after.find("\n----").unwrap_or(after.len().min(1500));
        let block = after[..end].trim();
        if let Some(q) = block.find("structural mismatches:") {
            return block[q..].trim().to_string();
        }
        return block.to_string();
    }
    "layer-2 failed (panic block not located in test output)".to_string()
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

/// Restores every scratch slot (single + all batch slots) on drop —
/// even on `?`/panic — so an aborted run never leaves a broken
/// candidate that would break every later cargo invocation in the
/// workspace.
struct ScratchGuard;
impl Drop for ScratchGuard {
    fn drop(&mut self) {
        if let Err(e) = arcana_gen::verify::restore_all_slots() {
            eprintln!("verify_dir: warn: failed to restore scratch slots: {e}");
        } else {
            eprintln!("verify_dir: all scratch slots restored to known-good.");
        }
    }
}
