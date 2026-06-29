//! RL P3: a value function LEARNED from self-play, the first rung past the
//! hand-tuned [`crate::search::MaterialValue`] heuristic.
//!
//! Pipeline (pure Rust, no external ML stack):
//! 1. [`collect_value_data`] plays self-play games and labels each visited
//!    state, from BOTH players' perspectives, with that player's eventual game
//!    outcome (win=1 / draw=0.5 / loss=0) — Monte-Carlo value targets.
//! 2. Features come from [`BasicE2Encoder`] (123 perspective-relative floats);
//!    they are standardized (zero mean / unit variance per feature) so plain
//!    gradient descent converges.
//! 3. [`train_logistic`] fits logistic regression (win probability) by
//!    full-batch gradient descent with L2.
//! 4. [`LinearValue`] wraps the weights as a [`ValueFn`]: it re-encodes +
//!    standardizes a state and returns `2·σ(w·x+b) − 1` in [-1, 1] (with the
//!    terminal result overriding to ±1), so it drops into any value-based
//!    policy ([`crate::search::GreedyValuePolicy`]).
//!
//! The payoff is measured by the yardstick: greedy(learned) vs greedy(material)
//! vs random vs PIMC in a round-robin.

use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::registry::CardRegistry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::types::{CardId, PlayerId};

use crate::observation::{BasicE2Encoder, Encoder};
use crate::search::{StatePolicy, ValueFn};

fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}

/// Play `n_games` self-play games (both seats built by `make_policy`, seeded per
/// game) on the mirror `deck`, returning `(features, labels)`: at every decision
/// state, one row PER PLAYER — `encoder.encode(state, Some(p))` with label 1.0
/// if `p` won the game, 0.5 on a draw, 0.0 on a loss. Recording both
/// perspectives (not just the mover's) covers the "opponent to move" states a
/// greedy policy evaluates at inference time.
pub fn collect_value_data(
    deck: &[CardId],
    registry: &CardRegistry,
    n_games: u32,
    max_steps: u32,
    make_policy: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    encoder: &BasicE2Encoder,
    seed: u64,
) -> (Vec<Vec<f32>>, Vec<f32>) {
    let mut x: Vec<Vec<f32>> = Vec::new();
    let mut y: Vec<f32> = Vec::new();

    for g in 0..n_games {
        let mut pa = make_policy(seed.wrapping_add(g as u64 * 2 + 1));
        let mut pb = make_policy(seed.wrapping_add(g as u64 * 2 + 2));
        let decks = vec![deck.to_vec(), deck.to_vec()];
        let (mut state, mut yld) = new_game(decks, registry, seed.wrapping_add(g as u64));

        // (row index in x, perspective) pending a label once the game ends.
        let mut pending: Vec<(usize, PlayerId)> = Vec::new();
        let mut steps = 0u32;
        let result = loop {
            match yld {
                EngineYield::GameOver(r) => break r,
                EngineYield::PendingDecision { player, legal_actions, .. } => {
                    if steps >= max_steps || legal_actions.is_empty() {
                        break GameResult::Draw;
                    }
                    for p in 0..state.num_players() {
                        let row = x.len();
                        x.push(encoder.encode(&state, Some(p)));
                        y.push(0.0); // filled in below
                        pending.push((row, p));
                    }
                    let action = if player == 0 {
                        pa.choose(&state, registry, player, &legal_actions)
                    } else {
                        pb.choose(&state, registry, player, &legal_actions)
                    };
                    let (s, yy) = step(state, action, registry);
                    state = s; yld = yy; steps += 1;
                }
            }
        };

        for (row, p) in pending {
            y[row] = match result {
                GameResult::Win(w) => if w == p { 1.0 } else { 0.0 },
                GameResult::Eliminated(e) => if e == p { 0.0 } else { 1.0 },
                GameResult::Draw => 0.5,
            };
        }
    }
    (x, y)
}

