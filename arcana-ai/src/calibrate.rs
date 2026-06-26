//! Calibrating the heuristic [`crate::search::value`] into a win PROBABILITY.
//!
//! The cockpit's eval bar shows a win% derived from the material heuristic
//! [`crate::search::value`] (roughly `[-1, 1]`, terminal `±1`). Mapping that to
//! a percentage with an arbitrary logistic scale (the original hard-coded `2.5`)
//! is just vibes — it has never been checked against what actually happens. This
//! module FITS the logistic against real self-play outcomes so the number means
//! something: at value `v`, the displayed `win_probability(v)` is the empirical
//! frequency with which a position scoring `v` is eventually won.
//!
//! # Method
//!
//! Play self-play games under a fixed policy. At a stride of decisions, record
//! the heuristic value of the (non-terminal) position from player 0's
//! perspective, and after the game append the eventual outcome (1 win / 0 loss /
//! 0.5 draw). The game is symmetric (both seats the same policy) and
//! [`crate::search::value`] is antisymmetric, so each sample `(v, y)` also yields
//! the mirror `(-v, 1 - y)` for free — doubling the data and forcing the fit to
//! be symmetric (bias ≈ 0), which a symmetric game demands.
//!
//! Then fit `p = σ(scale·v + bias)` by gradient descent on log-loss (convex), and
//! report reliability (predicted vs empirical win-rate per value bucket), log-loss
//! and Brier score — for the fitted curve and the old `2.5` constant, so the
//! improvement is visible. The fitted constants are baked into
//! [`VALUE_LOGISTIC_SCALE`] / [`VALUE_LOGISTIC_BIAS`] and consumed via
//! [`win_probability`], which is what `arcana-web`'s eval bar calls.
//!
//! # Honesty caveats (documented, not hidden)
//!
//! * Samples within one game are correlated (a stride thins but does not remove
//!   this), so the effective sample size is below the raw count.
//! * The outcome label is the result under the FIXED referee policy, not perfect
//!   play — "win probability" means "probability this position is won by THIS
//!   policy from here", a referee-relative quantity, like every self-play eval.
//! * It calibrates the hand-tuned [`crate::search::value`]; a learned value leaf
//!   would need its own fit.

use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::CardId;

use crate::search::{value, StatePolicy};

// =============================================================================
// The fitted mapping (consumed by arcana-web's eval bar)
// =============================================================================

/// Logistic SCALE mapping [`crate::search::value`] → win probability, fitted by
/// [`fit_logistic`] against self-play outcomes (see
/// `calibrate::tests::calibrate_value_to_winrate`). Replaces the original
/// arbitrary `2.5`.
///
/// Fit: 11,202 samples (perfectly symmetric 5601/5601) from the 7-deck standard
/// set under fixed ValueMc(Material), stride 8 → scale 3.22, bias ≈ 0; log-loss
/// 0.5566→0.5512 and Brier 0.1888→0.1880 vs the legacy constant, with a
/// reliability table that tracks empirical win-rate through the mid-range.
pub const VALUE_LOGISTIC_SCALE: f32 = 3.22;
/// Logistic BIAS for the value→win-probability map. ≈ 0 because the game is
/// symmetric and the value heuristic is antisymmetric (the mirror-sample fit
/// pins it at zero); kept as a named constant so a future asymmetric fit has a
/// home.
pub const VALUE_LOGISTIC_BIAS: f32 = 0.0;

/// Map a heuristic [`crate::search::value`] (`~[-1, 1]`) to a win PROBABILITY in
/// `[0, 1]` using the fitted logistic. `arcana-web` multiplies by 100 for the
/// eval bar. A calibrated estimate (see module docs), not perfect-play truth.
pub fn win_probability(value: f32) -> f32 {
    sigmoid(VALUE_LOGISTIC_SCALE * value + VALUE_LOGISTIC_BIAS)
}

#[inline]
fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}

// =============================================================================
// Sample collection
// =============================================================================

/// One calibration datum: the heuristic value of a position and the eventual
/// outcome label for the perspective that value was taken from (1 win / 0 loss /
/// 0.5 draw).
pub type Sample = (f32, f32);

