//! Monte-Carlo search baseline + a state-aware policy interface and an
//! evaluation harness (win-rate between two policies).
//!
//! The policy ladder, weakest to strongest, all measured by [`win_rate`]:
//!
//! * [`RandomStatePolicy`] — progress-biased random; the baseline.
//! * [`FlatMonteCarloPolicy`] — PERFECT-INFORMATION flat Monte-Carlo: scores
//!   each action by mean rollout value from the TRUE [`GameState`] (it peeks at
//!   hidden cards). Cleanly beats random; the search-infrastructure reference.
//! * [`PimcPolicy`] — Perfect-Information Monte-Carlo: flat-MC's algorithm but
//!   rolling out from [`crate::information_set::determinize`]d worlds, so it
//!   uses ONLY the decider's information. The honest imperfect-information
//!   upgrade — still beats random.
//! * [`IsmctsPolicy`] — Single-Observer IS-MCTS with a UCB tree. Correct
//!   machinery, but its minimax-style tree underperforms PIMC (and random)
//!   against a *random* opponent; see its doc.
//!
//! The [`Policy`](crate::selfplay::Policy) trait in `selfplay` takes an ENCODED
//! observation (for reactive neural policies). A search policy instead needs
//! the live state to simulate from, so it uses the [`StatePolicy`] trait here.

use std::collections::HashMap;

use arcana_core::actions::Action;
use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::registry::CardRegistry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::objects::ObjectId;
use arcana_core::types::{CardId, PlayerId};
use arcana_core::zones::Zone;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::information_set::{determinize, project, DeckList};

/// A policy that chooses an action given the live game state (so it can
/// simulate). `decider` is the player to move — the policy maximizes that
/// player's outcome.
pub trait StatePolicy {
    fn choose(
        &mut self,
        state: &GameState,
        registry: &CardRegistry,
        decider: PlayerId,
        legal: &[Action],
    ) -> Action;
}

/// A player's positional "material" score: life + board + card/mana advantage.
/// The richer leaf signal that lets short rollouts SEE development — playing a
/// land, casting a creature, attacking favorably all raise this immediately,
/// long before the life total moves. Used only as a heuristic (terminal results
/// dominate it; see [`value`]).
fn material(state: &GameState, player: PlayerId) -> f32 {
    // Life is the clock; weight it directly.
    let mut score = state.player(player).life as f32;
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if obj.controller != player { continue; }
        if obj.is_creature() {
            // Power is the offensive clock; toughness is staying power.
            let p = state.computed_power(obj.id).unwrap_or(0).max(0) as f32;
            let t = state.computed_toughness(obj.id).unwrap_or(0).max(0) as f32;
            score += 2.0 * p + t;
        } else if obj.is_land() {
            score += 1.0; // mana development (potential)
        } else {
            score += 2.0; // artifacts / enchantments / planeswalkers
        }
    }
    // Card advantage — cards in hand are future plays.
    score += state.objects.objects_in_zone(Zone::Hand(player)).count() as f32;
    score
}

/// Terminal/heuristic value of `state` from `player`'s perspective, in
/// roughly [-1, 1]. A decided game is ±1 / 0; an undecided (rollout-capped)
/// state falls back to a squashed [`material`] differential (life + board +
/// card/mana advantage), kept strictly inside ±1 (via `tanh`) so a real win
/// always dominates a heuristic estimate. Antisymmetric in 2-player games
/// (`value(s, me) == -value(s, opp)`), which IS-MCTS negamax relies on.
pub fn value(state: &GameState, player: PlayerId) -> f32 {
    match state.result {
        Some(GameResult::Win(p)) => if p == player { 1.0 } else { -1.0 },
        Some(GameResult::Draw) => 0.0,
        Some(GameResult::Eliminated(p)) => if p == player { -1.0 } else { 1.0 },
        None => {
            let me = material(state, player);
            let opp = state.opponents_of(player)
                .map(|o| material(state, o))
                .fold(f32::NEG_INFINITY, f32::max);
            let opp = if opp.is_finite() { opp } else { 0.0 };
            // tanh gives a smooth gradient that doesn't saturate as readily as a
            // hard clamp; 0.9 keeps it strictly inside the terminal ±1.
            0.9 * ((me - opp) / 30.0).tanh()
        }
    }
}

/// Per-permanent MARGINAL value — the in-game "card power" readout. For each
/// battlefield permanent `player` controls, how many percentage points of win
/// probability it is worth in THIS position: `win%(state) − win%(state with that
/// permanent's material removed)`, using the same material weights and calibrated
/// logistic as [`value`] / [`crate::calibrate::win_probability`] (so it's
/// consistent with the eval bar). Answers "which of my permanents is carrying the
/// position." Cheap — no clones, no rollouts: it subtracts each permanent's
/// material weight and re-squashes. Returns `(id, win%_points)` sorted
/// descending; empty if the game is already decided. Note: a MATERIAL marginal
/// (P/T + presence), so it doesn't see ability/evasion value beyond stats — the
/// same honest limit as the heuristic eval.
pub fn card_marginal_values(state: &GameState, player: PlayerId) -> Vec<(ObjectId, f32)> {
    if state.result.is_some() {
        return Vec::new();
    }
    let me = material(state, player);
    let opp = state.opponents_of(player)
        .map(|o| material(state, o))
        .fold(f32::NEG_INFINITY, f32::max);
    let opp = if opp.is_finite() { opp } else { 0.0 };
    // win% of a hypothetical own-material total (opp fixed), via value()'s squash.
    let win_pct = |me_total: f32| {
        100.0 * crate::calibrate::win_probability(0.9 * ((me_total - opp) / 30.0).tanh())
    };
    let full = win_pct(me);
    let mut out = Vec::new();
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if obj.controller != player {
            continue;
        }
        let w = if obj.is_creature() {
            let p = state.computed_power(obj.id).unwrap_or(0).max(0) as f32;
            let t = state.computed_toughness(obj.id).unwrap_or(0).max(0) as f32;
            2.0 * p + t
        } else if obj.is_land() {
            1.0
        } else {
            2.0
        };
        out.push((obj.id, full - win_pct(me - w)));
    }
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// A position evaluator: how good is `state` for `player`, in roughly [-1, 1]
/// (terminal ±1). The pluggable leaf signal — [`MaterialValue`] wraps the
/// hand-tuned [`value`] heuristic; `crate::learn::LinearValue` is learned from
/// self-play. A policy can then be A/B-tested purely on its evaluator.
pub trait ValueFn {
    fn value(&self, state: &GameState, player: PlayerId) -> f32;
}

/// The hand-tuned material heuristic ([`value`]) as a [`ValueFn`].
pub struct MaterialValue;
impl ValueFn for MaterialValue {
    fn value(&self, state: &GameState, player: PlayerId) -> f32 { value(state, player) }
}

/// Card-advantage-rebalanced material (referee **v2**). The naive [`material`]
/// weights board ~9:1 over cards-in-hand and over-weights power (`2P+T`), which
/// systematically over-rates go-wide aggro and under-rates grindy / card-advantage
/// decks — measured directly: optimizing a deck against v1 reward-hacks and the
/// result is the *worst* deck under PIMC, and v1 has ρ≈0 vs real tournament
/// finishes (see `docs/capsule-pioneer/` + `docs/gauntlet-results/`). v2 keeps the
/// same shape with three a-priori-motivated reweights (NOT tuned to any capsule):
///   - creatures `P + T` (drop the power lean that favors aggressive stats),
///   - hand cards count **double** (card advantage is future board, not 1/9th of it),
///   - planeswalkers worth `2 + loyalty` (engines, not signposts).
fn material_v2(state: &GameState, player: PlayerId) -> f32 {
    use arcana_core::types::CounterKind;
    let mut score = state.player(player).life as f32;
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if obj.controller != player { continue; }
        if obj.is_creature() {
            let p = state.computed_power(obj.id).unwrap_or(0).max(0) as f32;
            let t = state.computed_toughness(obj.id).unwrap_or(0).max(0) as f32;
            score += p + t;
        } else if obj.is_planeswalker() {
            score += 2.0 + obj.count_counters(CounterKind::Loyalty) as f32;
        } else if obj.is_land() {
            score += 1.0;
        } else {
            score += 2.0;
        }
    }
    score += 2.0 * state.objects.objects_in_zone(Zone::Hand(player)).count() as f32;
    score
}

