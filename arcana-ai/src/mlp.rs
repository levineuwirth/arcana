//! RL P3c: a small NONLINEAR value function — a one-hidden-layer MLP learned
//! from self-play, the rung past the linear [`crate::learn::LinearValue`].
//!
//! The linear logistic value (123 card-aware features) beats random as a
//! [`crate::search::ValueMcPolicy`] leaf but still loses to the hand-tuned
//! [`crate::search::MaterialValue`]. Hypothesis: the linear model is the
//! ceiling, so a nonlinear MLP should close the gap. This module builds the MLP
//! and the measurement that decides it.
//!
//! Pipeline — identical to [`crate::learn`] except the model:
//! 1. [`collect_value_data`] labels self-play states with the eventual outcome
//!    (win=1 / draw=0.5 / loss=0), both perspectives.
//! 2. [`standardize`] zero-means / unit-vars each feature.
//! 3. [`train_mlp`] fits `123 → H → 1` (tanh hidden, sigmoid output) by
//!    full-batch gradient descent on binary cross-entropy with L2 — manual
//!    forward + backprop, no external ML stack.
//! 4. [`MlpValue`] wraps it as a [`ValueFn`]: re-encode + standardize a state and
//!    return `2·σ(net(x)) − 1` in [-1, 1] (terminal result overrides to ±1).
//!
//! Backprop correctness is the main risk; `mlp_learns_xor` is the proof — it
//! learns XOR, which a linear model provably cannot.

use arcana_core::registry::CardRegistry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::types::{CardId, PlayerId};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::learn::{collect_value_data, standardize};
use crate::observation::{BasicE2Encoder, Encoder};
use crate::search::{StatePolicy, ValueFn};

fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}

/// The trained weights of a one-hidden-layer MLP: input `d` → hidden `h` (tanh)
/// → 1 (sigmoid). Weights are flat `Vec<f32>` (`w1` is row-major `[h][d]`, i.e.
/// `w1[i*d + j]`). Kept separate from [`MlpValue`] so the math is testable
/// without a [`GameState`] (see `mlp_learns_xor`).
#[derive(Clone)]
pub struct MlpParams {
    /// Input dimension.
    pub d: usize,
    /// Hidden width.
    pub h: usize,
    /// Hidden weights, row-major `[h][d]` (`w1[i*d + j]`).
    pub w1: Vec<f32>,
    /// Hidden biases `[h]`.
    pub b1: Vec<f32>,
    /// Output weights `[h]`.
    pub w2: Vec<f32>,
    /// Output bias.
    pub b2: f32,
}

impl MlpParams {
    /// Forward pass on an already-standardized input `x` (length `d`): returns
    /// the sigmoid output `σ(net(x)) ∈ [0, 1]` (a win probability).
    pub fn forward(&self, x: &[f32]) -> f32 {
        debug_assert_eq!(x.len(), self.d, "input length must equal d");
        let mut z2 = self.b2;
        for i in 0..self.h {
            let base = i * self.d;
            let mut z1 = self.b1[i];
            for j in 0..self.d {
                z1 += self.w1[base + j] * x[j];
            }
            z2 += self.w2[i] * z1.tanh();
        }
        sigmoid(z2)
    }
}