/// Outcome label for player 0 from a [`GameResult`] (1 win / 0 loss / 0.5 draw).
fn label_for_p0(result: &GameResult) -> f32 {
    match result {
        GameResult::Win(0) => 1.0,
        GameResult::Win(_) => 0.0,
        GameResult::Eliminated(0) => 0.0,
        GameResult::Eliminated(_) => 1.0,
        GameResult::Draw => 0.5,
    }
}

/// Play one game, sampling the heuristic value every `stride` decisions, and
/// append `(value, outcome)` samples (each paired with its antisymmetric mirror)
/// to `out`. Both seats use `mk`-built policies seeded off `seed`.
pub fn play_and_sample(
    deck_a: &[CardId],
    deck_b: &[CardId],
    registry: &CardRegistry,
    seed: u64,
    max_steps: u32,
    stride: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    out: &mut Vec<Sample>,
) {
    let mut p0 = mk(seed.wrapping_mul(2).wrapping_add(1));
    let mut p1 = mk(seed.wrapping_mul(2).wrapping_add(2));
    let mut policies: Vec<&mut dyn StatePolicy> = vec![p0.as_mut(), p1.as_mut()];

    let (mut state, mut yld) = new_game(vec![deck_a.to_vec(), deck_b.to_vec()], registry, seed);
    let mut pending: Vec<f32> = Vec::new(); // value(state, 0) at each sampled position
    let mut steps = 0u32;
    let stride = stride.max(1);

    let result = loop {
        match yld {
            EngineYield::GameOver(r) => break r,
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                if steps >= max_steps || legal_actions.is_empty() {
                    break GameResult::Draw;
                }
                // Sample only undecided positions (terminal ±1 is trivial).
                if steps % stride == 0 && state.result.is_none() {
                    pending.push(value(&state, 0));
                }
                let action =
                    policies[player as usize].choose(&state, registry, player, &legal_actions);
                let (s, y) = step(state, action, registry);
                state = s;
                yld = y;
                steps += 1;
            }
        }
    };

    let y0 = label_for_p0(&result);
    for v0 in pending {
        out.push((v0, y0)); // player 0's perspective
        out.push((-v0, 1.0 - y0)); // player 1's perspective (value is antisymmetric)
    }
}

/// Collect calibration samples over a deck set: every unordered pair plays
/// `games_per_pair` games (seats alternated), each sampled at `stride`. Returns
/// the flat sample list.
pub fn collect_samples(
    decks: &[Vec<CardId>],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
    stride: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> Vec<Sample> {
    let mut out = Vec::new();
    let mut g = 0u64;
    for i in 0..decks.len() {
        for j in (i + 1)..decks.len() {
            for k in 0..games_per_pair {
                // Alternate which deck sits in seat 0 to cancel first-player bias.
                let (a, b) = if k % 2 == 0 { (i, j) } else { (j, i) };
                play_and_sample(
                    &decks[a], &decks[b], registry, g, max_steps, stride, mk, &mut out,
                );
                g += 1;
            }
        }
    }
    out
}

// =============================================================================
// Logistic fit + scoring
// =============================================================================

/// Fit `p = σ(scale·v + bias)` to `samples` by gradient descent on mean log-loss
/// (convex, so it converges to the global optimum). Initialized at the legacy
/// `(2.5, 0.0)`. Returns `(scale, bias)`.
pub fn fit_logistic(samples: &[Sample], iters: usize, lr: f32) -> (f32, f32) {
    let n = samples.len().max(1) as f32;
    let (mut a, mut b) = (2.5f32, 0.0f32);
    for _ in 0..iters {
        let (mut ga, mut gb) = (0.0f32, 0.0f32);
        for &(x, y) in samples {
            let d = sigmoid(a * x + b) - y;
            ga += d * x;
            gb += d;
        }
        a -= lr * ga / n;
        b -= lr * gb / n;
    }
    (a, b)
}

/// Mean log-loss (cross-entropy) of the logistic `(scale, bias)` on `samples`.
/// Lower is better. Probabilities are clamped off `{0,1}` for numeric safety.
pub fn log_loss(samples: &[Sample], scale: f32, bias: f32) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let mut s = 0.0f32;
    for &(x, y) in samples {
        let p = sigmoid(scale * x + bias).clamp(1e-6, 1.0 - 1e-6);
        s += -(y * p.ln() + (1.0 - y) * (1.0 - p).ln());
    }
    s / samples.len() as f32
}