/// [`value`] with the v2 material leaf — same terminal handling + tanh squash.
pub fn value_v2(state: &GameState, player: PlayerId) -> f32 {
    match state.result {
        Some(GameResult::Win(p)) => if p == player { 1.0 } else { -1.0 },
        Some(GameResult::Draw) => 0.0,
        Some(GameResult::Eliminated(p)) => if p == player { -1.0 } else { 1.0 },
        None => {
            let me = material_v2(state, player);
            let opp = state.opponents_of(player)
                .map(|o| material_v2(state, o))
                .fold(f32::NEG_INFINITY, f32::max);
            let opp = if opp.is_finite() { opp } else { 0.0 };
            0.9 * ((me - opp) / 30.0).tanh()
        }
    }
}

/// The card-advantage-rebalanced material heuristic ([`value_v2`]) as a [`ValueFn`].
pub struct MaterialValueV2;
impl ValueFn for MaterialValueV2 {
    fn value(&self, state: &GameState, player: PlayerId) -> f32 { value_v2(state, player) }
}

/// Material weights defining a play-STYLE. The hand-tuned leaves are special
/// cases: [`MaterialValue`] is power-forward ("balanced"), [`MaterialValueV2`] is
/// card-advantage ("controlling"). [`WeightedMaterialValue::aggressive`] leans
/// power and discounts toughness / cards / life, so a [`ValueMcPolicy`] search
/// that maximizes it develops threats and races rather than stabilizing.
#[derive(Clone, Copy, Debug)]
pub struct WeightedMaterialValue {
    pub power: f32,
    pub toughness: f32,
    pub hand: f32,
    pub pw_base: f32,
    pub life: f32,
    pub land: f32,
    pub other: f32,
}

impl WeightedMaterialValue {
    /// Offense-forward weights (heavy power, light toughness/hand/life).
    pub fn aggressive() -> Self {
        Self { power: 3.0, toughness: 0.5, hand: 0.4, pw_base: 2.0, life: 0.5, land: 0.8, other: 1.5 }
    }
}

fn material_weighted(state: &GameState, player: PlayerId, w: &WeightedMaterialValue) -> f32 {
    use arcana_core::types::CounterKind;
    let mut score = state.player(player).life as f32 * w.life;
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if obj.controller != player { continue; }
        if obj.is_creature() {
            let p = state.computed_power(obj.id).unwrap_or(0).max(0) as f32;
            let t = state.computed_toughness(obj.id).unwrap_or(0).max(0) as f32;
            score += w.power * p + w.toughness * t;
        } else if obj.is_planeswalker() {
            score += w.pw_base + obj.count_counters(CounterKind::Loyalty) as f32;
        } else if obj.is_land() {
            score += w.land;
        } else {
            score += w.other;
        }
    }
    score += w.hand * state.objects.objects_in_zone(Zone::Hand(player)).count() as f32;
    score
}

impl ValueFn for WeightedMaterialValue {
    fn value(&self, state: &GameState, player: PlayerId) -> f32 {
        match state.result {
            Some(GameResult::Win(p)) => if p == player { 1.0 } else { -1.0 },
            Some(GameResult::Draw) => 0.0,
            Some(GameResult::Eliminated(p)) => if p == player { -1.0 } else { 1.0 },
            None => {
                let me = material_weighted(state, player, self);
                let opp = state.opponents_of(player)
                    .map(|o| material_weighted(state, o, self))
                    .fold(f32::NEG_INFINITY, f32::max);
                let opp = if opp.is_finite() { opp } else { 0.0 };
                0.9 * ((me - opp) / 30.0).tanh()
            }
        }
    }
}

/// A bot's play-STYLE — which value leaf its [`ValueMcPolicy`] search maximizes.
/// Distinguishes the World Stage rivals (derived from each one's deck) beyond raw
/// difficulty: each style is a different [`ValueFn`], so the search actually plays
/// differently (races vs stabilizes vs grinds).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Playstyle {
    /// Race: power-forward, defense/cards discounted ([`WeightedMaterialValue::aggressive`]).
    Aggressive,
    /// The hand-tuned default ([`MaterialValue`]).
    Balanced,
    /// Card-advantage / stability ([`MaterialValueV2`]).
    Controlling,
}

impl Playstyle {
    /// The value leaf this style searches on.
    pub fn leaf(self) -> Box<dyn ValueFn> {
        match self {
            Playstyle::Aggressive => Box::new(WeightedMaterialValue::aggressive()),
            Playstyle::Balanced => Box::new(MaterialValue),
            Playstyle::Controlling => Box::new(MaterialValueV2),
        }
    }

    /// Short stable label (UI / logs).
    pub fn label(self) -> &'static str {
        match self {
            Playstyle::Aggressive => "aggressive",
            Playstyle::Balanced => "balanced",
            Playstyle::Controlling => "controlling",
        }
    }
}

/// One-ply greedy on a [`ValueFn`]: pick the action whose resulting state has
/// the best evaluation for the decider. No rollouts — a direct, cheap test of
/// the evaluator's quality (greedy(learned) vs greedy(material) isolates whether
/// learning beat hand-tuning). Large action sets are sub-sampled (PassPriority
/// always kept).
pub struct GreedyValuePolicy {
    pub value: Box<dyn ValueFn>,
    rng: ChaCha8Rng,
    max_candidates: usize,
}
impl GreedyValuePolicy {
    pub fn new(value: Box<dyn ValueFn>, seed: u64) -> Self {
        Self { value, rng: ChaCha8Rng::seed_from_u64(seed), max_candidates: 24 }
    }
    fn candidates(&mut self, legal: &[Action]) -> Vec<usize> {
        if legal.len() <= self.max_candidates {
            return (0..legal.len()).collect();
        }
        let mut idxs: Vec<usize> = (0..legal.len()).collect();
        idxs.shuffle(&mut self.rng);
        idxs.truncate(self.max_candidates);
        if let Some(p) = legal.iter().position(|a| matches!(a, Action::PassPriority)) {
            if !idxs.contains(&p) { idxs[0] = p; }
        }
        idxs
    }
}
impl StatePolicy for GreedyValuePolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if legal.len() <= 1 { return legal[0].clone(); }
        let cands = self.candidates(legal);
        let mut best = cands[0];
        let mut best_v = f32::NEG_INFINITY;
        for ci in cands {
            let (s, _y) = step(state.clone(), legal[ci].clone(), registry);
            let v = self.value.value(&s, decider);
            if v > best_v { best_v = v; best = ci; }
        }
        legal[best].clone()
    }
}

/// Flat Monte-Carlo with a pluggable value LEAF and a SHORT rollout: for each
/// candidate action, run `rollouts` playouts of at most `rollout_step_cap` steps
/// and score the (usually non-terminal) leaf with `value`, then pick the best
/// mean. This is [`FlatMonteCarloPolicy`] generalized — the short rollout
/// supplies the lookahead a [`GreedyValuePolicy`] lacks (so it isn't myopic),
/// and `value` supplies the leaf signal, which a learned
/// `crate::learn::LinearValue` can sharpen. Perfect-information (rolls out from
/// the true state), so A/B-ing two leaf values is apples-to-apples.
pub struct ValueMcPolicy {
    pub value: Box<dyn ValueFn>,
    rng: ChaCha8Rng,
    pub rollouts: u32,
    pub rollout_step_cap: u32,
    pub max_candidates: usize,
}
impl ValueMcPolicy {
    pub fn new(value: Box<dyn ValueFn>, seed: u64) -> Self {
        Self { value, rng: ChaCha8Rng::seed_from_u64(seed),
               rollouts: 10, rollout_step_cap: 30, max_candidates: 16 }
    }
    pub fn with_budget(value: Box<dyn ValueFn>, seed: u64, rollouts: u32,
                       cap: u32, max_candidates: usize) -> Self {
        Self { value, rng: ChaCha8Rng::seed_from_u64(seed),
               rollouts, rollout_step_cap: cap, max_candidates }
    }
    fn candidate_indices(&mut self, legal: &[Action]) -> Vec<usize> {
        select_candidates(legal, self.max_candidates, &mut self.rng)
    }
}
impl StatePolicy for ValueMcPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if legal.len() <= 1 { return legal[0].clone(); }
        let cands = self.candidate_indices(legal);
        let mut best_idx = cands[0];
        let mut best_avg = f32::NEG_INFINITY;
        for ci in cands {
            let avg = score_candidate(state, registry, decider, &legal[ci],
                self.value.as_ref(), self.rollouts, self.rollout_step_cap, &mut self.rng);
            if avg > best_avg { best_avg = avg; best_idx = ci; }
        }
        legal[best_idx].clone()
    }
}