/// Standardize `x` IN PLACE to zero mean / unit variance per column; returns
/// `(means, stds)` (std clamped to ≥ 1e-6 so constant features don't divide by
/// zero). The same transform must be applied at inference — [`LinearValue`]
/// stores them.
pub fn standardize(x: &mut [Vec<f32>]) -> (Vec<f32>, Vec<f32>) {
    let d = x.first().map(|r| r.len()).unwrap_or(0);
    let n = x.len().max(1) as f32;
    let mut means = vec![0.0f32; d];
    for row in x.iter() {
        for j in 0..d { means[j] += row[j]; }
    }
    for m in means.iter_mut() { *m /= n; }
    let mut stds = vec![0.0f32; d];
    for row in x.iter() {
        for j in 0..d { let dlt = row[j] - means[j]; stds[j] += dlt * dlt; }
    }
    for s in stds.iter_mut() { *s = (*s / n).sqrt().max(1e-6); }
    for row in x.iter_mut() {
        for j in 0..d { row[j] = (row[j] - means[j]) / stds[j]; }
    }
    (means, stds)
}

/// Logistic regression by full-batch gradient descent with L2 regularization.
/// `x` is assumed already standardized. Returns `(weights, bias)`.
pub fn train_logistic(
    x: &[Vec<f32>], y: &[f32], epochs: usize, lr: f32, l2: f32,
) -> (Vec<f32>, f32) {
    let n = x.len();
    assert!(n > 0 && n == y.len(), "need matching non-empty x/y");
    let d = x[0].len();
    let mut w = vec![0.0f32; d];
    let mut b = 0.0f32;
    let scale = lr / n as f32;
    for _ in 0..epochs {
        let mut gw = vec![0.0f32; d];
        let mut gb = 0.0f32;
        for (xi, &yi) in x.iter().zip(y) {
            let p = sigmoid(b + dot(&w, xi));
            let err = p - yi;
            for j in 0..d { gw[j] += err * xi[j]; }
            gb += err;
        }
        for j in 0..d { w[j] -= scale * gw[j] + lr * l2 * w[j]; }
        b -= scale * gb;
    }
    (w, b)
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// A self-play-learned value function: logistic regression over
/// [`BasicE2Encoder`] features. Implements [`ValueFn`] so it drops into
/// [`crate::search::GreedyValuePolicy`] (or any value-based policy). `Clone` so
/// one trained model can seed a fresh policy per game in a tournament.
#[derive(Clone)]
pub struct LinearValue {
    encoder: BasicE2Encoder,
    means: Vec<f32>,
    stds: Vec<f32>,
    weights: Vec<f32>,
    bias: f32,
}

impl LinearValue {
    /// Win probability for `player` in `[0, 1]` (no terminal override).
    pub fn win_prob(&self, state: &GameState, player: PlayerId) -> f32 {
        let f = self.encoder.encode(state, Some(player));
        let mut z = self.bias;
        for j in 0..f.len() {
            z += (f[j] - self.means[j]) / self.stds[j] * self.weights[j];
        }
        sigmoid(z)
    }
}

impl ValueFn for LinearValue {
    fn value(&self, state: &GameState, player: PlayerId) -> f32 {
        match state.result {
            Some(GameResult::Win(p)) => if p == player { 1.0 } else { -1.0 },
            Some(GameResult::Draw) => 0.0,
            Some(GameResult::Eliminated(p)) => if p == player { -1.0 } else { 1.0 },
            None => 2.0 * self.win_prob(state, player) - 1.0,
        }
    }
}

/// End-to-end: collect self-play data with `make_policy`, standardize, fit
/// logistic regression, and return the [`LinearValue`].
#[allow(clippy::too_many_arguments)]
pub fn learn_value(
    deck: &[CardId],
    registry: &CardRegistry,
    n_games: u32,
    max_steps: u32,
    make_policy: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    epochs: usize,
    lr: f32,
    l2: f32,
    seed: u64,
) -> LinearValue {
    let encoder = BasicE2Encoder::for_two_players();
    let (mut x, y) =
        collect_value_data(deck, registry, n_games, max_steps, make_policy, &encoder, seed);
    let (means, stds) = standardize(&mut x);
    let (weights, bias) = train_logistic(&x, &y, epochs, lr, l2);
    LinearValue { encoder, means, stds, weights, bias }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fast, NON-ignored smoke test for the RL value pipeline (the slow
    /// learn→yardstick measurements live behind `#[ignore]`). It guards
    /// against silent bit-rot of the self-play → encode → logistic-fit →
    /// value-fn path as the engine evolves: it runs the WHOLE pipeline on
    /// tiny params and checks the value function is plumbed sanely (finite
    /// + bounded; correct terminal sign for winner/loser). It deliberately
    /// does NOT assert win-rate superiority — that's noisy and slow, and is
    /// what the `#[ignore]` tournaments measure.
    #[test]
    fn smoke_value_pipeline_runs_and_value_is_sane() {
        use crate::search::{MaterialValue, RandomStatePolicy, ValueFn};
        use arcana_core::state::GameResult;

        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);

        // 1) The full pipeline (self-play → encode → logistic fit) runs on
        //    tiny params without panicking, and yields a finite, bounded value.
        let lv = learn_value(
            &deck, &reg, /*n_games=*/ 2, /*max_steps=*/ 200,
            &|s| Box::new(RandomStatePolicy::new(s)),
            /*epochs=*/ 10, /*lr=*/ 0.3, /*l2=*/ 1e-4, /*seed=*/ 1);
        let (start, _y) = new_game(vec![deck.clone(), deck.clone()], &reg, 5);
        let v = lv.value(&start, 0);
        assert!(v.is_finite() && (-1.0..=1.0).contains(&v),
            "learned value must be finite & bounded, got {v}");

        // 2) Terminal-outcome plumbing: a won game scores positive for the
        //    winner and negative for the loser — for BOTH the learned value
        //    and the hand-tuned material leaf.
        let mut won = start.clone();
        won.result = Some(GameResult::Win(0));
        assert!(lv.value(&won, 0) > 0.0 && lv.value(&won, 1) < 0.0,
            "learned value: winner positive, loser negative");
        assert!(MaterialValue.value(&won, 0) > 0.0 && MaterialValue.value(&won, 1) < 0.0,
            "material value: winner positive, loser negative");
    }

    /// DIAGNOSTIC (non-asserting): why is the learned value a worse search
    /// leaf than MaterialValue (both baselines showed it)? Hypothesis: the
    /// value learned from weak (random) self-play is FLAT — under random
    /// continuation, outcomes barely depend on the position, so the fitted
    /// win-prob has little discriminative spread and is a poor leaf. This
    /// measures, on the SAME mid-game self-play states: spread (std/range)
    /// of learned vs material value, their correlation, and which better
    /// predicts the eventual MC outcome (log-loss). Run in release:
    ///   cargo test -p arcana-ai --release --lib \
    ///     learn::tests::diagnose_learned_vs_material_value -- --ignored --nocapture
    #[test]
    #[ignore]
    fn diagnose_learned_vs_material_value() {
        use crate::search::{MaterialValue, RandomStatePolicy, StatePolicy, ValueFn};

        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let lv = learn_value(
            &deck, &reg, 30, 4000,
            &|s| Box::new(RandomStatePolicy::new(s)), 300, 0.3, 1e-4, 1);
        let mv = MaterialValue;

        // Collect mid-game states (p0 perspective) + eventual MC outcome.
        let mut states: Vec<GameState> = Vec::new();
        let mut outcomes: Vec<f32> = Vec::new();
        for g in 0u64..20 {
            let mut pa = RandomStatePolicy::new(1000 + g * 2);
            let mut pb = RandomStatePolicy::new(1001 + g * 2);
            let (mut s, mut y) = new_game(vec![deck.clone(), deck.clone()], &reg, 500 + g);
            let mut snap: Vec<GameState> = Vec::new();
            let mut steps = 0u32;
            let res = loop {
                match y {
                    EngineYield::GameOver(r) => break r,
                    EngineYield::PendingDecision { player, legal_actions, .. } => {
                        if steps >= 4000 || legal_actions.is_empty() { break GameResult::Draw; }
                        if steps % 7 == 0 { snap.push(s.clone()); }
                        let a = if player == 0 {
                            pa.choose(&s, &reg, player, &legal_actions)
                        } else {
                            pb.choose(&s, &reg, player, &legal_actions)
                        };
                        let (ns, ny) = step(s, a, &reg); s = ns; y = ny; steps += 1;
                    }
                }
            };
            let label = match res {
                GameResult::Win(0) => 1.0, GameResult::Win(_) => 0.0,
                GameResult::Eliminated(0) => 0.0, GameResult::Eliminated(_) => 1.0,
                GameResult::Draw => 0.5,
            };
            for st in snap { states.push(st); outcomes.push(label); }
        }

        let lvs: Vec<f32> = states.iter().map(|s| lv.value(s, 0)).collect();
        let mvs: Vec<f32> = states.iter().map(|s| mv.value(s, 0)).collect();

        fn stats(v: &[f32]) -> (f32, f32, f32, f32) {
            let n = v.len() as f32;
            let mean = v.iter().sum::<f32>() / n;
            let var = v.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
            (mean, var.sqrt(),
             v.iter().cloned().fold(f32::INFINITY, f32::min),
             v.iter().cloned().fold(f32::NEG_INFINITY, f32::max))
        }
        fn pearson(a: &[f32], b: &[f32]) -> f32 {
            let n = a.len() as f32;
            let (ma, mb) = (a.iter().sum::<f32>() / n, b.iter().sum::<f32>() / n);
            let mut cov = 0.0; let mut va = 0.0; let mut vb = 0.0;
            for (x, yv) in a.iter().zip(b) {
                cov += (x - ma) * (yv - mb); va += (x - ma).powi(2); vb += (yv - mb).powi(2);
            }
            cov / (va.sqrt() * vb.sqrt()).max(1e-9)
        }
        // value in [-1,1] → prob; log-loss vs MC outcome.
        fn logloss(v: &[f32], y: &[f32]) -> f32 {
            let n = v.len() as f32;
            v.iter().zip(y).map(|(vi, yi)| {
                let p = ((vi + 1.0) / 2.0).clamp(1e-4, 1.0 - 1e-4);
                -(yi * p.ln() + (1.0 - yi) * (1.0 - p).ln())
            }).sum::<f32>() / n
        }

        let (lm, ls, lmin, lmax) = stats(&lvs);
        let (mm, ms, mmin, mmax) = stats(&mvs);
        println!("\n=== learned-vs-material value diagnosis ({} states) ===", states.len());
        println!("learned : mean={lm:+.3} std={ls:.3} range=[{lmin:+.3},{lmax:+.3}]");
        println!("material: mean={mm:+.3} std={ms:.3} range=[{mmin:+.3},{mmax:+.3}]");
        println!("corr(learned,material) = {:.3}", pearson(&lvs, &mvs));
        println!("log-loss  learned={:.4}  material={:.4}  (lower=better outcome predictor)",
            logloss(&lvs, &outcomes), logloss(&mvs, &outcomes));
        println!("interpretation: learned std << material std ⇒ flat value (poor leaf);");
        println!("                low corr + worse log-loss ⇒ learned fit is weak.");
    }

    /// DIAGNOSTIC (non-asserting): the overfit finding predicts that MORE
    /// training games should drop out-of-sample log-loss toward / below
    /// material's. Trains on 50/150/300 random self-play games and scores
    /// each on a FIXED held-out set of fresh-game states (apples-to-apples)
    /// vs MaterialValue + the always-0.5 baseline. If log-loss falls below
    /// material's ~0.54 with more data, the learner works given enough
    /// games (the tournaments under-trained at 24-30); if it plateaus
    /// above, the bottleneck is features/calibration, not data. Release:
    ///   cargo test -p arcana-ai --release --lib \
    ///     learn::tests::diagnose_value_learning_curve -- --ignored --nocapture
    #[test]
    #[ignore]
    fn diagnose_value_learning_curve() {
        use crate::search::{MaterialValue, RandomStatePolicy, StatePolicy, ValueFn};

        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);

        // Fixed held-out set: fresh games, seeds disjoint from training.
        let mut states: Vec<GameState> = Vec::new();
        let mut outcomes: Vec<f32> = Vec::new();
        for g in 9000u64..9024 {
            let mut pa = RandomStatePolicy::new(g * 2);
            let mut pb = RandomStatePolicy::new(g * 2 + 1);
            let (mut s, mut y) = new_game(vec![deck.clone(), deck.clone()], &reg, g);
            let mut snap: Vec<GameState> = Vec::new();
            let mut steps = 0u32;
            let res = loop {
                match y {
                    EngineYield::GameOver(r) => break r,
                    EngineYield::PendingDecision { player, legal_actions, .. } => {
                        if steps >= 4000 || legal_actions.is_empty() { break GameResult::Draw; }
                        if steps % 7 == 0 { snap.push(s.clone()); }
                        let a = if player == 0 {
                            pa.choose(&s, &reg, player, &legal_actions)
                        } else {
                            pb.choose(&s, &reg, player, &legal_actions)
                        };
                        let (ns, ny) = step(s, a, &reg); s = ns; y = ny; steps += 1;
                    }
                }
            };
            let label = match res {
                GameResult::Win(0) => 1.0, GameResult::Win(_) => 0.0,
                GameResult::Eliminated(0) => 0.0, GameResult::Eliminated(_) => 1.0,
                GameResult::Draw => 0.5,
            };
            for st in snap { states.push(st); outcomes.push(label); }
        }

        fn oos_logloss(states: &[GameState], y: &[f32], vf: &dyn ValueFn) -> f32 {
            let n = states.len() as f32;
            states.iter().zip(y).map(|(s, yi)| {
                let p = ((vf.value(s, 0) + 1.0) / 2.0).clamp(1e-4, 1.0 - 1e-4);
                -(yi * p.ln() + (1.0 - yi) * (1.0 - p).ln())
            }).sum::<f32>() / n
        }

        println!("\n=== value learning curve ({} held-out states) ===", states.len());
        println!("baseline (always 0.5) log-loss = {:.4}", 0.693_f32);
        println!("material            log-loss = {:.4}", oos_logloss(&states, &outcomes, &MaterialValue));
        for &n_games in &[50u32, 150, 300] {
            let lv = learn_value(
                &deck, &reg, n_games, 4000,
                &|s| Box::new(RandomStatePolicy::new(s)), 300, 0.3, 1e-4, 7);
            println!("learned (n={n_games:>3} games) log-loss = {:.4}",
                oos_logloss(&states, &outcomes, &lv));
        }
    }

    /// Logistic regression learns a linearly-separable toy problem: feature 0
    /// positive ⇒ label 1. After training, a clearly-positive input scores
    /// well above a clearly-negative one.
    #[test]
    fn train_logistic_separates_toy_data() {
        let mut x = Vec::new();
        let mut y = Vec::new();
        for i in 0..200 {
            let s = if i % 2 == 0 { 1.0 } else { -1.0 };
            x.push(vec![s, 0.3 * s]);
            y.push(if s > 0.0 { 1.0 } else { 0.0 });
        }
        let (w, b) = train_logistic(&x, &y, 300, 0.5, 0.0);
        let pos = sigmoid(b + w[0] * 1.0 + w[1] * 0.3);
        let neg = sigmoid(b + w[0] * -1.0 + w[1] * -0.3);
        assert!(pos > 0.8, "positive should score high, got {pos}");
        assert!(neg < 0.2, "negative should score low, got {neg}");
    }

    /// The full pipeline runs on a few real self-play games and produces a
    /// LinearValue that gives sane (finite, in-range) evaluations.
    #[test]
    fn learn_value_pipeline_runs() {
        use crate::search::RandomStatePolicy;
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let lv = learn_value(
            &deck, &reg, 3, 2000,
            &|s| Box::new(RandomStatePolicy::new(s)),
            50, 0.3, 1e-4, 1);
        let (state, _y) = new_game(vec![deck.clone(), deck.clone()], &reg, 5);
        let v = lv.value(&state, 0);
        assert!(v.is_finite() && (-1.0..=1.0).contains(&v), "value out of range: {v}");
    }

    /// PAYOFF (non-asserting measurement): learn a value from greedy(material)
    /// self-play, then round-robin greedy(learned) against random,
    /// greedy(material) and PIMC. Two findings from a representative run:
    ///   ranking: pimc(35) > random(22) > g-learned(15) > g-material(0)
    /// (1) the learned 123-feature linear value BEATS the hand-tuned 6-term
    /// material value head-to-head (g-learned 12-0 g-material) — learning helped;
    /// (2) but 1-ply greedy on ANY static value is myopic and loses to random:
    /// declaring attackers doesn't immediately raise material/board (combat
    /// damage resolves later), so greedy never attacks and can't close games.
    /// PIMC dominates because its rollouts supply the missing lookahead. The
    /// real payoff is therefore a learned value used as a SEARCH LEAF, not a
    /// greedy evaluator. Non-asserting (the comparison is the result).
    /// #[ignore], run in release.
    #[test]
    #[ignore]
    fn greedy_learned_vs_baselines() {
        use crate::search::{
            round_robin, GreedyValuePolicy, MaterialValue, PimcPolicy, RandomStatePolicy,
            StatePolicy,
        };
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);

        // Learn from reasonable (greedy-material) self-play, on-distribution
        // for the greedy policy we'll deploy.
        let lv = learn_value(
            &deck, &reg, 24, 4000,
            &|s| Box::new(GreedyValuePolicy::new(Box::new(MaterialValue), s)),
            300, 0.3, 1e-4, 1);

        let f_rand = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) };
        let f_gm = |s: u64| -> Box<dyn StatePolicy> {
            Box::new(GreedyValuePolicy::new(Box::new(MaterialValue), s)) };
        let lv_c = lv.clone();
        let f_gl = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(GreedyValuePolicy::new(Box::new(lv_c.clone()), s)) };
        let dp = deck.clone();
        let f_pimc = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(PimcPolicy::with_budget(s, vec![dp.clone(), dp.clone()], 15, 150, 10)) };

        let names: &[(&str, &dyn Fn(u64) -> Box<dyn StatePolicy>)] =
            &[("random", &f_rand), ("g-material", &f_gm), ("g-learned", &f_gl), ("pimc", &f_pimc)];
        let rr = round_robin(names, &deck, &reg, 12, 4000);
        println!("Learned-value tournament:\n{}", rr.format_table());
    }

    /// PAYOFF (non-asserting): the learned value as a SEARCH LEAF. ValueMcPolicy
    /// runs short rollouts (lookahead, so not myopic) and scores the leaf with a
    /// ValueFn — so this A/Bs vmc(learned) vs vmc(material) with everything else
    /// identical, plus random and PIMC as reference. The value is learned from
    /// RANDOM self-play (which actually attacks, unlike greedy-material), so it
    /// sees combat→damage→win. Tells us whether the learned leaf beats the
    /// hand-tuned leaf inside a search. #[ignore], run in release.
    #[test]
    #[ignore]
    fn value_mc_learned_vs_material() {
        use crate::search::{
            round_robin, MaterialValue, PimcPolicy, RandomStatePolicy, StatePolicy, ValueMcPolicy,
        };
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);

        // Train on 150 games, not 30: the learning-curve diagnostic showed
        // the value is overfit (worse-than-chance) at ~30 games but a
        // BETTER outcome predictor than material by ~150.
        let lv = learn_value(
            &deck, &reg, 150, 4000,
            &|s| Box::new(RandomStatePolicy::new(s)),
            300, 0.3, 1e-4, 1);

        let f_rand = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) };
        let f_vm = |s: u64| -> Box<dyn StatePolicy> {
            Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), s, 6, 25, 10)) };
        let lv_c = lv.clone();
        let f_vl = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(ValueMcPolicy::with_budget(Box::new(lv_c.clone()), s, 6, 25, 10)) };
        let dp = deck.clone();
        let f_pimc = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(PimcPolicy::with_budget(s, vec![dp.clone(), dp.clone()], 15, 150, 10)) };

        let names: &[(&str, &dyn Fn(u64) -> Box<dyn StatePolicy>)] = &[
            ("random", &f_rand), ("vmc-material", &f_vm), ("vmc-learned", &f_vl), ("pimc", &f_pimc),
        ];
        let rr = round_robin(names, &deck, &reg, 10, 4000);
        println!("Value-MC leaf tournament:\n{}", rr.format_table());
    }
}