/// Fit a `d → hidden → 1` MLP (tanh hidden, sigmoid output) by full-batch
/// gradient descent on binary cross-entropy with L2, mirroring
/// [`crate::learn::train_logistic`]. `x` is assumed already standardized.
///
/// Gradients (BCE + sigmoid output, tanh hidden):
/// * `dL/dz2 = p − y`
/// * `dL/dw2[i] = (p−y)·a1[i]`,  `dL/db2 = (p−y)`
/// * `dL/dz1[i] = (p−y)·w2[i]·(1 − a1[i]²)`  (tanh derivative `1 − a1²`)
/// * `dL/dw1[i][j] = dL/dz1[i]·x[j]`,  `dL/db1[i] = dL/dz1[i]`
///
/// `seed` drives the symmetric-breaking weight init (tanh nets are stuck at a
/// saddle if all hidden units start equal).
#[allow(clippy::too_many_arguments)]
pub fn train_mlp(
    x: &[Vec<f32>],
    y: &[f32],
    hidden: usize,
    epochs: usize,
    lr: f32,
    l2: f32,
    seed: u64,
) -> MlpParams {
    let n = x.len();
    assert!(n > 0 && n == y.len(), "need matching non-empty x/y");
    assert!(hidden > 0, "need at least one hidden unit");
    let d = x[0].len();
    let h = hidden;

    // Symmetry-breaking init: small uniform weights scaled by fan-in (Xavier-ish
    // so tanh starts in its near-linear region rather than saturated).
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let s1 = (1.0 / d as f32).sqrt();
    let s2 = (1.0 / h as f32).sqrt();
    let mut w1: Vec<f32> = (0..h * d).map(|_| (rng.gen::<f32>() * 2.0 - 1.0) * s1).collect();
    let mut b1 = vec![0.0f32; h];
    let mut w2: Vec<f32> = (0..h).map(|_| (rng.gen::<f32>() * 2.0 - 1.0) * s2).collect();
    let mut b2 = 0.0f32;

    let scale = lr / n as f32;
    let mut a1 = vec![0.0f32; h]; // reused per-sample activation buffer
    for _ in 0..epochs {
        let mut gw1 = vec![0.0f32; h * d];
        let mut gb1 = vec![0.0f32; h];
        let mut gw2 = vec![0.0f32; h];
        let mut gb2 = 0.0f32;

        for (xi, &yi) in x.iter().zip(y) {
            // Forward.
            let mut z2 = b2;
            for i in 0..h {
                let base = i * d;
                let mut z1 = b1[i];
                for j in 0..d {
                    z1 += w1[base + j] * xi[j];
                }
                let a = z1.tanh();
                a1[i] = a;
                z2 += w2[i] * a;
            }
            let p = sigmoid(z2);

            // Backward.
            let dz2 = p - yi; // dL/dz2 for BCE with sigmoid output
            gb2 += dz2;
            for i in 0..h {
                gw2[i] += dz2 * a1[i];
                let da1 = dz2 * w2[i];
                let dz1 = da1 * (1.0 - a1[i] * a1[i]); // tanh' = 1 − a²
                gb1[i] += dz1;
                let base = i * d;
                for j in 0..d {
                    gw1[base + j] += dz1 * xi[j];
                }
            }
        }

        // Gradient step with L2 weight decay (biases unregularized), matching
        // train_logistic's `w -= scale·grad + lr·l2·w`.
        for k in 0..h * d {
            w1[k] -= scale * gw1[k] + lr * l2 * w1[k];
        }
        for i in 0..h {
            b1[i] -= scale * gb1[i];
            w2[i] -= scale * gw2[i] + lr * l2 * w2[i];
        }
        b2 -= scale * gb2;
    }

    MlpParams { d, h, w1, b1, w2, b2 }
}

/// A self-play-learned NONLINEAR value: a one-hidden-layer MLP over
/// [`BasicE2Encoder`] features. Implements [`ValueFn`] so it drops into
/// [`crate::search::ValueMcPolicy`] (or any value-based policy). `Clone` so one
/// trained model can seed a fresh policy per game in a tournament.
#[derive(Clone)]
pub struct MlpValue {
    encoder: BasicE2Encoder,
    means: Vec<f32>,
    stds: Vec<f32>,
    params: MlpParams,
}

impl MlpValue {
    /// Win probability for `player` in `[0, 1]` (no terminal override).
    pub fn win_prob(&self, state: &GameState, player: PlayerId) -> f32 {
        let f = self.encoder.encode(state, Some(player));
        let mut x = vec![0.0f32; f.len()];
        for j in 0..f.len() {
            x[j] = (f[j] - self.means[j]) / self.stds[j];
        }
        self.params.forward(&x)
    }
}

impl ValueFn for MlpValue {
    fn value(&self, state: &GameState, player: PlayerId) -> f32 {
        match state.result {
            Some(GameResult::Win(p)) => if p == player { 1.0 } else { -1.0 },
            Some(GameResult::Draw) => 0.0,
            Some(GameResult::Eliminated(p)) => if p == player { -1.0 } else { 1.0 },
            None => 2.0 * self.win_prob(state, player) - 1.0,
        }
    }
}