/// Sub-sample candidate action indices for a value-MC decision: if `legal` fits
/// in `max_candidates` take all of it, else a random subset that ALWAYS retains
/// `PassPriority` (the do-nothing baseline every suggestion is measured against).
/// Shared by [`ValueMcPolicy`] and [`rank_actions`] so the bot and the analysis
/// panel select from the same pool.
fn select_candidates(legal: &[Action], max_candidates: usize, rng: &mut ChaCha8Rng) -> Vec<usize> {
    if legal.len() <= max_candidates {
        return (0..legal.len()).collect();
    }
    let mut idxs: Vec<usize> = (0..legal.len()).collect();
    idxs.shuffle(rng);
    idxs.truncate(max_candidates);
    if let Some(p) = legal.iter().position(|a| matches!(a, Action::PassPriority)) {
        if !idxs.contains(&p) { idxs[0] = p; }
    }
    idxs
}

/// Mean value (from `decider`'s perspective) of taking `action` then playing
/// `rollouts` short [`play_out`]s of at most `cap` steps each. The single
/// per-candidate scoring kernel shared by [`ValueMcPolicy::choose`] and
/// [`rank_actions`], so the bot's pick is exactly the analysis panel's top line.
fn score_candidate(
    state: &GameState, registry: &CardRegistry, decider: PlayerId, action: &Action,
    value_fn: &dyn ValueFn, rollouts: u32, cap: u32, rng: &mut ChaCha8Rng,
) -> f32 {
    let mut sum = 0.0f32;
    for _ in 0..rollouts {
        let (s, y) = step(state.clone(), action.clone(), registry);
        let leaf = play_out(s, y, registry, cap, rng);
        sum += value_fn.value(&leaf, decider);
    }
    sum / rollouts.max(1) as f32
}

/// One ranked candidate action: its `index` into the `legal` slice (the same
/// index the frontend sends back to apply it), its mean rollout `value` from the
/// decider's perspective, and the calibrated `win_pct` of that value.
#[derive(Clone, Debug)]
pub struct ScoredAction {
    pub index: usize,
    pub value: f32,
    pub win_pct: f32,
}

/// Rank `legal` actions for `decider` by value-MC lookahead — the "suggested
/// lines" backend. For each (sub-sampled, `PassPriority`-retaining) candidate,
/// run `rollouts` short playouts capped at `cap` steps, score the leaf with
/// `value_fn`, and convert the mean to a calibrated win% ([`crate::calibrate::
/// win_probability`]). Returns every scored candidate sorted by value
/// descending. This is [`ValueMcPolicy::choose`] turned inside-out: same
/// candidate pool, same [`score_candidate`] kernel, but it returns the whole
/// ranked list instead of just the argmax, so the panel's top line IS the bot's
/// pick. Perfect-information (rolls from the true state); `seed` makes it
/// reproducible.
#[allow(clippy::too_many_arguments)]
pub fn rank_actions(
    state: &GameState, registry: &CardRegistry, decider: PlayerId, legal: &[Action],
    value_fn: &dyn ValueFn, rollouts: u32, cap: u32, max_candidates: usize, seed: u64,
) -> Vec<ScoredAction> {
    if legal.is_empty() {
        return Vec::new();
    }
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let cands = select_candidates(legal, max_candidates, &mut rng);
    let mut scored: Vec<ScoredAction> = cands
        .into_iter()
        .map(|ci| {
            let v = score_candidate(state, registry, decider, &legal[ci],
                value_fn, rollouts, cap, &mut rng);
            ScoredAction { index: ci, value: v, win_pct: 100.0 * crate::calibrate::win_probability(v) }
        })
        .collect();
    scored.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap_or(std::cmp::Ordering::Equal));
    scored
}

/// Pick a "make progress" action index, mirroring the random-game harness /
/// `ProgressBiasedRandomPolicy`: end mulligans first, never concede, prefer
/// real actions over passing — then random within the chosen tier so rollouts
/// stay varied AND terminate. Panics-free: `legal` is assumed non-empty.
fn progress_pick(legal: &[Action], rng: &mut ChaCha8Rng) -> usize {
    use arcana_core::actions::Action as A;
    // Tier 1: keep a mulligan hand (stop the mulligan loop).
    let keepers: Vec<usize> = legal.iter().enumerate()
        .filter(|(_, a)| matches!(a, A::MulliganKeep))
        .map(|(i, _)| i).collect();
    if let Some(&i) = keepers.choose(rng) { return i; }
    // Tier 2: real actions — anything that isn't passing priority, conceding,
    // or asking for another mulligan.
    let real: Vec<usize> = legal.iter().enumerate()
        .filter(|(_, a)| !matches!(a,
            A::PassPriority | A::Concede | A::MulliganAgain))
        .map(|(i, _)| i).collect();
    if let Some(&i) = real.choose(rng) { return i; }
    // Tier 3: pass priority if available (advance the game), never concede.
    if let Some(i) = legal.iter().position(|a| matches!(a, A::PassPriority)) {
        return i;
    }
    // Fallback: first non-concede, else 0.
    legal.iter().position(|a| !matches!(a, A::Concede)).unwrap_or(0)
}

/// A development-biased rollout policy: like [`progress_pick`] but it (1) always
/// plays a land when it can, (2) attacks with the most creatures offered, then
/// (3) takes any other real action — so board / mana / tempo advantages actually
/// CONVERT during a rollout and show up in [`material`], giving short rollouts a
/// real signal. Still randomized within each tier so rollouts stay varied. Used
/// by [`play_out`]; [`RandomStatePolicy`] keeps [`progress_pick`] (it is the
/// baseline opponent, which must stay un-tuned).
fn heuristic_pick(legal: &[Action], rng: &mut ChaCha8Rng) -> usize {
    use arcana_core::actions::Action as A;
    // Tier 1: keep a mulligan hand.
    if let Some(i) = legal.iter().position(|a| matches!(a, A::MulliganKeep)) {
        return i;
    }
    // Tier 2: develop mana — play a land (random among available land plays).
    let lands: Vec<usize> = legal.iter().enumerate()
        .filter(|(_, a)| matches!(a, A::PlayLand { .. })).map(|(i, _)| i).collect();
    if let Some(&i) = lands.choose(rng) { return i; }
    // Tier 3: attack — the declaration with the most attackers (aggression
    // converts board presence into life pressure). Empty declarations skipped.
    let mut best: Option<(usize, usize)> = None;
    for (i, a) in legal.iter().enumerate() {
        if let A::DeclareAttackers { attackers } = a {
            let n = attackers.len();
            if n > 0 && best.map_or(true, |(_, bn)| n > bn) { best = Some((i, n)); }
        }
    }
    if let Some((i, _)) = best { return i; }
    // Tier 4: any other real action (cast, activate, block, resolution choices).
    let real: Vec<usize> = legal.iter().enumerate()
        .filter(|(_, a)| !matches!(a,
            A::PassPriority | A::Concede | A::MulliganAgain | A::DeclareAttackers { .. }))
        .map(|(i, _)| i).collect();
    if let Some(&i) = real.choose(rng) { return i; }
    // Tier 5: pass priority.
    if let Some(i) = legal.iter().position(|a| matches!(a, A::PassPriority)) {
        return i;
    }
    // Fallback: first non-concede (e.g. an empty attack declaration), else 0.
    legal.iter().position(|a| !matches!(a, A::Concede)).unwrap_or(0)
}