/// Mean Brier score (squared error of the probability) of `(scale, bias)` on
/// `samples`. Lower is better.
pub fn brier(samples: &[Sample], scale: f32, bias: f32) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let mut s = 0.0f32;
    for &(x, y) in samples {
        let p = sigmoid(scale * x + bias);
        s += (p - y) * (p - y);
    }
    s / samples.len() as f32
}

/// One reliability bucket: value range `[lo, hi)`, mean predicted probability,
/// empirical win-rate, and sample count.
#[derive(Clone, Debug)]
pub struct ReliabilityBin {
    pub lo: f32,
    pub hi: f32,
    pub mean_pred: f32,
    pub empirical: f32,
    pub count: usize,
}

/// Bucket `samples` by value into `n_bins` equal-width bins over `[-1, 1]` and
/// report, per bin, the mean predicted probability (under `(scale, bias)`) vs
/// the empirical win-rate — a reliability/calibration table.
pub fn reliability(samples: &[Sample], scale: f32, bias: f32, n_bins: usize) -> Vec<ReliabilityBin> {
    let n_bins = n_bins.max(1);
    let width = 2.0 / n_bins as f32;
    let mut pred = vec![0.0f32; n_bins];
    let mut emp = vec![0.0f32; n_bins];
    let mut cnt = vec![0usize; n_bins];
    for &(x, y) in samples {
        let idx = (((x + 1.0) / width).floor() as isize).clamp(0, n_bins as isize - 1) as usize;
        pred[idx] += sigmoid(scale * x + bias);
        emp[idx] += y;
        cnt[idx] += 1;
    }
    (0..n_bins)
        .map(|i| {
            let c = cnt[i].max(1) as f32;
            ReliabilityBin {
                lo: -1.0 + width * i as f32,
                hi: -1.0 + width * (i + 1) as f32,
                mean_pred: pred[i] / c,
                empirical: emp[i] / c,
                count: cnt[i],
            }
        })
        .collect()
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deckeval::{fixed_policy, standard_deck_set};
    use crate::search::RandomStatePolicy;

    fn rnd(s: u64) -> Box<dyn StatePolicy> {
        Box::new(RandomStatePolicy::new(s))
    }

    /// On a synthetic separable set, the fit recovers a strong positive scale and
    /// ~zero bias, and log-loss beats a deliberately bad constant.
    #[test]
    fn fit_recovers_a_sane_logistic() {
        // Symmetric synthetic data: positive values mostly win, negatives lose.
        let mut samples = Vec::new();
        for k in -10..=10 {
            let v = k as f32 / 10.0;
            let y = if v > 0.0 { 1.0 } else if v < 0.0 { 0.0 } else { 0.5 };
            samples.push((v, y));
            samples.push((-v, 1.0 - y));
        }
        let (scale, bias) = fit_logistic(&samples, 5000, 0.5);
        assert!(scale > 0.5, "expected positive scale, got {scale}");
        assert!(bias.abs() < 0.2, "symmetric data should fit ~zero bias, got {bias}");
        // A fitted curve should not be worse than the legacy constant on its own data.
        assert!(log_loss(&samples, scale, bias) <= log_loss(&samples, 2.5, 0.0) + 1e-3);
    }

    /// The collection path runs end-to-end on tiny random-policy games and yields
    /// mirrored, well-formed samples.
    #[test]
    fn collect_samples_is_well_formed() {
        let reg = arcana_cards::build_catalog();
        let decks: Vec<Vec<CardId>> = standard_deck_set(&reg)
            .into_iter()
            .take(2)
            .map(|d| d.cards)
            .collect();
        let samples = collect_samples(&decks, &reg, 1, 2000, 16, &rnd);
        assert!(!samples.is_empty(), "expected some samples");
        // Mirrored append → even count.
        assert_eq!(samples.len() % 2, 0);
        for &(v, y) in &samples {
            assert!(v.is_finite() && (-1.05..=1.05).contains(&v));
            assert!(y == 0.0 || y == 0.5 || y == 1.0);
        }
        // Scoring functions are finite on real samples.
        let (scale, bias) = fit_logistic(&samples, 200, 0.3);
        assert!(scale.is_finite() && bias.is_finite());
        assert!(log_loss(&samples, scale, bias).is_finite());
        assert!(brier(&samples, scale, bias).is_finite());
    }

    /// The baked-in [`win_probability`] is a monotone map through 50% at an even
    /// position.
    #[test]
    fn win_probability_is_monotone_and_centered() {
        assert!((win_probability(0.0) - 0.5).abs() < 1e-3);
        assert!(win_probability(0.5) > win_probability(0.0));
        assert!(win_probability(-0.5) < win_probability(0.0));
        assert!(win_probability(1.0) > 0.9);
        assert!(win_probability(-1.0) < 0.1);
    }

    /// MEASUREMENT (non-asserting): fit the value→win-probability logistic on
    /// self-play under the FIXED ValueMc(Material) policy, and print the fitted
    /// constants, the reliability table, and log-loss/Brier vs the legacy `2.5`.
    /// Slow (dozens of full value-MC games); `#[ignore]`, run in release:
    ///   cargo test -p arcana-ai --release --lib \
    ///     calibrate::tests::calibrate_value_to_winrate -- --ignored --nocapture
    #[test]
    #[ignore]
    fn calibrate_value_to_winrate() {
        use std::time::Instant;
        const GAMES_PER_PAIR: u32 = 3;
        const STRIDE: u32 = 8;

        let reg = arcana_cards::build_catalog();
        let decks: Vec<Vec<CardId>> =
            standard_deck_set(&reg).into_iter().map(|d| d.cards).collect();

        let t0 = Instant::now();
        let samples = collect_samples(&decks, &reg, GAMES_PER_PAIR, 4000, STRIDE, &fixed_policy);
        let elapsed = t0.elapsed();

        let wins = samples.iter().filter(|s| s.1 == 1.0).count();
        let losses = samples.iter().filter(|s| s.1 == 0.0).count();
        let draws = samples.iter().filter(|s| s.1 == 0.5).count();
        println!(
            "Collected {} samples ({wins} win / {losses} loss / {draws} draw) from \
             {} decks, {GAMES_PER_PAIR} games/pair, stride {STRIDE}, in {:.1}s.",
            samples.len(),
            decks.len(),
            elapsed.as_secs_f32()
        );

        let (scale, bias) = fit_logistic(&samples, 20_000, 0.5);
        println!("\nFitted logistic: scale = {scale:.4}, bias = {bias:.4}");
        println!("Legacy constant: scale = 2.5000, bias = 0.0000");
        println!(
            "log-loss  fitted = {:.4}   legacy = {:.4}",
            log_loss(&samples, scale, bias),
            log_loss(&samples, 2.5, 0.0)
        );
        println!(
            "Brier     fitted = {:.4}   legacy = {:.4}",
            brier(&samples, scale, bias),
            brier(&samples, 2.5, 0.0)
        );

        println!("\nReliability (fitted) — value bucket | mean pred | empirical | n:");
        for b in reliability(&samples, scale, bias, 10) {
            println!(
                "  [{:+.1}, {:+.1})  pred {:>5.1}%  emp {:>5.1}%  n={}",
                b.lo,
                b.hi,
                b.mean_pred * 100.0,
                b.empirical * 100.0,
                b.count
            );
        }
        println!(
            "\nBake VALUE_LOGISTIC_SCALE = {scale:.2}, VALUE_LOGISTIC_BIAS = {bias:.2} \
             if they differ from the current constants."
        );
    }
}