/// End-to-end: collect self-play data with `make_policy`, standardize, fit the
/// MLP, and return the [`MlpValue`]. Mirrors [`crate::learn::learn_value`].
#[allow(clippy::too_many_arguments)]
pub fn train_mlp_value(
    deck: &[CardId],
    registry: &CardRegistry,
    n_games: u32,
    max_steps: u32,
    make_policy: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    hidden: usize,
    epochs: usize,
    lr: f32,
    l2: f32,
    seed: u64,
) -> MlpValue {
    let encoder = BasicE2Encoder::for_two_players();
    let (mut x, y) =
        collect_value_data(deck, registry, n_games, max_steps, make_policy, &encoder, seed);
    let (means, stds) = standardize(&mut x);
    let params = train_mlp(&x, &y, hidden, epochs, lr, l2, seed);
    MlpValue { encoder, means, stds, params }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// KEY CORRECTNESS CHECK: the MLP learns XOR — a nonlinear pattern a linear
    /// model provably cannot separate. Inputs in {−1, +1}; label is "exactly one
    /// input positive". After full-batch training the net classifies all four
    /// points correctly with confident margins. If backprop were wrong this
    /// would not converge (a linear model is stuck at 50%).
    #[test]
    fn mlp_learns_xor() {
        let x = vec![
            vec![-1.0, -1.0],
            vec![-1.0, 1.0],
            vec![1.0, -1.0],
            vec![1.0, 1.0],
        ];
        let y = vec![0.0, 1.0, 1.0, 0.0];
        let p = train_mlp(&x, &y, 8, 4000, 0.5, 0.0, 7);
        let preds: Vec<f32> = x.iter().map(|xi| p.forward(xi)).collect();
        // Each prediction must land on the correct side, confidently.
        for (xi, (&pred, &yi)) in x.iter().zip(preds.iter().zip(&y)) {
            if yi > 0.5 {
                assert!(pred > 0.8, "XOR-true point {xi:?} should score high, got {pred}");
            } else {
                assert!(pred < 0.2, "XOR-false point {xi:?} should score low, got {pred}");
            }
        }
    }

    /// A purely linear model fails the same XOR problem — a 0-hidden sanity foil
    /// so the test above can't pass by accident (it confirms XOR is genuinely
    /// non-linearly-separable, not that our setup is trivial). Reuses the linear
    /// trainer; best-case accuracy is 50%, so at least one point is misclassified.
    #[test]
    fn linear_cannot_learn_xor() {
        use crate::learn::train_logistic;
        let x = vec![
            vec![-1.0, -1.0],
            vec![-1.0, 1.0],
            vec![1.0, -1.0],
            vec![1.0, 1.0],
        ];
        let y = vec![0.0, 1.0, 1.0, 0.0];
        let (w, b) = train_logistic(&x, &y, 4000, 0.5, 0.0);
        let sig = |xi: &[f32]| 1.0 / (1.0 + (-(b + w[0] * xi[0] + w[1] * xi[1])).exp());
        let correct = x.iter().zip(&y).filter(|(xi, &yi)| (sig(xi) > 0.5) == (yi > 0.5)).count();
        assert!(correct < 4, "linear model should NOT separate XOR, got {correct}/4 correct");
    }

    /// The full pipeline runs on a few real self-play games and produces an
    /// MlpValue that gives sane (finite, in-range) evaluations.
    #[test]
    fn train_mlp_value_pipeline_runs() {
        use crate::search::RandomStatePolicy;
        use arcana_core::engine::new_game;
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let mv = train_mlp_value(
            &deck, &reg, 3, 2000,
            &|s| Box::new(RandomStatePolicy::new(s)),
            16, 50, 0.3, 1e-4, 1);
        let (state, _y) = new_game(vec![deck.clone(), deck.clone()], &reg, 5);
        let v = mv.value(&state, 0);
        assert!(v.is_finite() && (-1.0..=1.0).contains(&v), "value out of range: {v}");
    }

    /// PAYOFF (non-asserting measurement): the MLP value as a SEARCH LEAF, A/B'd
    /// against the linear learned value and the hand-tuned material value, with
    /// IDENTICAL ValueMc budget for all three. random + PIMC are references.
    /// Both learned values are trained from RANDOM self-play (matches the
    /// existing `value_mc_learned_vs_material` measurement so it's comparable).
    /// This decides whether the nonlinear MLP closes the gap to material that the
    /// linear model could not. #[ignore], run in release.
    #[test]
    #[ignore]
    fn value_mc_mlp_vs_material_and_linear() {
        use crate::learn::learn_value;
        use crate::search::{
            round_robin, MaterialValue, PimcPolicy, RandomStatePolicy, StatePolicy, ValueMcPolicy,
        };
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);

        // Both learned values trained from RANDOM self-play (which actually
        // attacks, so it sees combat→damage→win), same data regime.
        let lv = learn_value(
            &deck, &reg, 30, 4000,
            &|s| Box::new(RandomStatePolicy::new(s)),
            300, 0.3, 1e-4, 1);
        let mv = train_mlp_value(
            &deck, &reg, 30, 4000,
            &|s| Box::new(RandomStatePolicy::new(s)),
            32, 300, 0.3, 1e-4, 1);

        let f_rand = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) };
        let f_vm = |s: u64| -> Box<dyn StatePolicy> {
            Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), s, 6, 25, 10)) };
        let lv_c = lv.clone();
        let f_vl = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(ValueMcPolicy::with_budget(Box::new(lv_c.clone()), s, 6, 25, 10)) };
        let mv_c = mv.clone();
        let f_vmlp = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(ValueMcPolicy::with_budget(Box::new(mv_c.clone()), s, 6, 25, 10)) };
        let dp = deck.clone();
        let f_pimc = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(PimcPolicy::with_budget(s, vec![dp.clone(), dp.clone()], 15, 150, 10)) };

        let names: &[(&str, &dyn Fn(u64) -> Box<dyn StatePolicy>)] = &[
            ("random", &f_rand),
            ("vmc-material", &f_vm),
            ("vmc-linear", &f_vl),
            ("vmc-mlp", &f_vmlp),
            ("pimc", &f_pimc),
        ];
        let rr = round_robin(names, &deck, &reg, 12, 4000);
        println!("MLP value-MC leaf tournament:\n{}", rr.format_table());
    }
}