/// Play `state`/`yld` forward with the development-biased [`heuristic_pick`]
/// rollout policy until the game is decided or `step_cap` steps elapse,
/// returning the final state. The shared rollout engine for flat-MC, PIMC, and
/// IS-MCTS simulations.
fn play_out(mut state: GameState, mut yld: EngineYield, registry: &CardRegistry,
            step_cap: u32, rng: &mut ChaCha8Rng) -> GameState {
    let mut steps = 0u32;
    loop {
        match yld {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { legal_actions, .. } => {
                if steps >= step_cap || legal_actions.is_empty() { break; }
                let i = heuristic_pick(&legal_actions, rng);
                let (s, y) = step(state, legal_actions[i].clone(), registry);
                state = s; yld = y; steps += 1;
            }
        }
    }
    state
}

/// Progress-biased random state policy — the rollout policy, and a baseline
/// opponent to measure search against.
pub struct RandomStatePolicy {
    rng: ChaCha8Rng,
}
impl RandomStatePolicy {
    pub fn new(seed: u64) -> Self { Self { rng: ChaCha8Rng::seed_from_u64(seed) } }
}
impl StatePolicy for RandomStatePolicy {
    fn choose(&mut self, _state: &GameState, _registry: &CardRegistry,
              _decider: PlayerId, legal: &[Action]) -> Action {
        legal[progress_pick(legal, &mut self.rng)].clone()
    }
}

/// Flat Monte-Carlo (the simplest MCTS variant — no tree growth): for each
/// candidate action, run `rollouts` progress-biased random playouts to a
/// decided game or `rollout_step_cap` steps, and pick the action with the best
/// mean [`value`] from the decider's perspective.
pub struct FlatMonteCarloPolicy {
    rng: ChaCha8Rng,
    /// Random playouts per candidate action.
    pub rollouts: u32,
    /// Steps before a playout is cut off and scored heuristically.
    pub rollout_step_cap: u32,
    /// Cap on how many legal actions to evaluate (huge combat declaration sets
    /// are sub-sampled to stay tractable; `PassPriority` is always kept so
    /// "do nothing" is considered).
    pub max_candidates: usize,
}
impl FlatMonteCarloPolicy {
    pub fn new(seed: u64) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed),
               rollouts: 20, rollout_step_cap: 300, max_candidates: 16 }
    }
    pub fn with_budget(seed: u64, rollouts: u32, cap: u32, max_candidates: usize) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed),
               rollouts, rollout_step_cap: cap, max_candidates }
    }

    /// Play `state`/`yld` out with progress-biased random until decided or the
    /// step cap, then score from `decider`'s view.
    fn rollout_value(&mut self, state: GameState, yld: EngineYield,
                     registry: &CardRegistry, decider: PlayerId) -> f32 {
        let final_state = play_out(state, yld, registry, self.rollout_step_cap, &mut self.rng);
        value(&final_state, decider)
    }

    /// Candidate indices to evaluate: all of `legal` if small, else a random
    /// sample of `max_candidates` (always including a PassPriority if present).
    fn candidate_indices(&mut self, legal: &[Action]) -> Vec<usize> {
        use arcana_core::actions::Action as A;
        if legal.len() <= self.max_candidates {
            return (0..legal.len()).collect();
        }
        let mut idxs: Vec<usize> = (0..legal.len()).collect();
        idxs.shuffle(&mut self.rng);
        idxs.truncate(self.max_candidates);
        if let Some(p) = legal.iter().position(|a| matches!(a, A::PassPriority)) {
            if !idxs.contains(&p) { idxs[0] = p; }
        }
        idxs
    }
}
impl StatePolicy for FlatMonteCarloPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if legal.len() <= 1 { return legal[0].clone(); }
        let candidates = self.candidate_indices(legal);
        let mut best_idx = candidates[0];
        let mut best_avg = f32::NEG_INFINITY;
        for ci in candidates {
            let mut sum = 0.0f32;
            for _ in 0..self.rollouts {
                let (s, y) = step(state.clone(), legal[ci].clone(), registry);
                sum += self.rollout_value(s, y, registry, decider);
            }
            let avg = sum / self.rollouts.max(1) as f32;
            if avg > best_avg { best_avg = avg; best_idx = ci; }
        }
        legal[best_idx].clone()
    }
}

// =============================================================================
// IS-MCTS — imperfect-information search
// =============================================================================

/// One node in the Single-Observer IS-MCTS tree (arena-indexed by `usize`).
/// A node is an information set from the root observer's view; `player` is who
/// moves there. `stats` are per-action and keyed by the action value (legal
/// action sets differ across determinizations, so a positional index won't do).
struct IsNode {
    player: PlayerId,
    stats: HashMap<Action, ActionStat>,
}

/// Per-action statistics at a node. `avail` (how often the action was legal in
/// the sampled worlds that reached this node) drives the IS-MCTS exploration
/// term — the key difference from plain MCTS, which uses parent visit count.
#[derive(Default)]
struct ActionStat {
    child: Option<usize>,
    visits: u32,
    value_sum: f32,
    avail: u32,
}

/// Single-Observer Information-Set Monte-Carlo Tree Search (SO-IS-MCTS): the
/// honest imperfect-information sibling of [`FlatMonteCarloPolicy`]. Each
/// iteration samples a [`determinize`]d world consistent with the decider's
/// information set, descends a shared tree with UCB1 selection over the actions
/// legal *in that world*, expands one new action, runs a progress-biased
/// rollout, and backpropagates the result (negamax — each node scores from its
/// own mover's perspective, valid because [`value`] is antisymmetric in 2-player
/// games). The most-visited root action is returned.
///
/// Needs the per-player starting [`DeckList`]s (the universe of hidden cards)
/// to determinize. Unlike flat-MC it does NOT peek at hidden information.
///
/// ⚠ Against the *random* baseline this UNDERperforms [`PimcPolicy`] (and even
/// random) at practical budgets: its UCB tree computes minimax-style values
/// that assume an *optimal* opponent, so it plays too cautiously against an
/// actually-random one — the known "PIMC beats IS-MCTS" result for card games
/// (confirmed here, including with perfect information, so it is the tree, not
/// determinization variance). Kept as correct, reusable machinery for
/// strong-opponent settings; [`PimcPolicy`] is the shipped imperfect-info
/// policy. See `search_policy_ladder`.
pub struct IsmctsPolicy {
    rng: ChaCha8Rng,
    decks: Vec<DeckList>,
    /// Tree iterations (each = one determinization + descent + rollout).
    pub iterations: u32,
    /// Steps before a rollout is cut off and scored heuristically.
    pub rollout_step_cap: u32,
    /// Cap on actions considered per node (large sets are sub-sampled;
    /// `PassPriority` is always kept).
    pub max_children: usize,
    /// UCB1 exploration constant.
    pub exploration: f32,
}

impl IsmctsPolicy {
    pub fn new(seed: u64, decks: Vec<DeckList>) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed), decks,
               iterations: 80, rollout_step_cap: 200, max_children: 16, exploration: 1.0 }
    }
    pub fn with_budget(seed: u64, decks: Vec<DeckList>, iterations: u32,
                       cap: u32, max_children: usize) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed), decks,
               iterations, rollout_step_cap: cap, max_children, exploration: 1.0 }
    }

    /// Actions to consider at a node: all of `legal` if small, else a stable
    /// prefix of `max_children` (always including a `PassPriority`). The subset
    /// must be DETERMINISTIC across iterations — re-sampling it per iteration
    /// would scatter visits across an ever-changing action set and prevent any
    /// action from accumulating a usable mean.
    fn candidates(&self, legal: &[Action]) -> Vec<Action> {
        if legal.len() <= self.max_children {
            return legal.to_vec();
        }
        let mut out: Vec<Action> = legal.iter().take(self.max_children).cloned().collect();
        if !out.iter().any(|a| matches!(a, Action::PassPriority)) {
            if let Some(p) = legal.iter().find(|a| matches!(a, Action::PassPriority)) {
                out[0] = p.clone();
            }
        }
        out
    }

    /// UCB1 over already-expanded candidates (negamax mean + availability-based
    /// exploration). Unvisited children sort first (infinite score).
    fn ucb_select(&self, node: &IsNode, cands: &[Action]) -> Action {
        let c = self.exploration;
        let mut best = cands[0].clone();
        let mut best_score = f32::NEG_INFINITY;
        for a in cands {
            let s = &node.stats[a];
            let score = if s.visits == 0 {
                f32::INFINITY
            } else {
                let mean = s.value_sum / s.visits as f32;
                mean + c * ((s.avail.max(1) as f32).ln() / s.visits as f32).sqrt()
            };
            if score > best_score { best_score = score; best = a.clone(); }
        }
        best
    }

    /// Run one SO-IS-MCTS iteration on a freshly-determinized `world`.
    /// `root_legal` is the decider's true legal set (determinization-invariant
    /// for the decider's own decision, so reused rather than recomputed).
    fn iterate(&mut self, arena: &mut Vec<IsNode>, root_legal: &[Action],
               mut world: GameState, registry: &CardRegistry) {
        let mut path: Vec<(usize, Action)> = Vec::new();
        let mut node_idx = 0usize;
        let mut legal: Vec<Action> = root_legal.to_vec();

        let final_state = loop {
            if world.is_game_over() || legal.is_empty() {
                break world;
            }
            let cands = self.candidates(&legal);
            for a in &cands {
                arena[node_idx].stats.entry(a.clone()).or_default().avail += 1;
            }
            let untried: Vec<Action> = cands.iter()
                .filter(|a| arena[node_idx].stats[*a].child.is_none())
                .cloned().collect();

            if !untried.is_empty() {
                // EXPANSION: try a new action, then roll out from its result.
                let chosen = untried[self.rng.gen_range(0..untried.len())].clone();
                path.push((node_idx, chosen.clone()));
                let (w2, y2) = step(world, chosen.clone(), registry);
                let child_player = match &y2 {
                    EngineYield::PendingDecision { player, .. } => *player,
                    EngineYield::GameOver(_) => 0,
                };
                let child_idx = arena.len();
                arena.push(IsNode { player: child_player, stats: HashMap::new() });
                arena[node_idx].stats.get_mut(&chosen).unwrap().child = Some(child_idx);
                break play_out(w2, y2, registry, self.rollout_step_cap, &mut self.rng);
            }

            // SELECTION: UCB-descend into an existing child.
            let chosen = self.ucb_select(&arena[node_idx], &cands);
            path.push((node_idx, chosen.clone()));
            let child_idx = arena[node_idx].stats[&chosen].child.unwrap();
            let (w2, y2) = step(world, chosen.clone(), registry);
            node_idx = child_idx;
            world = w2;
            match y2 {
                EngineYield::GameOver(_) => break world,
                EngineYield::PendingDecision { player, legal_actions, .. } => {
                    arena[node_idx].player = player; // keep in sync with reality
                    legal = legal_actions;
                }
            }
        };

        // BACKPROP — negamax: each node scores from its own mover's view.
        for (n, a) in &path {
            let pl = arena[*n].player;
            let s = arena[*n].stats.get_mut(a).unwrap();
            s.visits += 1;
            s.value_sum += value(&final_state, pl);
        }
    }
}

impl StatePolicy for IsmctsPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if legal.len() <= 1 { return legal[0].clone(); }
        let view = project(state, decider);
        let mut arena: Vec<IsNode> = vec![IsNode { player: decider, stats: HashMap::new() }];
        for _ in 0..self.iterations {
            let seed = self.rng.gen::<u64>();
            let world = determinize(&view, &self.decks, registry, seed);
            self.iterate(&mut arena, legal, world, registry);
        }
        // Pick by highest MEAN value among visited root actions — NOT the
        // most-visited "robust child". With a shallow budget, development is
        // split across many distinct actions (PlayLand(a), cast X, …) while
        // PassPriority is a single action that concentrates visits, so the
        // most-visited child is pathologically biased toward passing. Mean
        // value judges each action on its own merit (as flat-MC does).
        let root = &arena[0];
        let mut best = legal[0].clone();
        let mut best_mean = f32::NEG_INFINITY;
        for a in legal {
            if let Some(s) = root.stats.get(a) {
                if s.visits == 0 { continue; }
                let mean = s.value_sum / s.visits as f32;
                if mean > best_mean { best_mean = mean; best = a.clone(); }
            }
        }
        best
    }
}

// =============================================================================
// PIMC — Perfect-Information Monte-Carlo (determinized flat-MC)
// =============================================================================

/// Perfect-Information Monte-Carlo: the imperfect-information upgrade of
/// [`FlatMonteCarloPolicy`] that actually beats the random baseline. It is
/// flat-MC's algorithm — score each candidate by mean rollout value, pick the
/// max — but each rollout starts from a freshly [`determinize`]d world instead
/// of the true (cheating) state, so it never peeks at hidden cards.
///
/// PIMC models the opponent with the same random rollout policy used to score,
/// which MATCHES a random baseline opponent — so unlike a full SO-IS-MCTS UCT
/// tree (which computes minimax values assuming an *optimal* opponent and thus
/// plays too cautiously against a random one), PIMC plays exploitatively and
/// wins. This mirrors the well-known result that PIMC is a strong, hard-to-beat
/// baseline for trick/card games. Needs the per-player [`DeckList`]s to sample.
pub struct PimcPolicy {
    rng: ChaCha8Rng,
    decks: Vec<DeckList>,
    /// Determinized worlds sampled; each contributes one rollout per candidate.
    pub samples: u32,
    /// Steps before a rollout is cut off and scored heuristically.
    pub rollout_step_cap: u32,
    /// Cap on candidate actions evaluated (`PassPriority` always kept).
    pub max_candidates: usize,
}

impl PimcPolicy {
    pub fn new(seed: u64, decks: Vec<DeckList>) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed), decks,
               samples: 60, rollout_step_cap: 250, max_candidates: 16 }
    }
    pub fn with_budget(seed: u64, decks: Vec<DeckList>, samples: u32,
                       cap: u32, max_candidates: usize) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed), decks,
               samples, rollout_step_cap: cap, max_candidates }
    }

    /// Stable candidate subset (always keeping a `PassPriority`).
    fn candidates(&self, legal: &[Action]) -> Vec<Action> {
        if legal.len() <= self.max_candidates {
            return legal.to_vec();
        }
        let mut out: Vec<Action> = legal.iter().take(self.max_candidates).cloned().collect();
        if !out.iter().any(|a| matches!(a, Action::PassPriority)) {
            if let Some(p) = legal.iter().find(|a| matches!(a, Action::PassPriority)) {
                out[0] = p.clone();
            }
        }
        out
    }
}

/// One candidate action scored by PIMC — its mean rollout value for the player
/// to move, in roughly `[-1, 1]` (higher = better). The reusable output of
/// [`PimcPolicy::score_actions`]; also the dense teacher signal for value
/// distillation (`crate::learn::learn_value_from_pimc_scores`).
#[derive(Clone, Debug)]
pub struct PimcScoredAction {
    pub action: Action,
    pub score: f32,
}

impl PimcPolicy {
    /// Score every evaluated candidate action by its **mean** rollout value for
    /// `decider` (averaged over the sampled determinizations) — the reusable core
    /// that [`StatePolicy::choose`] argmaxes over. `PassPriority` is always kept
    /// among candidates. For `legal.len() <= 1`, returns that lone action at
    /// score 0. Consumes exactly the same RNG draws as `choose`, so a same-seed
    /// `choose` returns `score_actions`'s top action (see the parity test).
    pub fn score_actions(
        &mut self,
        state: &GameState,
        registry: &CardRegistry,
        decider: PlayerId,
        legal: &[Action],
    ) -> Vec<PimcScoredAction> {
        if legal.len() <= 1 {
            return legal
                .iter()
                .map(|a| PimcScoredAction { action: a.clone(), score: 0.0 })
                .collect();
        }
        let view = project(state, decider);
        let cands = self.candidates(legal);
        let mut sums = vec![0.0f32; cands.len()];
        for _ in 0..self.samples {
            let seed = self.rng.gen::<u64>();
            let world = determinize(&view, &self.decks, registry, seed);
            for (i, a) in cands.iter().enumerate() {
                let (w, y) = step(world.clone(), a.clone(), registry);
                let final_state = play_out(w, y, registry, self.rollout_step_cap, &mut self.rng);
                sums[i] += value(&final_state, decider);
            }
        }
        let n = self.samples.max(1) as f32;
        cands
            .into_iter()
            .enumerate()
            .map(|(i, a)| PimcScoredAction { action: a, score: sums[i] / n })
            .collect()
    }
}

impl StatePolicy for PimcPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if legal.len() <= 1 { return legal[0].clone(); }
        // argmax over the scored candidates — first-strict-max (ties → lowest
        // index), identical to the pre-refactor tie-break.
        let scored = self.score_actions(state, registry, decider, legal);
        let mut best = 0usize;
        for i in 1..scored.len() {
            if scored[i].score > scored[best].score { best = i; }
        }
        scored[best].action.clone()
    }
}

/// Drive one full game between state-aware policies (`policies[p]` plays player
/// `p`). Returns the result; a hard step cap yields a Draw (matches the
/// random-game harness's non-termination guard).
pub fn play_match(
    decks: Vec<Vec<CardId>>,
    registry: &CardRegistry,
    seed: u64,
    policies: &mut [&mut dyn StatePolicy],
    max_steps: u32,
) -> GameResult {
    let (mut state, mut yld) = new_game(decks, registry, seed);
    let mut steps = 0u32;
    loop {
        match yld {
            EngineYield::GameOver(r) => return r,
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                if steps >= max_steps || legal_actions.is_empty() {
                    return GameResult::Draw;
                }
                let action = policies[player as usize]
                    .choose(&state, registry, player, &legal_actions);
                let (s, y) = step(state, action, registry);
                state = s; yld = y; steps += 1;
            }
        }
    }
}

/// Like [`play_match`] but also captures a replayable [`GameRecord`]
/// (decks + seed + the action taken at each decision + the result). The record
/// re-derives any intermediate state via `arcana_core::record::replay_to`, so
/// self-play games are fully inspectable without serializing the live state.
pub fn play_match_recorded(
    decks: Vec<Vec<CardId>>,
    registry: &CardRegistry,
    seed: u64,
    policies: &mut [&mut dyn StatePolicy],
    max_steps: u32,
) -> (GameResult, arcana_core::record::GameRecord) {
    let mut record = arcana_core::record::GameRecord::new(decks.clone(), seed);
    let (mut state, mut yld) = new_game(decks, registry, seed);
    let mut steps = 0u32;
    let result = loop {
        match yld {
            EngineYield::GameOver(r) => break r,
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                if steps >= max_steps || legal_actions.is_empty() {
                    break GameResult::Draw;
                }
                let action = policies[player as usize]
                    .choose(&state, registry, player, &legal_actions);
                record.actions.push(action.clone());
                let (s, y) = step(state, action, registry);
                state = s; yld = y; steps += 1;
            }
        }
    };
    record.result = Some(result.clone());
    (result, record)
}

/// Win-rate of policy A vs policy B over `n_games`, ALTERNATING seats each game
/// to cancel first-player bias. `mk_a`/`mk_b` build a fresh policy per game
/// (seeded by game index). Returns `(a_wins, b_wins, draws)`.
#[allow(clippy::type_complexity)]
pub fn win_rate(
    deck_a: &[CardId],
    deck_b: &[CardId],
    registry: &CardRegistry,
    n_games: u32,
    max_steps: u32,
    mk_a: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    mk_b: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> (u32, u32, u32) {
    let (mut a_wins, mut b_wins, mut draws) = (0, 0, 0);
    for g in 0..n_games {
        let a_seat: PlayerId = (g % 2) as PlayerId; // alternate who is player 0
        let mut pa = mk_a(g as u64 * 2 + 1);
        let mut pb = mk_b(g as u64 * 2 + 2);
        let decks = if a_seat == 0 {
            vec![deck_a.to_vec(), deck_b.to_vec()]
        } else {
            vec![deck_b.to_vec(), deck_a.to_vec()]
        };
        let mut slots: Vec<&mut dyn StatePolicy> = if a_seat == 0 {
            vec![pa.as_mut(), pb.as_mut()]
        } else {
            vec![pb.as_mut(), pa.as_mut()]
        };
        match play_match(decks, registry, g as u64, &mut slots, max_steps) {
            GameResult::Win(p) if p == a_seat => a_wins += 1,
            GameResult::Win(_) => b_wins += 1,
            _ => draws += 1,
        }
    }
    (a_wins, b_wins, draws)
}

// =============================================================================
// Round-robin tournament — the self-play yardstick
// =============================================================================

/// Result of a [`round_robin`]: a win matrix plus per-contestant totals. Once
/// the honest policies all ceiling-out against random, this is how you tell
/// them apart — they play each OTHER.
pub struct RoundRobin {
    pub names: Vec<String>,
    /// `wins[i][j]` = games contestant `i` won against `j` (0 on the diagonal).
    pub wins: Vec<Vec<u32>>,
    /// `draws[i][j]` = drawn games between `i` and `j` (symmetric).
    pub draws: Vec<Vec<u32>>,
    /// Total wins per contestant across all opponents.
    pub totals: Vec<u32>,
}

impl RoundRobin {
    /// Contestant indices sorted by total wins, descending (the ranking).
    pub fn ranking(&self) -> Vec<usize> {
        let mut idx: Vec<usize> = (0..self.names.len()).collect();
        idx.sort_by(|&a, &b| self.totals[b].cmp(&self.totals[a]));
        idx
    }

    /// A human-readable win matrix + ranking (for CLI / test output).
    pub fn format_table(&self) -> String {
        let mut s = String::new();
        let w = self.names.iter().map(|n| n.len()).max().unwrap_or(4).max(6);
        // Header.
        s.push_str(&format!("{:>w$} |", "", w = w));
        for n in &self.names { s.push_str(&format!(" {:>8}", n)); }
        s.push_str("  |    total\n");
        // Rows.
        for i in 0..self.names.len() {
            s.push_str(&format!("{:>w$} |", self.names[i], w = w));
            for j in 0..self.names.len() {
                if i == j { s.push_str(&format!(" {:>8}", "—")); }
                else { s.push_str(&format!(" {:>8}", self.wins[i][j])); }
            }
            s.push_str(&format!("  | {:>8}\n", self.totals[i]));
        }
        // Ranking line.
        s.push_str("ranking: ");
        let rank: Vec<String> = self.ranking().iter()
            .map(|&i| format!("{}({})", self.names[i], self.totals[i])).collect();
        s.push_str(&rank.join(" > "));
        s
    }
}

/// Round-robin self-play tournament: every unordered pair of `contestants`
/// plays `games_per_pair` games on the mirror `deck` ([`win_rate`] alternates
/// seats), and wins are tallied into a [`RoundRobin`]. Each contestant is a
/// `(name, maker)` where the maker builds a fresh policy seeded per game.
#[allow(clippy::type_complexity)]
pub fn round_robin(
    contestants: &[(&str, &dyn Fn(u64) -> Box<dyn StatePolicy>)],
    deck: &[CardId],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
) -> RoundRobin {
    let n = contestants.len();
    let mut wins = vec![vec![0u32; n]; n];
    let mut draws = vec![vec![0u32; n]; n];
    let mut totals = vec![0u32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let (iw, jw, d) = win_rate(
                deck, deck, registry, games_per_pair, max_steps,
                contestants[i].1, contestants[j].1);
            wins[i][j] = iw;
            wins[j][i] = jw;
            draws[i][j] = d;
            draws[j][i] = d;
            totals[i] += iw;
            totals[j] += jw;
        }
    }
    RoundRobin {
        names: contestants.iter().map(|c| c.0.to_string()).collect(),
        wins, draws, totals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The play-styles are genuinely distinct leaves — aggressive weights favor
    /// offense over defense/cards, every leaf stays finite/bounded, and all agree
    /// on terminal sign (winner positive). A power-heavy board also ranks higher
    /// under Aggressive than under Controlling (the whole point).
    #[test]
    fn playstyles_are_distinct_value_leaves() {
        // Weight shape: aggressive leans power, discounts toughness/cards.
        let a = WeightedMaterialValue::aggressive();
        assert!(a.power > a.toughness && a.power > a.hand && a.hand < 1.0,
            "aggressive leans power, discounts defense + cards");
        assert_ne!(Playstyle::Aggressive.label(), Playstyle::Controlling.label());

        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 5);
        let (mut s, _y) = new_game(vec![deck.clone(), deck.clone()], &reg, 3);

        for style in [Playstyle::Aggressive, Playstyle::Balanced, Playstyle::Controlling] {
            let v = style.leaf().value(&s, 0);
            assert!(v.is_finite() && (-1.0..=1.0).contains(&v),
                "{}: value {v} must be bounded", style.label());
        }
        // Terminal override holds for every style.
        s.result = Some(GameResult::Win(0));
        for style in [Playstyle::Aggressive, Playstyle::Balanced, Playstyle::Controlling] {
            let leaf = style.leaf();
            assert!(leaf.value(&s, 0) > 0.0 && leaf.value(&s, 1) < 0.0,
                "{}: winner positive / loser negative", style.label());
        }
        // (The board-level "aggressive prefers high power" behaviour follows from
        // the weight assertion above — power 3 vs toughness 0.5 — and is exercised
        // end-to-end by the web personality test.)
    }

    /// A match between two random policies terminates and yields a result.
    #[test]
    fn random_match_terminates() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let mut a = RandomStatePolicy::new(1);
        let mut b = RandomStatePolicy::new(2);
        let mut slots: Vec<&mut dyn StatePolicy> = vec![&mut a, &mut b];
        // Any terminal result is fine; the point is it returns (no hang/panic).
        let _ = play_match(vec![deck.clone(), deck], &reg, 42, &mut slots, 4000);
    }

    /// Flat-MC with a tiny budget still returns a legal action without panic.
    #[test]
    fn flat_mc_returns_a_legal_action() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 3);
        let (state, yld) = new_game(vec![deck.clone(), deck], &reg, 5);
        if let EngineYield::PendingDecision { player, legal_actions, .. } = yld {
            let mut p = FlatMonteCarloPolicy::with_budget(1, 2, 30, 4);
            let a = p.choose(&state, &reg, player, &legal_actions);
            assert!(legal_actions.contains(&a));
        }
    }

    /// `PimcPolicy::choose` is exactly argmax over `score_actions`: two same-seed
    /// policies agree (choose picks the top-scored action). Advances a few random
    /// steps to reach a decision with >1 legal action so the argmax is non-trivial.
    #[test]
    fn pimc_choose_is_argmax_of_score_actions() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 5);
        let decks = vec![deck.clone(), deck.clone()];
        let mut rp = RandomStatePolicy::new(3);
        let (mut s, mut y) = new_game(decks.clone(), &reg, 7);
        let mut steps = 0u32;
        loop {
            match y {
                EngineYield::GameOver(_) => return, // no rich decision reached; skip
                EngineYield::PendingDecision { player, legal_actions, .. } => {
                    if legal_actions.len() > 1 {
                        let mut p_choose = PimcPolicy::with_budget(99, decks.clone(), 6, 60, 8);
                        let mut p_score = PimcPolicy::with_budget(99, decks.clone(), 6, 60, 8);
                        let scored = p_score.score_actions(&s, &reg, player, &legal_actions);
                        assert!(!scored.is_empty());
                        let mut best = 0;
                        for i in 1..scored.len() {
                            if scored[i].score > scored[best].score { best = i; }
                        }
                        let chosen = p_choose.choose(&s, &reg, player, &legal_actions);
                        assert_eq!(
                            chosen, scored[best].action,
                            "choose must equal the top action of score_actions"
                        );
                        assert!(legal_actions.contains(&chosen));
                        return;
                    }
                    if steps > 60 { return; }
                    let a = rp.choose(&s, &reg, player, &legal_actions);
                    let (ns, ny) = step(s, a, &reg);
                    s = ns; y = ny; steps += 1;
                }
            }
        }
    }

    /// [`rank_actions`] (the suggested-lines backend) returns every candidate
    /// scored, sorted by value descending, with valid distinct indices, calibrated
    /// win%, and is deterministic for a fixed seed.
    #[test]
    fn rank_actions_ranks_legal_candidates() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 3);
        let (state, yld) = new_game(vec![deck.clone(), deck], &reg, 5);
        if let EngineYield::PendingDecision { player, legal_actions, .. } = yld {
            let ranked =
                rank_actions(&state, &reg, player, &legal_actions, &MaterialValue, 2, 30, 8, 7);
            assert!(!ranked.is_empty());
            // Sorted by value descending.
            for w in ranked.windows(2) {
                assert!(w[0].value >= w[1].value, "not sorted descending");
            }
            for s in &ranked {
                assert!(s.index < legal_actions.len()); // valid index into legal
                assert!((0.0..=100.0).contains(&s.win_pct));
                // win_pct is exactly the calibrated map of the value.
                let expect = 100.0 * crate::calibrate::win_probability(s.value);
                assert!((s.win_pct - expect).abs() < 1e-3);
            }
            // Indices are distinct.
            let mut idxs: Vec<usize> = ranked.iter().map(|s| s.index).collect();
            idxs.sort_unstable();
            idxs.dedup();
            assert_eq!(idxs.len(), ranked.len());
            // Deterministic for a fixed seed.
            let again =
                rank_actions(&state, &reg, player, &legal_actions, &MaterialValue, 2, 30, 8, 7);
            assert_eq!(ranked.len(), again.len());
            for (a, b) in ranked.iter().zip(&again) {
                assert_eq!(a.index, b.index);
                assert!((a.value - b.value).abs() < 1e-6);
            }
        }
    }

    /// [`card_marginal_values`] (the card-power readout) returns one finite,
    /// sorted-descending entry per the player's battlefield permanent.
    #[test]
    fn card_marginal_values_are_sane() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let (state, yld) = new_game(vec![deck.clone(), deck], &reg, 11);
        // Develop a board with a short random playout.
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        let dev = play_out(state, yld, &reg, 200, &mut rng);
        if dev.result.is_some() {
            return; // decided early; nothing to assert
        }
        let mv = card_marginal_values(&dev, 0);
        let n = dev.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.controller == 0).count();
        assert_eq!(mv.len(), n, "one entry per controlled permanent");
        for w in mv.windows(2) {
            assert!(w[0].1 >= w[1].1, "not sorted descending");
        }
        for (_, v) in &mv {
            assert!(v.is_finite());
        }
    }

    /// A recorded random match round-trips through JSON, replays to the same
    /// final state, and renders to a readable snapshot.
    #[test]
    fn recorded_match_replays_and_renders() {
        use arcana_core::record::{replay, replay_to};
        use arcana_core::render::{render, render_oneline};
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 11);
        let mut a = RandomStatePolicy::new(1);
        let mut b = RandomStatePolicy::new(2);
        let mut slots: Vec<&mut dyn StatePolicy> = vec![&mut a, &mut b];
        let (result, record) =
            play_match_recorded(vec![deck.clone(), deck], &reg, 99, &mut slots, 4000);

        // Record round-trips through JSON.
        let json = serde_json::to_string(&record).expect("serialize");
        let back: arcana_core::record::GameRecord =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.actions.len(), record.actions.len());
        assert_eq!(back.result, Some(result));

        // Replay reaches the same final life totals + turn.
        let (final_state, _) = replay(&back, &reg);
        let (mut state, mut yld) = arcana_core::engine::new_game(
            record.decks.clone(), &reg, record.seed);
        for act in &record.actions {
            if matches!(yld, arcana_core::engine::EngineYield::GameOver(_)) { break; }
            let (s, y) = arcana_core::engine::step(state, act.clone(), &reg);
            state = s; yld = y;
        }
        assert_eq!(final_state.turn.turn_number, state.turn.turn_number);

        // render() and replay_to() work at an intermediate point.
        let (mid, _) = replay_to(&back, &reg, record.actions.len() / 2);
        let snap = render(&mid, &reg);
        assert!(snap.contains("Turn ") && snap.contains("life"));
        assert!(render_oneline(&mid).starts_with('T'));
    }

    /// BASELINE: flat Monte-Carlo should beat progress-biased random. Slow
    /// (search × rollouts), so #[ignore]; run in release to read the win-rate.
    #[test]
    #[ignore]
    fn flat_mc_beats_random_baseline() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let (mc, rnd, draws) = win_rate(
            &deck, &deck, &reg, 30, 4000,
            &|s| Box::new(FlatMonteCarloPolicy::with_budget(s, 15, 200, 12)),
            &|s| Box::new(RandomStatePolicy::new(s)),
        );
        println!("flat-MC vs random: MC={mc} random={rnd} draws={draws}");
        assert!(mc > rnd, "search should beat random ({mc} vs {rnd})");
    }

    /// PIMC and IS-MCTS with tiny budgets still return a legal action without
    /// panic (exercises determinize + rollout / tree descent end-to-end).
    #[test]
    fn imperfect_info_policies_return_a_legal_action() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 3);
        let (state, yld) = new_game(vec![deck.clone(), deck.clone()], &reg, 5);
        if let EngineYield::PendingDecision { player, legal_actions, .. } = yld {
            let mut pimc = PimcPolicy::with_budget(1, vec![deck.clone(), deck.clone()], 4, 40, 6);
            assert!(legal_actions.contains(&pimc.choose(&state, &reg, player, &legal_actions)));
            let mut ismcts = IsmctsPolicy::with_budget(1, vec![deck.clone(), deck], 8, 40, 6);
            assert!(legal_actions.contains(&ismcts.choose(&state, &reg, player, &legal_actions)));
        }
    }

    /// BASELINE: PIMC (imperfect-information determinized flat-MC) beats random
    /// despite never peeking at hidden cards. Slow; #[ignore], run in release.
    #[test]
    #[ignore]
    fn pimc_beats_random_baseline() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let d = deck.clone();
        let (pimc, rnd, draws) = win_rate(
            &deck, &deck, &reg, 16, 4000,
            &|s| Box::new(PimcPolicy::with_budget(s, vec![d.clone(), d.clone()], 15, 150, 10)),
            &|s| Box::new(RandomStatePolicy::new(s)),
        );
        println!("PIMC vs random: PIMC={pimc} random={rnd} draws={draws}");
        assert!(pimc > rnd, "PIMC should beat random ({pimc} vs {rnd})");
    }

    /// MEASUREMENT (non-asserting): the full policy ladder vs the random
    /// baseline + the perfect-vs-imperfect gap. Documents the empirical finding
    /// that PIMC (random-opponent rollouts) beats random while a full SO-IS-MCTS
    /// UCT tree (minimax-style, assumes an optimal opponent) does NOT — the
    /// known PIMC-beats-ISMCTS phenomenon for card games. #[ignore]; run in
    /// release to read the numbers.
    #[test]
    #[ignore]
    fn search_policy_ladder() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let d = deck.clone();
        let vs_random = |label: &str, mk: &dyn Fn(u64) -> Box<dyn StatePolicy>| {
            let (w, l, dr) = win_rate(&deck, &deck, &reg, 16, 4000, mk,
                &|s| Box::new(RandomStatePolicy::new(s)));
            println!("{label} vs random: win={w} lose={l} draws={dr}");
        };
        vs_random("flat-MC(perfect)", &|s| Box::new(FlatMonteCarloPolicy::with_budget(s, 15, 150, 10)));
        let d1 = d.clone();
        vs_random("PIMC(imperfect)", &move |s| Box::new(PimcPolicy::with_budget(s, vec![d1.clone(), d1.clone()], 15, 150, 10)));
        let d2 = d.clone();
        vs_random("ISMCTS-UCT", &move |s| Box::new(IsmctsPolicy::with_budget(s, vec![d2.clone(), d2.clone()], 120, 150, 10)));

        // Imperfect-vs-perfect gap: PIMC against the cheating flat-MC.
        let d3 = d.clone();
        let (pimc, flat, dr) = win_rate(&deck, &deck, &reg, 16, 4000,
            &move |s| Box::new(PimcPolicy::with_budget(s, vec![d3.clone(), d3.clone()], 15, 150, 10)),
            &|s| Box::new(FlatMonteCarloPolicy::with_budget(s, 15, 150, 10)));
        println!("PIMC vs flat-MC(perfect): PIMC={pimc} flatMC={flat} draws={dr}");
    }

    /// Cheap, deterministic check that the [`round_robin`] harness tallies
    /// correctly: matrix is square, totals equal the row sums, and every pair's
    /// games are fully accounted for (wins + draws). Two random contestants.
    #[test]
    fn round_robin_tallies_correctly() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let a = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) };
        let b = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s ^ 0xABCD)) };
        let g = 4u32;
        let rr = round_robin(&[("a", &a), ("b", &b)], &deck, &reg, g, 4000);
        assert_eq!(rr.names, vec!["a", "b"]);
        assert_eq!(rr.wins.len(), 2);
        assert_eq!(rr.totals[0], rr.wins[0][1]);
        assert_eq!(rr.totals[1], rr.wins[1][0]);
        // The single pair's games are fully accounted for.
        assert_eq!(rr.wins[0][1] + rr.wins[1][0] + rr.draws[0][1], g);
        assert_eq!(rr.ranking().len(), 2);
        assert!(rr.format_table().contains("ranking:"));
    }

    /// YARDSTICK (non-asserting): does PIMC get stronger with more
    /// determinizations? Vs random every budget ceilings out, so this measures
    /// budgets against EACH OTHER in self-play. Prints the win matrix + ranking;
    /// asserting strict monotonicity over a handful of high-variance games would
    /// be flaky. #[ignore], run in release.
    #[test]
    #[ignore]
    fn pimc_budget_ladder() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let mk = |samples: u32| {
            let d = deck.clone();
            move |s: u64| -> Box<dyn StatePolicy> {
                Box::new(PimcPolicy::with_budget(s, vec![d.clone(), d.clone()], samples, 150, 10))
            }
        };
        let (f5, f15, f30) = (mk(5), mk(15), mk(30));
        let rr = round_robin(
            &[("pimc5", &f5), ("pimc15", &f15), ("pimc30", &f30)],
            &deck, &reg, 10, 4000);
        println!("PIMC budget ladder:\n{}", rr.format_table());
    }

    /// MEASUREMENT (non-asserting): the full self-play ranking — random,
    /// flat-MC (perfect info), PIMC (imperfect), IS-MCTS — all playing each
    /// other. The yardstick that distinguishes policies the vs-random ceiling
    /// hides. #[ignore], run in release.
    #[test]
    #[ignore]
    fn full_tournament() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let d_pimc = deck.clone();
        let d_is = deck.clone();
        let f_rand = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) };
        let f_flat = |s: u64| -> Box<dyn StatePolicy> {
            Box::new(FlatMonteCarloPolicy::with_budget(s, 15, 150, 10)) };
        let f_pimc = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(PimcPolicy::with_budget(s, vec![d_pimc.clone(), d_pimc.clone()], 15, 150, 10)) };
        let f_is = move |s: u64| -> Box<dyn StatePolicy> {
            Box::new(IsmctsPolicy::with_budget(s, vec![d_is.clone(), d_is.clone()], 60, 130, 10)) };
        let rr = round_robin(
            &[("random", &f_rand), ("flatMC", &f_flat), ("pimc", &f_pimc), ("ismcts", &f_is)],
            &deck, &reg, 8, 4000);
        println!("Full tournament:\n{}", rr.format_table());
    }
}
