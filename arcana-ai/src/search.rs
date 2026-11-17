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
use arcana_core::legal_actions::{is_mana_activation, legal_actions, pending_choice_asks_mana};
use arcana_core::registry::CardRegistry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::objects::ObjectId;
use arcana_core::types::{CardId, PlayerId};
use arcana_core::zones::Zone;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::information_set::{determinize, project, sample_world, DeckList};
use crate::plays::{top_candidates, Candidate, KeptPlan};

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
            0.9 * crate::portable::tanh((me - opp) / 30.0)
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
        100.0 * crate::calibrate::win_probability(0.9 * crate::portable::tanh((me_total - opp) / 30.0))
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
            0.9 * crate::portable::tanh((me - opp) / 30.0)
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
                0.9 * crate::portable::tanh((me - opp) / 30.0)
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
/// `crate::learn::LinearValue` can sharpen. By default perfect-information:
/// it rolls out from the true state, seeing hidden cards and the engine's RNG
/// seed, so A/B-ing two leaf values is apples-to-apples. A policy built with
/// [`Self::sampled_with_budget`] is the same search on sampled worlds instead
/// (A2.2's arm I): the same candidate selection, rollout policy, leaf and
/// budget, but rollout `r` of every candidate starts from world `r`, sampled
/// by [`sample_world`] from the decider's view alone.
pub struct ValueMcPolicy {
    pub value: Box<dyn ValueFn>,
    rng: ChaCha8Rng,
    pub rollouts: u32,
    pub rollout_step_cap: u32,
    pub max_candidates: usize,
    sampled: Option<SampledWorlds>,
    /// The rest of a play this policy chose and is carrying out (A1.5ci).
    plan: KeptPlan,
    #[cfg(test)]
    pub(crate) top: TopRule,
}

/// Sampled-state mode's own state: the seat-ordered decklists worlds are
/// sampled from, and a second random stream for their seeds, so the policy's
/// first stream (candidate selection, rollouts) draws as it does in true-state
/// mode.
struct SampledWorlds {
    decks: Vec<DeckList>,
    rng: ChaCha8Rng,
}

impl ValueMcPolicy {
    pub fn new(value: Box<dyn ValueFn>, seed: u64) -> Self {
        Self::with_budget(value, seed, 10, 30, 16)
    }
    pub fn with_budget(value: Box<dyn ValueFn>, seed: u64, rollouts: u32,
                       cap: u32, max_candidates: usize) -> Self {
        Self { value, rng: ChaCha8Rng::seed_from_u64(seed),
               rollouts, rollout_step_cap: cap, max_candidates, sampled: None,
               plan: KeptPlan::default(),
               #[cfg(test)] top: TopRule::Plays }
    }
    /// As [`Self::with_budget`], but rolling out from worlds sampled from the
    /// decider's view (`decks` are the seats' starting decklists, as PIMC
    /// takes them). The world stream is seeded from `seed` by a fixed mix.
    pub fn sampled_with_budget(value: Box<dyn ValueFn>, seed: u64, rollouts: u32,
                               cap: u32, max_candidates: usize, decks: Vec<DeckList>) -> Self {
        let sampled = SampledWorlds {
            decks,
            rng: ChaCha8Rng::seed_from_u64(seed ^ 0x5A3B_1ED0_57A7_E5ED),
        };
        Self { sampled: Some(sampled), ..Self::with_budget(value, seed, rollouts, cap, max_candidates) }
    }
    /// Whether this policy rolls out from sampled worlds.
    pub fn is_sampled(&self) -> bool {
        self.sampled.is_some()
    }
    /// The pool its candidates are selected from: [`top_candidates`].
    fn pool(&self, state: &GameState, registry: &CardRegistry, decider: PlayerId,
            legal: &[Action]) -> Vec<Candidate> {
        #[cfg(test)]
        if self.top != TopRule::Plays {
            return self.top.pool(state, registry, decider, legal);
        }
        top_candidates(state, registry, decider, legal)
    }
    /// Each evaluated candidate's index ([`Candidate::index`]: into `legal`,
    /// or past its end for a play) and its mean rollout value for `decider`,
    /// in evaluation order; [`StatePolicy::choose`] takes the first strict
    /// maximum. Candidates are selected before any world is sampled, from
    /// the first stream in both modes.
    pub fn score_actions(&mut self, state: &GameState, registry: &CardRegistry,
                         decider: PlayerId, legal: &[Action]) -> Vec<(usize, f32)> {
        self.score_candidates(state, registry, decider, legal).into_iter()
            .map(|(c, v)| (c.index, v)).collect()
    }
    /// [`Self::score_actions`] with each candidate whole: its index and the
    /// actions it takes.
    pub fn score_candidates(&mut self, state: &GameState, registry: &CardRegistry,
                            decider: PlayerId, legal: &[Action]) -> Vec<(Candidate, f32)> {
        let pool = self.pool(state, registry, decider, legal);
        self.score_pool(state, registry, decider, legal, pool)
    }
    fn score_pool(&mut self, state: &GameState, registry: &CardRegistry,
                  decider: PlayerId, legal: &[Action], pool: Vec<Candidate>) -> Vec<(Candidate, f32)> {
        let picks = select_candidates(&pool, self.max_candidates, &mut self.rng);
        let worlds: Option<Vec<GameState>> = self.sampled.as_mut().map(|w| {
            let view = project(state, decider);
            (0..self.rollouts)
                .map(|_| {
                    let seed: u64 = w.rng.gen();
                    let engine_seed: u64 = w.rng.gen();
                    sample_world(&view, &w.decks, registry, seed, engine_seed)
                })
                .collect()
        });
        // Each world's own legal list, against which a play's first tap is
        // checked; only when a play is among the candidates.
        let any_play = picks.iter().any(|&k| pool[k].plan.len() > 1);
        let world_legal: Option<Vec<Vec<Action>>> = worlds.as_ref().filter(|_| any_play)
            .map(|ws| ws.iter().map(|w| legal_actions(w, registry)).collect());
        picks
            .into_iter()
            .map(|k| {
                let plan = &pool[k].plan;
                let v = match &worlds {
                    None => score_candidate(state, registry, decider, plan, legal,
                        self.value.as_ref(), self.rollouts, self.rollout_step_cap, &mut self.rng),
                    Some(ws) => score_candidate_from(|r| &ws[r as usize],
                        |r| world_legal.as_ref().map_or(&[][..], |l| &l[r as usize][..]),
                        registry, decider, plan, self.value.as_ref(), self.rollouts,
                        self.rollout_step_cap, &mut self.rng),
                };
                (pool[k].clone(), v)
            })
            .collect()
    }
    /// Rank `legal` for `decider` — the "suggested lines" backend. Every
    /// candidate [`Self::score_actions`] evaluates, with the calibrated win% of
    /// its mean value ([`crate::calibrate::win_probability`]), sorted by value
    /// descending (a stable sort, so ties keep evaluation order). This is
    /// [`StatePolicy::choose`] turned inside-out: the same candidates and
    /// kernel, so the top line is this policy's own pick. A sampled-mode
    /// policy ranks from worlds sampled from `decider`'s view.
    pub fn rank_actions(&mut self, state: &GameState, registry: &CardRegistry,
                        decider: PlayerId, legal: &[Action]) -> Vec<ScoredAction> {
        if legal.is_empty() {
            return Vec::new();
        }
        let mut scored: Vec<ScoredAction> = self.score_candidates(state, registry, decider, legal)
            .into_iter()
            .map(|(c, value)| ScoredAction {
                index: c.index, value, win_pct: 100.0 * crate::calibrate::win_probability(value),
                plan: c.plan,
            })
            .collect();
        scored.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }
}
impl StatePolicy for ValueMcPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if let Some(a) = self.plan.next(state, decider, legal) { return a; }
        if legal.len() <= 1 { return legal[0].clone(); }
        let pool = self.pool(state, registry, decider, legal);
        if pool.len() <= 1 { return take(&mut self.plan, &pool[0], state, decider); }
        let scored = self.score_pool(state, registry, decider, legal, pool);
        let mut best = 0usize;
        let mut best_avg = f32::NEG_INFINITY;
        for (k, (_, avg)) in scored.iter().enumerate() {
            if *avg > best_avg { best_avg = *avg; best = k; }
        }
        take(&mut self.plan, &scored[best].0, state, decider)
    }
}

/// Which pool a search's candidates come from (A1.5ci). Only `Plays`,
/// [`top_candidates`], ships; the other two exist in test builds alone, for
/// the head-to-heads that measure the repair. `Every` is the selection
/// before A1.5ci, every legal action; `Filter` is handoff 14's probe filter,
/// the legal list with mana abilities left out outside the decider's own
/// main phase and outside a choice that asks for mana, unless only conceding
/// would be left (a measurement, never a design: it also removes every cast
/// on the opponent's turn).
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TopRule {
    Plays,
    Every,
    Filter,
}

#[cfg(test)]
impl TopRule {
    pub(crate) fn pool(self, state: &GameState, registry: &CardRegistry, decider: PlayerId,
                       legal: &[Action]) -> Vec<Candidate> {
        use crate::plays::every_action;
        match self {
            TopRule::Plays => top_candidates(state, registry, decider, legal),
            TopRule::Every => every_action(legal),
            TopRule::Filter => {
                let set_aside = legal.iter().any(|a| matches!(a, Action::ActivateAbility { .. }))
                    && !pending_choice_asks_mana(state, registry)
                    && !(state.turn.active_player == decider && state.turn.phase.is_main());
                let kept: Vec<Candidate> = every_action(legal).into_iter()
                    .filter(|c| !(set_aside && is_mana_activation(state, &c.plan[0], registry)))
                    .collect();
                if kept.iter().all(|c| c.plan[0].is_concede()) { every_action(legal) } else { kept }
            }
        }
    }
}

/// The first action of the chosen candidate, keeping the rest of a play.
fn take(plan: &mut KeptPlan, chosen: &Candidate, state: &GameState, decider: PlayerId) -> Action {
    plan.keep(&chosen.plan, state, decider);
    chosen.plan[0].clone()
}

/// Sub-sample candidates for a value-MC decision, as positions in `pool`: if
/// `pool` fits in `max_candidates` take all of it, else a random subset that
/// ALWAYS retains `PassPriority` (the do-nothing baseline every suggestion is
/// measured against). [`ValueMcPolicy`]'s, so its
/// [`ValueMcPolicy::rank_actions`], the analysis panel's backend, selects as
/// its choice does. (PIMC/IS-MCTS use the seedless [`stable_candidate_subset`]
/// instead — they need per-decision stability.)
fn select_candidates(pool: &[Candidate], max_candidates: usize, rng: &mut ChaCha8Rng) -> Vec<usize> {
    if pool.len() <= max_candidates {
        return (0..pool.len()).collect();
    }
    let mut idxs: Vec<usize> = (0..pool.len()).collect();
    idxs.shuffle(rng);
    idxs.truncate(max_candidates);
    if let Some(p) = pool.iter().position(Candidate::is_pass) {
        if !idxs.contains(&p) { idxs[0] = p; }
    }
    idxs
}

/// Mean value (from `decider`'s perspective) of taking the actions of
/// `plan` then playing `rollouts` short [`play_out`]s of at most `cap` steps
/// each. The single per-candidate scoring kernel behind
/// [`ValueMcPolicy::choose`] and [`ValueMcPolicy::rank_actions`], so a
/// policy's pick is its ranking's top line. `legal` is the decision's.
#[allow(clippy::too_many_arguments)]
fn score_candidate(
    state: &GameState, registry: &CardRegistry, decider: PlayerId, plan: &[Action],
    legal: &[Action], value_fn: &dyn ValueFn, rollouts: u32, cap: u32, rng: &mut ChaCha8Rng,
) -> f32 {
    score_candidate_from(|_| state, |_| legal, registry, decider, plan, value_fn, rollouts, cap, rng)
}

/// [`score_candidate`] with rollout `r` starting from `start(r)`, whose
/// legal list is `legal(r)`: the true state for every rollout in true-state
/// mode, world `r` in sampled-state mode.
#[allow(clippy::too_many_arguments)]
fn score_candidate_from<'s, 'l>(
    start: impl Fn(u32) -> &'s GameState, legal: impl Fn(u32) -> &'l [Action],
    registry: &CardRegistry, decider: PlayerId, plan: &[Action], value_fn: &dyn ValueFn,
    rollouts: u32, cap: u32, rng: &mut ChaCha8Rng,
) -> f32 {
    let mut sum = 0.0f32;
    for r in 0..rollouts {
        let (s, y) = apply_plan(start(r).clone(), plan, legal(r), registry, decider);
        let leaf = play_out(s, y, registry, cap, rng);
        sum += value_fn.value(&leaf, decider);
    }
    sum / rollouts.max(1) as f32
}

/// Apply a candidate's actions in order. A play's are the decider's own
/// mana abilities and then its cast, planned from its own permanents and
/// hand, so each is legal in the true state and in every world sampled
/// from the decider's view, and the decider keeps the decision between
/// them: anything else is a bug, and panics. `legal` is the starting
/// state's legal list. A game that ends part way (a pain land's damage)
/// ends there.
fn apply_plan(state: GameState, plan: &[Action], legal: &[Action], registry: &CardRegistry,
              decider: PlayerId) -> (GameState, EngineYield) {
    if plan.len() > 1 {
        assert!(legal.contains(&plan[0]), "a play's first action is not legal here: {:?}", plan[0]);
    }
    let (mut s, mut y) = step(state, plan[0].clone(), registry);
    for a in &plan[1..] {
        match &y {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { player, legal_actions, .. } => assert!(
                *player == decider && legal_actions.contains(a),
                "a play's next action is not legal here: {a:?}"),
        }
        (s, y) = step(s, a.clone(), registry);
    }
    (s, y)
}

/// One ranked candidate: its `index` into the `legal` slice (the same index
/// the frontend sends back to apply it), or past its end for a play, its
/// mean rollout `value` from the decider's perspective, the calibrated
/// `win_pct` of that value, and the actions it takes.
#[derive(Clone, Debug)]
pub struct ScoredAction {
    pub index: usize,
    pub value: f32,
    pub win_pct: f32,
    /// The actions it takes: `[legal[index]]`, or a play's taps and cast,
    /// whose `index` is past the end of `legal` ([`Candidate::index`]).
    pub plan: Vec<Action>,
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
///
/// A mana ability is taken only where its mana can be spent. Pools empty as
/// each step and phase ends (CR 500.5) and a cast pays from the pool, so a
/// land tapped in the upkeep is a land the main phase does not have (A1.5cg).
/// In its own main phase the rule taps for mana before any other real action,
/// as it once did in the upkeep, so what it casts is chosen from everything
/// it can afford; in a choice that asks it for mana ([`rollout_pays_now`]) a
/// mana ability is one of the real actions. Anywhere else the mana abilities
/// are set aside, and the fallback takes one only when nothing else is
/// legal, so a rollout cannot stall on them.
fn heuristic_pick(state: &GameState, registry: &CardRegistry, decider: PlayerId,
                  legal: &[Action], rng: &mut ChaCha8Rng) -> usize {
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
    // Tier 4: mana, in its own main phase — every mana ability before any
    // other real action there. A choice that asks for mana comes first: in
    // it a mana ability is one of tier 5's real actions.
    let offered = legal.iter().any(|a| matches!(a, A::ActivateAbility { .. }));
    let pays_now = offered && rollout_pays_now(state, registry);
    if offered && !pays_now && state.turn.active_player == decider && state.turn.phase.is_main() {
        let taps: Vec<usize> = legal.iter().enumerate()
            .filter(|(_, a)| is_mana_activation(state, a, registry)).map(|(i, _)| i).collect();
        if let Some(&i) = taps.choose(rng) { return i; }
    }
    // Tier 5: any other real action (cast, activate, block, resolution choices).
    let real: Vec<usize> = legal.iter().enumerate()
        .filter(|(_, a)| !matches!(a,
            A::PassPriority | A::Concede | A::MulliganAgain | A::DeclareAttackers { .. })
            && (pays_now || !is_mana_activation(state, a, registry)))
        .map(|(i, _)| i).collect();
    if let Some(&i) = real.choose(rng) { return i; }
    // Tier 6: pass priority.
    if let Some(i) = legal.iter().position(|a| matches!(a, A::PassPriority)) {
        return i;
    }
    // Fallback: first non-concede (e.g. an empty attack declaration, or a
    // mana ability set aside above when nothing else is legal), else 0.
    legal.iter().position(|a| !matches!(a, A::Concede)).unwrap_or(0)
}

/// Is the rollout being asked for mana now (CR 605.3a;
/// [`pending_choice_asks_mana`], A1.5cd's window)? Then a mana ability pays
/// for the answer, whoever's turn it is.
fn rollout_pays_now(state: &GameState, registry: &CardRegistry) -> bool {
    pending_choice_asks_mana(state, registry)
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
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                if steps >= step_cap || legal_actions.is_empty() { break; }
                let i = heuristic_pick(&state, registry, player, &legal_actions, rng);
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
    /// The rest of a play this policy chose and is carrying out (A1.5ci).
    plan: KeptPlan,
    #[cfg(test)]
    pub(crate) top: TopRule,
}
impl FlatMonteCarloPolicy {
    pub fn new(seed: u64) -> Self {
        Self::with_budget(seed, 20, 300, 16)
    }
    pub fn with_budget(seed: u64, rollouts: u32, cap: u32, max_candidates: usize) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed),
               rollouts, rollout_step_cap: cap, max_candidates,
               plan: KeptPlan::default(),
               #[cfg(test)] top: TopRule::Plays }
    }

    /// Play `state`/`yld` out with progress-biased random until decided or the
    /// step cap, then score from `decider`'s view.
    fn rollout_value(&mut self, state: GameState, yld: EngineYield,
                     registry: &CardRegistry, decider: PlayerId) -> f32 {
        let final_state = play_out(state, yld, registry, self.rollout_step_cap, &mut self.rng);
        value(&final_state, decider)
    }

    /// The pool its candidates are selected from: [`top_candidates`].
    fn pool(&self, state: &GameState, registry: &CardRegistry, decider: PlayerId,
            legal: &[Action]) -> Vec<Candidate> {
        #[cfg(test)]
        if self.top != TopRule::Plays {
            return self.top.pool(state, registry, decider, legal);
        }
        top_candidates(state, registry, decider, legal)
    }
}
impl StatePolicy for FlatMonteCarloPolicy {
    /// Candidates to evaluate: all of the pool if small, else a random sample
    /// of `max_candidates` (always including a PassPriority if present), as
    /// [`select_candidates`] draws it.
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if let Some(a) = self.plan.next(state, decider, legal) { return a; }
        if legal.len() <= 1 { return legal[0].clone(); }
        let pool = self.pool(state, registry, decider, legal);
        if pool.len() <= 1 { return take(&mut self.plan, &pool[0], state, decider); }
        let candidates = select_candidates(&pool, self.max_candidates, &mut self.rng);
        let mut best = candidates[0];
        let mut best_avg = f32::NEG_INFINITY;
        for k in candidates {
            let mut sum = 0.0f32;
            for _ in 0..self.rollouts {
                let (s, y) = apply_plan(state.clone(), &pool[k].plan, legal, registry, decider);
                sum += self.rollout_value(s, y, registry, decider);
            }
            let avg = sum / self.rollouts.max(1) as f32;
            if avg > best_avg { best_avg = avg; best = k; }
        }
        take(&mut self.plan, &pool[best], state, decider)
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

    /// Actions to consider at a node: all of `legal` if small, else a STABLE
    /// pseudorandom subset of `max_children` (always including `PassPriority`).
    /// The subset must be deterministic across iterations — re-sampling it per
    /// iteration would scatter visits across an ever-changing action set and
    /// prevent any action from accumulating a usable mean — but it must not be
    /// the enumeration-order prefix (see [`stable_candidate_subset`], A-2).
    fn candidates(&self, legal: &[Action]) -> Vec<Action> {
        stable_candidate_subset(legal, self.max_children)
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
                mean + c * (crate::portable::ln(s.avail.max(1) as f32) / s.visits as f32).sqrt()
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
    /// The rest of a play this policy chose and is carrying out (A1.5ci).
    plan: KeptPlan,
    #[cfg(test)]
    pub(crate) top: TopRule,
}

impl PimcPolicy {
    pub fn new(seed: u64, decks: Vec<DeckList>) -> Self {
        Self::with_budget(seed, decks, 60, 250, 16)
    }
    pub fn with_budget(seed: u64, decks: Vec<DeckList>, samples: u32,
                       cap: u32, max_candidates: usize) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed), decks,
               samples, rollout_step_cap: cap, max_candidates,
               plan: KeptPlan::default(),
               #[cfg(test)] top: TopRule::Plays }
    }

    /// The pool its candidates are selected from: [`top_candidates`].
    fn pool(&self, state: &GameState, registry: &CardRegistry, decider: PlayerId,
            legal: &[Action]) -> Vec<Candidate> {
        #[cfg(test)]
        if self.top != TopRule::Plays {
            return self.top.pool(state, registry, decider, legal);
        }
        top_candidates(state, registry, decider, legal)
    }

    /// Stable candidate subset of the pool (always keeping a `PassPriority`);
    /// see [`stable_candidate_subset`] for the A-2 bias story.
    fn candidates(&self, pool: Vec<Candidate>) -> Vec<Candidate> {
        let keep = stable_pool_subset(&pool, self.max_candidates);
        keep.into_iter().map(|k| pool[k].clone()).collect()
    }
}

/// Deterministic-but-unbiased candidate subset for the search policies (A-2).
/// When `legal` exceeds `cap`, take a pseudorandom subset seeded from a hash
/// of the action set itself: stable for a given decision (IS-MCTS iterations
/// must revisit one fixed child set; replays must reproduce), but free of the
/// old `.take(cap)` prefix bias — which systematically dropped whatever the
/// engine enumerates last. Multi-attacker/-blocker declarations are exactly
/// the sets that blow past the cap, so "attack with everything" / the strong
/// block assignments could go entirely unscored. `PassPriority` is always
/// retained as the do-nothing baseline.
fn stable_candidate_subset(legal: &[Action], cap: usize) -> Vec<Action> {
    if legal.len() <= cap {
        return legal.to_vec();
    }
    use std::hash::{Hash, Hasher};
    // DefaultHasher::new() is keyed with fixed constants (unlike RandomState),
    // so the subset is reproducible across processes.
    let mut h = std::collections::hash_map::DefaultHasher::new();
    format!("{legal:?}").hash(&mut h);
    let mut rng = ChaCha8Rng::seed_from_u64(h.finish());
    let mut out: Vec<Action> = legal.to_vec();
    out.shuffle(&mut rng);
    out.truncate(cap);
    if !out.iter().any(|a| matches!(a, Action::PassPriority)) {
        if let Some(p) = legal.iter().find(|a| matches!(a, Action::PassPriority)) {
            out[0] = p.clone();
        }
    }
    out
}

/// [`stable_candidate_subset`] over a candidate pool, as positions in it. A
/// pool of single actions is seeded from the same hash of the same list, and
/// so keeps the same subset; a pool with plays hashes its plans.
fn stable_pool_subset(pool: &[Candidate], cap: usize) -> Vec<usize> {
    if pool.len() <= cap {
        return (0..pool.len()).collect();
    }
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    if pool.iter().all(|c| c.plan.len() == 1) {
        let actions: Vec<&Action> = pool.iter().map(|c| &c.plan[0]).collect();
        format!("{actions:?}").hash(&mut h);
    } else {
        let plans: Vec<&Vec<Action>> = pool.iter().map(|c| &c.plan).collect();
        format!("{plans:?}").hash(&mut h);
    }
    let mut rng = ChaCha8Rng::seed_from_u64(h.finish());
    let mut out: Vec<usize> = (0..pool.len()).collect();
    out.shuffle(&mut rng);
    out.truncate(cap);
    if !out.iter().any(|&k| pool[k].is_pass()) {
        if let Some(p) = pool.iter().position(Candidate::is_pass) {
            out[0] = p;
        }
    }
    out
}

/// One candidate scored by PIMC — its mean rollout value for the player to
/// move, in roughly `[-1, 1]` (higher = better). The reusable output of
/// [`PimcPolicy::score_actions`]; also the dense teacher signal for value
/// distillation (`crate::learn::learn_value_from_pimc_scores`). `action` is
/// what [`StatePolicy::choose`] returns for it, the first of `plan`'s
/// actions: the action itself, or a play's first tap.
#[derive(Clone, Debug)]
pub struct PimcScoredAction {
    pub action: Action,
    pub score: f32,
    pub plan: Vec<Action>,
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
                .map(|a| PimcScoredAction { action: a.clone(), score: 0.0, plan: vec![a.clone()] })
                .collect();
        }
        let pool = self.pool(state, registry, decider, legal);
        self.score_pool(state, registry, decider, pool)
    }

    fn score_pool(&mut self, state: &GameState, registry: &CardRegistry, decider: PlayerId,
                  pool: Vec<Candidate>) -> Vec<PimcScoredAction> {
        let view = project(state, decider);
        let cands = self.candidates(pool);
        let any_play = cands.iter().any(|c| c.plan.len() > 1);
        let mut sums = vec![0.0f32; cands.len()];
        for _ in 0..self.samples {
            let seed = self.rng.gen::<u64>();
            let world = determinize(&view, &self.decks, registry, seed);
            // The world's own legal list, against which a play's first tap
            // is checked; only when a play is among the candidates.
            let world_legal = if any_play { legal_actions(&world, registry) } else { Vec::new() };
            for (i, c) in cands.iter().enumerate() {
                let (w, y) = apply_plan(world.clone(), &c.plan, &world_legal, registry, decider);
                let final_state = play_out(w, y, registry, self.rollout_step_cap, &mut self.rng);
                sums[i] += value(&final_state, decider);
            }
        }
        let n = self.samples.max(1) as f32;
        cands
            .into_iter()
            .enumerate()
            .map(|(i, c)| PimcScoredAction { action: c.plan[0].clone(), score: sums[i] / n, plan: c.plan })
            .collect()
    }
}

impl StatePolicy for PimcPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if let Some(a) = self.plan.next(state, decider, legal) { return a; }
        if legal.len() <= 1 { return legal[0].clone(); }
        let pool = self.pool(state, registry, decider, legal);
        if pool.len() <= 1 { return take(&mut self.plan, &pool[0], state, decider); }
        // argmax over the scored candidates — first-strict-max (ties → lowest
        // index), identical to the pre-refactor tie-break.
        let scored = self.score_pool(state, registry, decider, pool);
        let mut best = 0usize;
        for i in 1..scored.len() {
            if scored[i].score > scored[best].score { best = i; }
        }
        let chosen = &scored[best];
        self.plan.keep(&chosen.plan, state, decider);
        chosen.action.clone()
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
        match pair_game(deck_a, deck_b, registry, g as u64, max_steps, mk_a, mk_b) {
            PairOutcome::AWin => a_wins += 1,
            PairOutcome::BWin => b_wins += 1,
            PairOutcome::Draw => draws += 1,
        }
    }
    (a_wins, b_wins, draws)
}

/// How one game of a pairing ended, for its first contestant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairOutcome {
    AWin,
    BWin,
    Draw,
}

/// Game `g` of a pairing, as [`win_rate`] schedules it: `a` sits as player
/// `g % 2`, the policies are seeded `2g + 1` and `2g + 2`, and the game `g`.
/// [`win_rate`] plays games `0..n`; any other range of `g` plays further games
/// of the same schedule, so disjoint ranges run apart sum to one run.
pub fn pair_game(
    deck_a: &[CardId],
    deck_b: &[CardId],
    registry: &CardRegistry,
    g: u64,
    max_steps: u32,
    mk_a: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    mk_b: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> PairOutcome {
    let a_seat: PlayerId = (g % 2) as PlayerId; // alternate who is player 0
    let mut pa = mk_a(g * 2 + 1);
    let mut pb = mk_b(g * 2 + 2);
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
    match play_match(decks, registry, g, &mut slots, max_steps) {
        GameResult::Win(p) if p == a_seat => PairOutcome::AWin,
        GameResult::Win(_) => PairOutcome::BWin,
        _ => PairOutcome::Draw,
    }
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
    use crate::plays::Candidate;

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

    /// [`ValueMcPolicy::rank_actions`] (the suggested-lines backend) returns
    /// every candidate scored, sorted by value descending, with valid distinct
    /// indices and calibrated win%; it is deterministic for a fixed seed, and
    /// its top line is the same policy's choice. In both modes.
    #[test]
    fn rank_actions_ranks_legal_candidates() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 3);
        let decks = vec![deck.clone(), deck];
        let (state, yld) = new_game(decks.clone(), &reg, 5);
        let EngineYield::PendingDecision { player, legal_actions, .. } = yld else {
            panic!("a new game starts at a decision");
        };
        let modes: [&dyn Fn() -> ValueMcPolicy; 2] = [
            &|| ValueMcPolicy::with_budget(Box::new(MaterialValue), 7, 2, 30, 8),
            &|| ValueMcPolicy::sampled_with_budget(Box::new(MaterialValue), 7, 2, 30, 8, decks.clone()),
        ];
        for policy in modes {
            let ranked = policy().rank_actions(&state, &reg, player, &legal_actions);
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
            let again = policy().rank_actions(&state, &reg, player, &legal_actions);
            assert_eq!(ranked.len(), again.len());
            for (a, b) in ranked.iter().zip(&again) {
                assert_eq!(a.index, b.index);
                assert!((a.value - b.value).abs() < 1e-6);
            }
            // The top line is the policy's own pick.
            assert_eq!(policy().choose(&state, &reg, player, &legal_actions),
                       legal_actions[ranked[0].index]);
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

    // --- A1.5cg: the rollout rule and mana abilities ------------------------

    /// Mountain, Forest, Lightning Bolt, Grizzly Bears and Prodigal Sorcerer
    /// ("{T}: 1 damage to any target", not a mana ability) from the catalog,
    /// and Toll, an instant: "You may pay {2}. If you do, draw a card."
    /// For A1.5ci also Tundra ("{T}: Add {W}." then "{T}: Add {U}."),
    /// Disintegrate ({X}{R}: X damage to any target), Burst Lightning ({R},
    /// kicker {4}), and Study, a {U} sorcery with no effect.
    struct RuleCards {
        reg: CardRegistry,
        mountain: CardId,
        forest: CardId,
        bolt: CardId,
        bears: CardId,
        sorcerer: CardId,
        toll: CardId,
        tundra: CardId,
        disintegrate: CardId,
        burst: CardId,
        study: CardId,
    }

    fn rule_cards() -> RuleCards {
        use arcana_core::actions::OptionalPaymentKind;
        use arcana_core::effects::Effect;
        use arcana_core::mana::ManaCost;
        use arcana_core::objects::Characteristics;
        use arcana_core::registry::{CardDefinition, SpellAbilityDef};
        use arcana_core::stack::StackEntry;
        use arcana_core::types::{ColorSet, TypeLine};
        fn toll_resolve(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
            vec![Effect::OptionalPayment {
                chooser: entry.controller,
                cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").unwrap()),
                then: Box::new(Effect::DrawCards { player: entry.controller, count: 1 }),
                else_effect: None,
            }]
        }
        let mut reg = CardRegistry::new();
        let mountain = arcana_cards::lea::mountain::register(&mut reg);
        let forest = arcana_cards::lea::forest::register(&mut reg);
        let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
        let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
        let sorcerer = arcana_cards::lea::prodigal_sorcerer::register(&mut reg);
        let name = reg.interner_mut().intern("Rule Test Toll");
        let toll = reg.register(CardDefinition::new(name, Characteristics {
            name,
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "You may pay {2}. If you do, draw a card.".into(),
            target_requirements: Vec::new(),
            modal: None,
            effect: toll_resolve,
        }));
        fn study_resolve(_: &GameState, _: &StackEntry, _: &CardRegistry) -> Vec<Effect> { Vec::new() }
        let tundra = arcana_cards::lea::tundra::register(&mut reg);
        let disintegrate = arcana_cards::lea::disintegrate::register(&mut reg);
        let burst = arcana_cards::zen::burst_lightning::register(&mut reg);
        let name = reg.interner_mut().intern("Rule Test Study");
        let study = reg.register(CardDefinition::new(name, Characteristics {
            name,
            mana_cost: Some(ManaCost::parse("{U}").unwrap()),
            colors: ColorSet::blue(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Nothing happens.".into(),
            target_requirements: Vec::new(),
            modal: None,
            effect: study_resolve,
        }));
        RuleCards { reg, mountain, forest, bolt, bears, sorcerer, toll, tundra, disintegrate, burst, study }
    }

    /// `active`'s turn at `phase` and `step`, `holder` with priority, and ten
    /// Forests in each library.
    fn rule_table(c: &RuleCards, active: PlayerId, phase: arcana_core::turn::Phase,
                  step: arcana_core::turn::Step, holder: PlayerId) -> GameState {
        let mut s = GameState::new(2, 20261116);
        s.turn.active_player = active;
        s.turn.phase = phase;
        s.turn.step = step;
        s.priority.give_to(holder);
        for p in [0, 1] {
            for _ in 0..10 { rule_add(&mut s, c, p, c.forest, Zone::Library(p)); }
        }
        s
    }

    fn rule_add(s: &mut GameState, c: &RuleCards, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
        let id = s.allocate_object_id();
        s.objects.insert(arcana_core::objects::GameObject::new(id, owner, zone, card,
            c.reg.get(card).unwrap().initial_characteristics().clone()));
        if let Zone::Library(p) = zone {
            s.player_mut(p).library_top_to_bottom.push(id);
        }
        id
    }

    /// Onto the battlefield through the zone mover.
    fn rule_enter(s: &mut GameState, c: &RuleCards, owner: PlayerId, card: CardId) -> ObjectId {
        let id = rule_add(s, c, owner, card, Zone::Hand(owner));
        s.move_object_to_zone(id, Zone::Battlefield, arcana_core::events::MoveCause::SpellResolution)
            .expect("entered")
    }

    fn rule_act(s: GameState, c: &RuleCards, pick: impl Fn(&GameState, &Action) -> bool) -> GameState {
        let chosen = arcana_core::legal_actions::legal_actions(&s, &c.reg).into_iter()
            .find(|a| pick(&s, a)).expect("the action is offered");
        step(s, chosen, &c.reg).0
    }

    /// `decider` at priority with {R} floating, an untapped Mountain, an
    /// untapped Prodigal Sorcerer and a Lightning Bolt in hand: the list
    /// holds a cast, a mana ability, an activation that is not one, and a
    /// pass. Returns the state and that list.
    fn bolt_in_hand(c: &RuleCards, decider: PlayerId, active: PlayerId,
                    phase: arcana_core::turn::Phase, st: arcana_core::turn::Step)
        -> (GameState, Vec<Action>)
    {
        let mut s = rule_table(c, active, phase, st, decider);
        let first = rule_enter(&mut s, c, decider, c.mountain);
        rule_enter(&mut s, c, decider, c.mountain);
        let sorcerer = rule_enter(&mut s, c, decider, c.sorcerer);
        s.objects.get_mut(sorcerer).unwrap().status.summoning_sick = false;
        rule_add(&mut s, c, decider, c.bolt, Zone::Hand(decider));
        let s = rule_act(s, c, |_, a| matches!(a, Action::ActivateAbility { source, .. } if *source == first));
        let legal = arcana_core::legal_actions::legal_actions(&s, &c.reg);
        assert_eq!(legal.iter().filter(|a| is_mana_activation(&s, a, &c.reg)).count(), 1,
            "the second Mountain's ability is offered");
        assert!(legal.iter().any(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == sorcerer)),
            "the Sorcerer's ability is offered");
        assert!(legal.iter().any(|a| matches!(a, Action::CastSpell { .. })), "the Bolt is castable");
        assert!(legal.iter().any(|a| matches!(a, Action::PassPriority)));
        (s, legal)
    }

    /// `decider` answering Toll's "you may pay {2}" with two untapped
    /// Mountains, a choice that asks for mana: in the opponent's upkeep,
    /// outside the decider's own main phase, or (`own_main`) inside it.
    fn toll_window(c: &RuleCards, decider: PlayerId, own_main: bool) -> (GameState, Vec<Action>) {
        use arcana_core::turn::{Phase, Step};
        let opp = 1 - decider;
        let mut s = if own_main {
            rule_table(c, decider, Phase::PreCombatMain, Step::Main, decider)
        } else {
            rule_table(c, opp, Phase::Beginning, Step::Upkeep, decider)
        };
        let first = rule_enter(&mut s, c, decider, c.mountain);
        for _ in 0..2 { rule_enter(&mut s, c, decider, c.mountain); }
        rule_add(&mut s, c, decider, c.toll, Zone::Hand(decider));
        let s = rule_act(s, c, |_, a| matches!(a, Action::ActivateAbility { source, .. } if *source == first));
        let s = rule_act(s, c, |_, a| matches!(a, Action::CastSpell { .. }));
        // Both players pass and Toll resolves into its question.
        let s = rule_act(s, c, |_, a| a.is_pass());
        let s = rule_act(s, c, |_, a| a.is_pass());
        assert!(pending_choice_asks_mana(&s, &c.reg), "Toll asks for mana");
        assert_eq!(s.pending_choice.as_ref().map(|p| p.choosing_player), Some(decider));
        assert!(if own_main { s.turn.active_player == decider && s.turn.phase.is_main() }
                else { s.turn.active_player == opp && s.turn.step == Step::Upkeep });
        let legal = arcana_core::legal_actions::legal_actions(&s, &c.reg);
        assert_eq!(legal.iter().filter(|a| is_mana_activation(&s, a, &c.reg)).count(), 2,
            "both untapped Mountains are offered in the window");
        (s, legal)
    }

    /// The rule as it stood before A1.5cg, verbatim: the comparison the tests
    /// below make, and the text the head-to-head probe carries.
    #[allow(clippy::unnecessary_map_or)] // verbatim
    fn heuristic_pick_before_a15cg(legal: &[Action], rng: &mut ChaCha8Rng) -> usize {
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

    /// A1.5cg: where the mana would be lost, the rollout rule takes no mana
    /// ability while anything else is legal. With a cast, a mana ability,
    /// another activation and a pass on offer at priority, it casts or uses
    /// the other ability, at every seed: in its own upkeep and beginning of
    /// combat, and on the opponent's turn. Before the repair the Mountain
    /// was one of its choices.
    #[test]
    fn the_rollout_rule_sets_mana_abilities_aside_where_their_mana_would_be_lost() {
        use arcana_core::turn::{Phase, Step};
        let c = rule_cards();
        for decider in [0u8, 1] {
            let opp = 1 - decider;
            for (active, phase, st) in [
                (decider, Phase::Beginning, Step::Upkeep),
                (decider, Phase::Combat, Step::BeginCombat),
                (opp, Phase::Beginning, Step::Upkeep),
                (opp, Phase::PreCombatMain, Step::Main),
            ] {
                let (s, legal) = bolt_in_hand(&c, decider, active, phase, st);
                let (mut casts, mut others) = (0, 0);
                for seed in 0..200 {
                    let mut rng = ChaCha8Rng::seed_from_u64(seed);
                    let i = heuristic_pick(&s, &c.reg, decider, &legal, &mut rng);
                    assert!(!is_mana_activation(&s, &legal[i], &c.reg),
                        "seat {decider}, {st:?} of seat {active}'s turn, seed {seed}: picked {:?}", legal[i]);
                    match &legal[i] {
                        Action::CastSpell { .. } => casts += 1,
                        Action::ActivateAbility { .. } => others += 1,
                        other => panic!("seat {decider}, seed {seed}: picked {other:?}"),
                    }
                }
                assert!(casts > 20 && others > 20,
                    "only the mana ability is set aside: {casts} casts, {others} other activations");
                // With nothing else to do it passes, and with no pass either
                // it takes the mana ability sooner than concede or stall.
                let no_cast: Vec<Action> = legal.iter()
                    .filter(|a| !matches!(a, Action::CastSpell { .. })
                        && (is_mana_activation(&s, a, &c.reg) || !matches!(a, Action::ActivateAbility { .. })))
                    .cloned().collect();
                let only_taps: Vec<Action> = no_cast.iter()
                    .filter(|a| !matches!(a, Action::PassPriority)).cloned().collect();
                for seed in 0..20 {
                    let mut rng = ChaCha8Rng::seed_from_u64(seed);
                    let i = heuristic_pick(&s, &c.reg, decider, &no_cast, &mut rng);
                    assert!(matches!(no_cast[i], Action::PassPriority), "it passes: {:?}", no_cast[i]);
                    let i = heuristic_pick(&s, &c.reg, decider, &only_taps, &mut rng);
                    assert!(is_mana_activation(&s, &only_taps[i], &c.reg), "no stall: {:?}", only_taps[i]);
                }
            }
        }
    }

    /// A1.5cg: from its upkeep, with two untapped Forests and Grizzly Bears in
    /// hand, a rollout reaches its main phase with the Forests untapped and
    /// casts the Bears there, at every seed and from either seat. Before the
    /// repair it tapped both Forests in the upkeep and the mana was gone when
    /// the upkeep ended.
    #[test]
    fn a_rollout_keeps_its_lands_for_the_main_phase_and_casts_there() {
        use arcana_core::actions::DecisionContext;
        use arcana_core::events::GameEvent;
        use arcana_core::turn::{Phase, Step};
        let c = rule_cards();
        for seat in [0u8, 1] {
            let mut s = rule_table(&c, seat, Phase::Beginning, Step::Upkeep, seat);
            for _ in 0..2 { rule_enter(&mut s, &c, seat, c.forest); }
            rule_add(&mut s, &c, seat, c.bears, Zone::Hand(seat));
            let start = s.event_log.len();
            let legal = arcana_core::legal_actions::legal_actions(&s, &c.reg);
            assert_eq!(legal.iter().filter(|a| is_mana_activation(&s, a, &c.reg)).count(), 2,
                "the upkeep offers both Forests");
            assert!(!legal.iter().any(|a| matches!(a, Action::CastSpell { .. })),
                "and no cast, with nothing in the pool");
            let yld = EngineYield::PendingDecision {
                player: seat, legal_actions: legal, context: DecisionContext::Priority };
            for seed in 0..24 {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);
                let leaf = play_out(s.clone(), yld.clone(), &c.reg, 40, &mut rng);
                let log = &leaf.event_log[start..];
                let at = |want: &dyn Fn(&GameEvent) -> bool| log.iter().position(want);
                let main = at(&|e| matches!(e, GameEvent::StepBegins { step: Step::Main }))
                    .expect("the rollout reaches the main phase");
                let combat = at(&|e| matches!(e, GameEvent::StepBegins { step: Step::BeginCombat }))
                    .expect("and the combat after it");
                let mana = at(&|e| matches!(e, GameEvent::ManaAdded { .. }))
                    .expect("the Forests are tapped at some point");
                let cast = at(&|e| matches!(e, GameEvent::SpellCast { .. }));
                assert!(mana > main, "seat {seat}, seed {seed}: a Forest was tapped before the main phase");
                assert!(cast.is_some_and(|k| main < k && k < combat),
                    "seat {seat}, seed {seed}: the Bears are cast in the first main phase");
                assert!(leaf.objects.objects_in_zone(Zone::Battlefield)
                    .any(|o| o.card_id == c.bears && o.controller == seat),
                    "seat {seat}, seed {seed}: the Bears are on the battlefield");
            }
        }
    }

    /// A1.5cg: on the opponent's turn a rollout leaves its lands alone. From
    /// the opponent's upkeep, with two untapped Forests and Grizzly Bears in
    /// hand, this seat makes no mana until the opponent's turn has ended, and
    /// then casts the Bears. (The rule asks about the player deciding, who
    /// is not always the player whose turn it is.)
    #[test]
    fn a_rollout_does_not_tap_on_the_opponents_turn() {
        use arcana_core::actions::DecisionContext;
        use arcana_core::events::GameEvent;
        use arcana_core::turn::{Phase, Step};
        let c = rule_cards();
        for seat in [0u8, 1] {
            let opp = 1 - seat;
            let mut s = rule_table(&c, opp, Phase::Beginning, Step::Upkeep, opp);
            for _ in 0..2 { rule_enter(&mut s, &c, seat, c.forest); }
            rule_add(&mut s, &c, seat, c.bears, Zone::Hand(seat));
            let start = s.event_log.len();
            let legal = arcana_core::legal_actions::legal_actions(&s, &c.reg);
            let yld = EngineYield::PendingDecision {
                player: opp, legal_actions: legal, context: DecisionContext::Priority };
            for seed in 0..24 {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);
                let leaf = play_out(s.clone(), yld.clone(), &c.reg, 80, &mut rng);
                let log = &leaf.event_log[start..];
                let turn_end = log.iter().position(|e| matches!(e, GameEvent::TurnEnds { .. }))
                    .expect("the opponent's turn ends");
                let mine = log.iter().position(|e| matches!(e, GameEvent::ManaAdded { player, .. } if *player == seat))
                    .expect("this seat taps its Forests at some point");
                assert!(mine > turn_end,
                    "seat {seat}, seed {seed}: it tapped a Forest on the opponent's turn");
                assert!(leaf.objects.objects_in_zone(Zone::Battlefield)
                    .any(|o| o.card_id == c.bears && o.controller == seat),
                    "seat {seat}, seed {seed}: the Bears are on the battlefield");
            }
        }
    }

    /// A1.5cg: in its own main phase the rollout rule taps for mana before
    /// any other real action: with a cast, another activation and a mana
    /// ability on offer it takes the mana ability at every seed, and once
    /// none is left its choice is what it was before the repair. In a choice
    /// that asks it for mana, on the opponent's turn, a mana ability is one
    /// of its real actions as before: the same pick at every seed, the mana
    /// ability among them.
    #[test]
    fn the_rollout_rule_taps_first_in_its_own_main_phase_and_pays_in_a_payment_window() {
        use arcana_core::turn::{Phase, Step};
        let c = rule_cards();
        for decider in [0u8, 1] {
            for phase in [Phase::PreCombatMain, Phase::PostCombatMain] {
                let (s, legal) = bolt_in_hand(&c, decider, decider, phase, Step::Main);
                assert!(!pending_choice_asks_mana(&s, &c.reg), "the main phase alone allows it");
                let untapped: Vec<Action> = legal.iter()
                    .filter(|a| !is_mana_activation(&s, a, &c.reg)).cloned().collect();
                for seed in 0..200 {
                    let i = heuristic_pick(&s, &c.reg, decider, &legal, &mut ChaCha8Rng::seed_from_u64(seed));
                    assert!(is_mana_activation(&s, &legal[i], &c.reg),
                        "seat {decider}, {phase:?}, seed {seed}: picked {:?} before the Mountain", legal[i]);
                    let i = heuristic_pick(&s, &c.reg, decider, &untapped, &mut ChaCha8Rng::seed_from_u64(seed));
                    let before = heuristic_pick_before_a15cg(&untapped, &mut ChaCha8Rng::seed_from_u64(seed));
                    assert_eq!(i, before, "seat {decider}, {phase:?}, seed {seed}: with no mana ability left");
                }
            }
            // The window, on the opponent's turn and in its own main phase:
            // being asked comes before tapping first.
            for own_main in [false, true] {
                let (s, legal) = toll_window(&c, decider, own_main);
                let mut tapped = 0;
                for seed in 0..200 {
                    let i = heuristic_pick(&s, &c.reg, decider, &legal, &mut ChaCha8Rng::seed_from_u64(seed));
                    let before = heuristic_pick_before_a15cg(&legal, &mut ChaCha8Rng::seed_from_u64(seed));
                    assert_eq!(i, before, "seat {decider}, the window (own main: {own_main}), seed {seed}");
                    if is_mana_activation(&s, &legal[i], &c.reg) { tapped += 1; }
                }
                assert!(tapped > 20 && tapped < 180,
                    "seat {decider}, own main {own_main}: in the window a mana ability was picked {tapped} times in 200");
            }
        }
    }

    /// A1.5cg: the repair changes the rule only around mana abilities. Over
    /// three games the rule plays against itself, every decision is picked
    /// at one seed by the rule and by its text before the repair. With no
    /// mana ability on offer they agree. With one on offer in the decider's
    /// own main phase, the rule plays a land if it can and otherwise takes a
    /// mana ability; anywhere else its pick is the old rule's pick from the
    /// list without the mana abilities.
    #[test]
    fn the_rollout_rule_changes_only_around_mana_abilities() {
        let c = rule_cards();
        let mut deck = Vec::new();
        for (card, n) in [(c.mountain, 12), (c.forest, 12), (c.bolt, 18), (c.bears, 18)] {
            deck.extend(std::iter::repeat_n(card, n));
        }
        let (mut plain, mut first, mut aside) = (0, 0, 0);
        for game in 0..3u64 {
            let (mut state, mut yld) = new_game(vec![deck.clone(), deck.clone()], &c.reg, 20261116 + game);
            let mut game_rng = ChaCha8Rng::seed_from_u64(game);
            for decision in 0..4000u64 {
                let (player, legal) = match &yld {
                    EngineYield::PendingDecision { player, legal_actions, .. } => (*player, legal_actions.clone()),
                    EngineYield::GameOver(_) => break,
                };
                assert!(!pending_choice_asks_mana(&state, &c.reg), "these decks ask for no mana");
                let new = heuristic_pick(&state, &c.reg, player, &legal, &mut ChaCha8Rng::seed_from_u64(decision));
                let taps = legal.iter().filter(|a| is_mana_activation(&state, a, &c.reg)).count();
                let own_main = state.turn.active_player == player && state.turn.phase.is_main();
                if taps == 0 {
                    let old = heuristic_pick_before_a15cg(&legal, &mut ChaCha8Rng::seed_from_u64(decision));
                    assert_eq!(new, old, "game {game}, decision {decision}");
                    plain += 1;
                } else if own_main {
                    let land = legal.iter().any(|a| matches!(a, Action::PlayLand { .. }));
                    assert!(if land { matches!(legal[new], Action::PlayLand { .. }) }
                            else { is_mana_activation(&state, &legal[new], &c.reg) },
                        "game {game}, decision {decision}: picked {:?}", legal[new]);
                    first += 1;
                } else {
                    let kept: Vec<usize> = (0..legal.len())
                        .filter(|&i| !is_mana_activation(&state, &legal[i], &c.reg)).collect();
                    let rest: Vec<Action> = kept.iter().map(|&i| legal[i].clone()).collect();
                    let old = heuristic_pick_before_a15cg(&rest, &mut ChaCha8Rng::seed_from_u64(decision));
                    assert_eq!(new, kept[old], "game {game}, decision {decision}");
                    aside += 1;
                }
                let i = heuristic_pick(&state, &c.reg, player, &legal, &mut game_rng);
                let (s, y) = step(state, legal[i].clone(), &c.reg);
                state = s; yld = y;
            }
            assert!(state.result.is_some(), "game {game} ends");
        }
        assert!(plain > 300 && first > 60 && aside > 40,
            "the games hold each kind of decision: {plain} without a mana ability, {first} in the own main phase, {aside} elsewhere");
    }

    // --- A1.5ci: plays as candidates at the top of the search ---------------

    /// `decider` holding priority with an empty pool, `lands` untapped on its
    /// battlefield and `hand` in its hand: in its own precombat main phase
    /// (`own`), or in the opponent's upkeep. The state, its legal list and
    /// the hand's ids.
    fn plays_table(c: &RuleCards, decider: PlayerId, own: bool, lands: &[CardId], hand: &[CardId])
        -> (GameState, Vec<Action>, Vec<ObjectId>)
    {
        use arcana_core::turn::{Phase, Step};
        let mut s = if own {
            rule_table(c, decider, Phase::PreCombatMain, Step::Main, decider)
        } else {
            rule_table(c, 1 - decider, Phase::Beginning, Step::Upkeep, decider)
        };
        for &l in lands { rule_enter(&mut s, c, decider, l); }
        let ids = hand.iter().map(|&h| rule_add(&mut s, c, decider, h, Zone::Hand(decider))).collect();
        let legal = arcana_core::legal_actions::legal_actions(&s, &c.reg);
        assert!(s.player(decider).mana_pool.is_empty());
        (s, legal, ids)
    }

    /// The decklists a sampler needs for a `plays_table` state: each seat's
    /// every card, wherever it is.
    fn table_decks(s: &GameState) -> Vec<DeckList> {
        (0..s.num_players()).map(|p| s.objects.iter().filter(|o| o.owner == p)
            .map(|o| o.card_id).collect()).collect()
    }

    fn lone_mana(s: &GameState, c: &RuleCards, cands: &[Candidate]) -> usize {
        cands.iter().filter(|k| k.plan.len() == 1 && is_mana_activation(s, &k.plan[0], &c.reg)).count()
    }

    /// Apply a candidate's plan to a clone, each action from the legal list of
    /// the moment, `decider` deciding throughout; the state after it.
    fn run_plan(s: &GameState, c: &RuleCards, decider: PlayerId, plan: &[Action]) -> GameState {
        let mut s = s.clone();
        for a in plan {
            let legal = arcana_core::legal_actions::legal_actions(&s, &c.reg);
            assert!(legal.contains(a), "planned {a:?} is not legal");
            assert_eq!(s.priority.player, decider);
            s = step(s, a.clone(), &c.reg).0;
        }
        s
    }

    /// A1.5ci: in its own main phase with an empty pool, two Forests and
    /// Grizzly Bears in hand, a search's candidates are the pass, conceding
    /// and one play, the two taps then the cast, and no lone mana ability;
    /// the play, applied, puts the Bears on the stack. In both seats. Before
    /// the repair the candidates were the legal list: two lone taps.
    #[test]
    fn the_candidates_are_the_cast_with_its_taps_and_no_lone_mana_ability() {
        let c = rule_cards();
        for decider in [0, 1] {
            let (s, legal, hand) = plays_table(&c, decider, true, &[c.forest, c.forest], &[c.bears]);
            assert_eq!(legal.iter().filter(|a| is_mana_activation(&s, a, &c.reg)).count(), 2);
            assert!(!legal.iter().any(|a| matches!(a, Action::CastSpell { .. })), "nothing is castable yet");
            let cands = top_candidates(&s, &c.reg, decider, &legal);
            assert_eq!(lone_mana(&s, &c, &cands), 0, "seat {decider}: a lone tap is a candidate");
            let plays: Vec<&Candidate> = cands.iter().filter(|k| k.is_play(legal.len())).collect();
            assert_eq!(plays.len(), 1, "seat {decider}: {cands:?}");
            let p = &plays[0].plan;
            assert_eq!(p.len(), 3);
            assert!(p[..2].iter().all(|a| is_mana_activation(&s, a, &c.reg)));
            assert!(matches!(p[2], Action::CastSpell { object_id, .. } if object_id == hand[0]));
            assert_eq!(plays[0].index, legal.len());
            for k in cands.iter().filter(|k| !k.is_play(legal.len())) {
                assert_eq!(k.plan, vec![legal[k.index].clone()]);
            }
            assert!(cands.iter().any(Candidate::is_pass));
            let after = run_plan(&s, &c, decider, p);
            assert!(after.objects.objects_in_zone(Zone::Stack).any(|o| o.card_id == c.bears),
                "the Bears are cast");
        }
    }

    /// A1.5ci: on the opponent's turn, with a Mountain and Lightning Bolt in
    /// hand, Bolt is a play for each target the legal list would enumerate
    /// once the mana floats, and no lone tap is a candidate; handoff 14's
    /// probe filter, beside it, offers no cast at all. In both seats.
    #[test]
    fn an_instant_on_the_opponents_turn_is_a_play() {
        let c = rule_cards();
        for decider in [0, 1] {
            let (s, legal, hand) = plays_table(&c, decider, false, &[c.mountain], &[c.bolt]);
            let bolts = arcana_core::legal_actions::potential_actions(&s, decider, &c.reg).into_iter()
                .filter(|a| matches!(a, Action::CastSpell { .. })).count();
            assert!(bolts >= 2, "Bolt has a target in each player");
            let cands = top_candidates(&s, &c.reg, decider, &legal);
            assert_eq!(lone_mana(&s, &c, &cands), 0);
            let plays: Vec<&Candidate> = cands.iter().filter(|k| k.is_play(legal.len())).collect();
            assert_eq!(plays.len(), bolts, "seat {decider}: a play for each Bolt target");
            for k in &plays {
                assert_eq!(k.plan.len(), 2);
                assert!(matches!(k.plan[1], Action::CastSpell { object_id, .. } if object_id == hand[0]));
                run_plan(&s, &c, decider, &k.plan);
            }
            let filtered = TopRule::Filter.pool(&s, &c.reg, decider, &legal);
            assert!(!filtered.iter().any(|k| k.plan.iter().any(|a| matches!(a, Action::CastSpell { .. }))),
                "the probe filter casts nothing here");
        }
    }

    /// A1.5ci: in a choice that asks for mana (Toll's "you may pay {2}"), every
    /// legal action is a candidate, the two untapped Mountains' abilities
    /// among them, on the opponent's turn and in the decider's own main phase.
    /// In both seats.
    #[test]
    fn a_choice_that_asks_for_mana_keeps_its_mana_abilities_as_candidates() {
        let c = rule_cards();
        for decider in [0, 1] {
            for own_main in [false, true] {
                let (s, legal) = toll_window(&c, decider, own_main);
                let cands = top_candidates(&s, &c.reg, decider, &legal);
                assert_eq!(cands, crate::plays::every_action(&legal));
                assert_eq!(lone_mana(&s, &c, &cands), 2);
            }
        }
    }

    /// A1.5ci: with three Mountains and Disintegrate ({X}{R}) in hand, the
    /// plays are each target at each X the Mountains can pay, 0 to 2, and
    /// each taps X + 1 of them. The planner the web client's one-click cast
    /// uses taps only until some Disintegrate is legal, which is X = 0. In
    /// both seats.
    #[test]
    fn an_x_spell_gives_a_play_per_payable_x_each_tapped_for_its_x() {
        let c = rule_cards();
        for decider in [0, 1] {
            let lands = [c.mountain, c.mountain, c.mountain];
            let (s, legal, hand) = plays_table(&c, decider, true, &lands, &[c.disintegrate]);
            let auto = arcana_core::legal_actions::auto_tap_sequence(&s, &c.reg, decider, hand[0]).unwrap();
            assert_eq!(auto.len(), 1, "the one-click planner stops at the first tap: {auto:?}");
            let cands = top_candidates(&s, &c.reg, decider, &legal);
            let mut seen = Vec::new();
            for k in cands.iter().filter(|k| k.is_play(legal.len())) {
                let Some(Action::CastSpell { x_value: Some(x), targets, .. }) = k.plan.last() else {
                    panic!("a play ends in the cast: {k:?}");
                };
                assert_eq!(k.plan.len() as u32, x + 2, "X = {x} taps {} Mountains", x + 1);
                let after = run_plan(&s, &c, decider, &k.plan);
                assert!(after.player(decider).mana_pool.is_empty(), "X = {x} spends all it taps");
                seen.push((*x, format!("{targets:?}")));
            }
            for x in 0..=2u32 {
                assert_eq!(seen.iter().filter(|(v, _)| *v == x).count(), 2,
                    "seat {decider}: one play for each player as target at X = {x}: {seen:?}");
            }
            assert_eq!(seen.len(), 6);
        }
    }

    /// A1.5ci: a variant that costs more than its card's cheapest, Burst
    /// Lightning kicked, is a play of its own, tapped for its whole cost: on
    /// the opponent's turn with five Mountains, each target unkicked taps one
    /// Mountain and kicked taps all five. In both seats.
    #[test]
    fn a_kicked_variant_is_a_play_tapped_for_its_kicker() {
        let c = rule_cards();
        for decider in [0, 1] {
            let (s, legal, _) = plays_table(&c, decider, false, &[c.mountain; 5], &[c.burst]);
            let cands = top_candidates(&s, &c.reg, decider, &legal);
            let mut taps: Vec<usize> = cands.iter().filter(|k| k.is_play(legal.len()))
                .map(|k| k.plan.len() - 1).collect();
            taps.sort_unstable();
            assert_eq!(taps, vec![1, 1, 5, 5], "seat {decider}: {cands:?}");
            for k in cands.iter().filter(|k| k.is_play(legal.len())) {
                let after = run_plan(&s, &c, decider, &k.plan);
                assert!(after.player(decider).mana_pool.is_empty());
            }
        }
    }

    /// A1.5ci: a card only a dual land's second color pays for is still a
    /// play. `potential_actions` colors Tundra by its first ability, {W}, and
    /// misses a {U} sorcery; the candidates hold the play that taps Tundra for
    /// {U} and casts it. In both seats.
    #[test]
    fn a_dual_lands_second_color_still_makes_a_play() {
        let c = rule_cards();
        for decider in [0, 1] {
            let (s, legal, hand) = plays_table(&c, decider, true, &[c.tundra], &[c.study]);
            assert!(!arcana_core::legal_actions::potential_actions(&s, decider, &c.reg).iter()
                .any(|a| matches!(a, Action::CastSpell { .. })), "the cheap list misses the {{U}} cast");
            let cands = top_candidates(&s, &c.reg, decider, &legal);
            let plays: Vec<&Candidate> = cands.iter().filter(|k| k.is_play(legal.len())).collect();
            assert_eq!(plays.len(), 1, "seat {decider}: {cands:?}");
            assert!(matches!(plays[0].plan[1], Action::CastSpell { object_id, .. } if object_id == hand[0]));
            let after = run_plan(&s, &c, decider, &plays[0].plan);
            assert!(after.objects.objects_in_zone(Zone::Stack).any(|o| o.card_id == c.study),
                "Study is cast");
            assert_eq!(lone_mana(&s, &c, &cands), 0);
        }
    }

    /// The first `n` decisions of each seat, past the mulligans, where a
    /// search would have a play among its candidates, reached by random play
    /// of the catalog's sample decks 4 and 9.
    fn decisions_with_plays(reg: &CardRegistry, decks: &[Vec<CardId>], n: usize)
        -> Vec<(GameState, PlayerId, Vec<Action>, Vec<Candidate>)>
    {
        let mut out = Vec::new();
        let mut per_seat = [0usize; 2];
        for game in 0..8u64 {
            let (mut state, mut yld) = new_game(decks.to_vec(), reg, 90 + game);
            let mut rand = RandomStatePolicy::new(game);
            for _ in 0..3000 {
                let EngineYield::PendingDecision { player, legal_actions, .. } = yld else { break };
                if per_seat[player as usize] < n && state.turn.turn_number >= 2 {
                    let cands = top_candidates(&state, reg, player, &legal_actions);
                    if cands.iter().any(|k| k.is_play(legal_actions.len())) {
                        out.push((state.clone(), player, legal_actions.clone(), cands));
                        per_seat[player as usize] += 1;
                    }
                }
                let a = rand.choose(&state, reg, player, &legal_actions);
                let (s, y) = step(state, a, reg);
                state = s; yld = y;
            }
            if per_seat == [n, n] { break; }
        }
        assert_eq!(per_seat, [n, n], "each seat reaches {n} decisions with a play");
        out
    }

    /// A1.5ci: plays come from the decider's own legal list and permanents,
    /// so each play's every action is legal, with the decider deciding, in
    /// every world sampled from its view, by A2.2's sampler and by PIMC's,
    /// and both search modes score the decisions (`apply_plan` asserts it
    /// there too). Six decisions with plays in each seat.
    #[test]
    fn a_plays_actions_are_legal_in_every_sampled_world() {
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        let mut checked = 0;
        for (state, decider, legal, cands) in decisions_with_plays(&reg, &decks, 6) {
            let view = project(&state, decider);
            for seed in 0..12u64 {
                for world in [sample_world(&view, &decks, &reg, seed, seed ^ 7),
                              determinize(&view, &decks, &reg, seed)] {
                    for k in cands.iter().filter(|k| k.is_play(legal.len())) {
                        let mut w = world.clone();
                        for a in &k.plan {
                            let wl = arcana_core::legal_actions::legal_actions(&w, &reg);
                            assert!(wl.contains(a), "world {seed}: planned {a:?} is not legal");
                            assert_eq!(w.priority.player, decider);
                            w = step(w, a.clone(), &reg).0;
                        }
                        checked += 1;
                    }
                }
            }
            ValueMcPolicy::sampled_with_budget(Box::new(MaterialValue), 3, 3, 20, 32, decks.clone())
                .score_actions(&state, &reg, decider, &legal);
            PimcPolicy::with_budget(3, decks.clone(), 3, 20, 32).score_actions(&state, &reg, decider, &legal);
        }
        assert!(checked >= 12 * 2 * 12, "{checked} plays checked");
    }

    /// A1.5ci: a chosen play is carried out over the following decisions
    /// without a search and ends in its cast; a plan whose next action is no
    /// longer legal, a decision of another player and a new game each drop
    /// the plan, and the policy searches again. With a Mountain and Lightning
    /// Bolt on the opponent's turn the search bolts the opponent: the tap, then
    /// the cast. `vmc-material` in both modes, PIMC and flat Monte Carlo, in
    /// both seats; two policies at one seed choose alike.
    #[test]
    fn a_chosen_play_is_carried_out_and_a_stale_plan_dropped() {
        let c = rule_cards();
        for decider in [0, 1] {
            let (s, legal, hand) = plays_table(&c, decider, false, &[c.mountain, c.mountain], &[c.bolt]);
            let decks = table_decks(&s);
            type Make<'a> = Box<dyn Fn() -> Box<dyn StatePolicy> + 'a>;
            let makers: Vec<(&str, Make)> = vec![
                ("vmc", Box::new(|| Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), 5, 4, 20, 16)))),
                ("vmc sampled", Box::new(|| Box::new(ValueMcPolicy::sampled_with_budget(
                    Box::new(MaterialValue), 5, 4, 20, 16, decks.clone())))),
                ("pimc", Box::new(|| Box::new(PimcPolicy::with_budget(5, decks.clone(), 4, 20, 16)))),
                ("flat", Box::new(|| Box::new(FlatMonteCarloPolicy::with_budget(5, 4, 20, 16)))),
            ];
            for (name, make) in &makers {
                // Carried out: the tap, then, without a search, the cast.
                let mut pol = make();
                let a1 = pol.choose(&s, &c.reg, decider, &legal);
                assert!(is_mana_activation(&s, &a1, &c.reg), "{name}, seat {decider}: chose {a1:?}");
                let (s1, y1) = step(s.clone(), a1.clone(), &c.reg);
                let EngineYield::PendingDecision { player, legal_actions: l1, .. } = y1 else { panic!() };
                assert_eq!(player, decider);
                let a2 = pol.choose(&s1, &c.reg, decider, &l1);
                let Action::CastSpell { object_id, targets, .. } = &a2 else { panic!("{name}: {a2:?}") };
                assert_eq!(*object_id, hand[0]);
                assert!(format!("{targets:?}").contains(&format!("Player({})", 1 - decider)),
                    "{name}, seat {decider}: the Bolt goes at the opponent: {targets:?}");
                // The same seed chooses alike.
                let mut twin = make();
                assert_eq!(twin.choose(&s, &c.reg, decider, &legal), a1);
                assert_eq!(twin.choose(&s1, &c.reg, decider, &l1), a2);
                // Stale: after the tap the decider passes, both pass, and the
                // upkeep ends with the pool; the planned cast is gone.
                let mut pol = make();
                let a1 = pol.choose(&s, &c.reg, decider, &legal);
                let s2 = rule_act(step(s.clone(), a1, &c.reg).0, &c, |_, a| a.is_pass());
                let s2 = rule_act(s2, &c, |_, a| a.is_pass());
                assert_ne!(s2.turn.step, arcana_core::turn::Step::Upkeep);
                let l2 = arcana_core::legal_actions::legal_actions(&s2, &c.reg);
                let who = s2.priority.player;
                let a = pol.choose(&s2, &c.reg, who, &l2);
                assert!(l2.contains(&a), "{name}: a stale plan's cast came back: {a:?}");
                // Another decider, then the decider again: dropped.
                let mut pol = make();
                let a1 = pol.choose(&s, &c.reg, decider, &legal);
                let (s1, y1) = step(s.clone(), a1, &c.reg);
                let EngineYield::PendingDecision { legal_actions: l1, .. } = y1 else { panic!() };
                let mut other = s1.clone();
                other.priority.give_to(1 - decider);
                let lo = arcana_core::legal_actions::legal_actions(&other, &c.reg);
                let ao = pol.choose(&other, &c.reg, 1 - decider, &lo);
                assert!(lo.contains(&ao));
                let again = pol.choose(&s1, &c.reg, decider, &l1);
                assert!(l1.contains(&again));
                // A new game: dropped.
                let mut pol = make();
                pol.choose(&s, &c.reg, decider, &legal);
                let (ng, ny) = new_game(decks.clone(), &c.reg, 3);
                let EngineYield::PendingDecision { player, legal_actions, .. } = ny else { panic!() };
                let a = pol.choose(&ng, &c.reg, player, &legal_actions);
                assert!(legal_actions.contains(&a), "{name}: a plan crossed into a new game");
            }
        }
    }

    /// A1.5ci: `KeptPlan` itself. It returns the planned actions in order
    /// while each is legal and the same player decides in the same step, and
    /// drops the rest at the first that is not, at another decider, in
    /// another step and at an empty plan.
    #[test]
    fn a_kept_plan_runs_in_order_and_drops_on_any_change() {
        let c = rule_cards();
        let (s, legal, _) = plays_table(&c, 0, true, &[c.forest, c.forest], &[c.bears]);
        let play = top_candidates(&s, &c.reg, 0, &legal).into_iter()
            .find(|k| k.is_play(legal.len())).unwrap().plan;
        let mut plan = KeptPlan::default();
        assert_eq!(plan.next(&s, 0, &legal), None, "an empty plan");
        plan.keep(&play, &s, 0);
        let s1 = step(s.clone(), play[0].clone(), &c.reg).0;
        let l1 = arcana_core::legal_actions::legal_actions(&s1, &c.reg);
        assert_eq!(plan.next(&s1, 1, &l1), None, "another decider drops it");
        assert!(plan.rest().is_empty());
        plan.keep(&play, &s, 0);
        assert_eq!(plan.next(&s1, 0, &l1), Some(play[1].clone()));
        let s2 = step(s1, play[1].clone(), &c.reg).0;
        let l2 = arcana_core::legal_actions::legal_actions(&s2, &c.reg);
        assert_eq!(plan.next(&s2, 0, &l2), Some(play[2].clone()), "the plan ends in its cast");
        assert_eq!(plan.next(&s2, 0, &l2), None);
        plan.keep(&play, &s, 0);
        let mut later = s.clone();
        later.turn.step = arcana_core::turn::Step::End;
        later.turn.phase = arcana_core::turn::Phase::Ending;
        assert_eq!(plan.next(&later, 0, &legal), None, "another step drops it");
        plan.keep(&play, &s, 0);
        assert_eq!(plan.next(&s, 0, &legal[..1]), None, "an action no longer legal drops it");
        assert!(plan.rest().is_empty());
    }

    /// The test arm `TopRule::Every` is the selection before A1.5ci: on the
    /// guard game's states, with or without mana abilities, `vmc-material`
    /// and PIMC under it choose and score what the policy did at `e276894a`
    /// would, which the recorded fixture checks over a whole game; here, that
    /// the pool is the legal list itself, and that over it both selections
    /// draw what they drew before: `select_candidates` the subset the
    /// verbatim copy below draws at the same seed, and `stable_pool_subset`
    /// the subset `stable_candidate_subset` keeps, whose text is unchanged.
    #[test]
    fn the_test_arm_every_is_the_legal_list() {
        /// `select_candidates` as it stood at `e276894a`, verbatim.
        fn select_candidates_before_a15ci(legal: &[Action], max_candidates: usize, rng: &mut ChaCha8Rng) -> Vec<usize> {
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
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        let mut subsets = 0;
        for (state, decider, legal) in guard_states(&reg, &decks) {
            let pool = TopRule::Every.pool(&state, &reg, decider, &legal);
            assert_eq!(pool, crate::plays::every_action(&legal));
            for cap in 1..legal.len() {
                for seed in 0..16 {
                    assert_eq!(select_candidates(&pool, cap, &mut ChaCha8Rng::seed_from_u64(seed)),
                               select_candidates_before_a15ci(&legal, cap, &mut ChaCha8Rng::seed_from_u64(seed)));
                }
                let kept: Vec<Action> = stable_pool_subset(&pool, cap).into_iter()
                    .map(|k| pool[k].plan[0].clone()).collect();
                assert_eq!(kept, stable_candidate_subset(&legal, cap));
                subsets += 1;
            }
        }
        assert!(subsets >= 2, "a subset is drawn only below the legal list's length");
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

    /// A2.2 true-state parity: `vmc-material` at A2.1's budget (6/25/10),
    /// rolling out from the true state, plays a whole recorded game exactly.
    /// The recording first pinned the game as it was played before the
    /// sampled-state mode existed (`2be0ed11`), and held on every engine to
    /// `767499b8`. A1.5s's timing repair changed the engine's game: the two
    /// agree to decision 410, where player 1's rollouts, now offered
    /// instant-speed activations, score the same five legal actions
    /// differently. The fixture is re-recorded on the repaired engine, so
    /// it now pins true-state play there. A1.5ce's repair, mana pools
    /// emptying as each step ends, changed the game again: the two agree
    /// to decision 108, in turn 5's declare attackers step, where player 1,
    /// offered the same four actions, used to activate a mana ability
    /// whose mana stayed in its pool to the end of the turn, and now
    /// passes; the older game went on to pay for two spells with mana
    /// floated in an earlier step (decisions 146 and 390 of its 477). The
    /// fixture was re-recorded again, at 390 decisions. A1.5cg's rollout
    /// rule, which keeps its lands for its own main phase, changed the game
    /// once more: the two agree to decision 68 and part at decision 69, in
    /// turn 3's second main phase, where player 0, offered the same five
    /// actions, used to pass with a land in hand and now plays it; nothing
    /// in the engine moved, and the choice changed in its rollouts. The
    /// fixture is re-recorded at 1080 decisions. Two fingerprints
    /// (FNV-1a) hold it. A1.5ci's plays as candidates changed it again: the
    /// two agree to decision 106 and part at decision 107, in turn 5's
    /// declare attackers step, where the old selection activated a mana
    /// ability whose mana the step's end would empty, and the new one passes.
    /// With plays the game is 1006 decisions and player 1 wins; its 86 mana
    /// abilities lose mana at one step end, where the old game's 216 lost it
    /// at 76. The 1080-decision game is kept as the record of the test arm
    /// `TopRule::Every`, the selection before the repair, which must still
    /// play it exactly.
    /// The portable one covers every scored candidate's index, every action and
    /// how far the deciding policy's stream has advanced after it, so a change
    /// in true-state mode's draws fails at the first decision it touches. The
    /// scores' one covers every candidate's index and value bits and every
    /// action, so a change in its rollouts or values fails even where the
    /// actions and draws would survive it. Both are checked on every machine:
    /// the scores' bits used to rest on the platform's `tanhf`, which glibc
    /// 2.39 and 2.44 disagree on by up to 2 ulp, and since A1.5u they come from
    /// `crate::portable`, which makes no platform transcendental call. The
    /// game is
    /// played by production `choose`; a second policy pair at the same seeds
    /// scores each decision in lockstep, and every production choice must be the
    /// first strict maximum of those scores.
    #[test]
    fn vmc_true_state_game_matches_the_recorded_fixture() {
        let plays = fixture_game(TopRule::Plays);
        assert_eq!((plays.0, plays.1), (PLAYS_FIXTURE.0, PLAYS_FIXTURE.1));
        assert_eq!(plays.2, PLAYS_FIXTURE.2, "portable fingerprint {:#018x}", plays.2);
        assert_eq!(plays.3, PLAYS_FIXTURE.3, "fingerprint {:#018x}", plays.3);
        let every = fixture_game(TopRule::Every);
        assert_eq!((every.0, every.1), (1080, GameResult::Win(0)));
        assert_eq!(every.2, 0xd1fd_1860_41db_d8c7, "portable fingerprint {:#018x}", every.2);
        assert_eq!(every.3, 0x39a5_ebea_8c4d_b895, "fingerprint {:#018x}", every.3);
    }

    /// The fixture game with plays as candidates: its decisions, result and
    /// two fingerprints.
    const PLAYS_FIXTURE: (u32, GameResult, u64, u64) =
        (1006, GameResult::Win(1), 0x9248_41f1_03cd_2b9e, 0x3c61_904d_3b60_7a13);

    /// The fixture game with `top` as the pool both seats search: decisions,
    /// result, the portable fingerprint, the scores' fingerprint, and the
    /// actions in order. A decision that carries out a kept play is not
    /// scored (the policy does not search there); its action must be the
    /// play's next one.
    fn fixture_game(top: TopRule) -> (u32, GameResult, u64, u64, Vec<Action>) {
        fn eat(h: &mut u64, b: &[u8]) { for &x in b { *h ^= x as u64; *h = h.wrapping_mul(0x0100_0000_01b3); } }
        const FNV: u64 = 0xcbf2_9ce4_8422_2325;
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        let pair = || {
            let mut pair = [ValueMcPolicy::with_budget(Box::new(MaterialValue), 11, 6, 25, 10),
                            ValueMcPolicy::with_budget(Box::new(MaterialValue), 12, 6, 25, 10)];
            for p in &mut pair { p.top = top; }
            pair
        };
        let (mut players, mut scorers) = (pair(), pair());
        let mut following: [Vec<Action>; 2] = [Vec::new(), Vec::new()];
        let (mut state, mut yld) = new_game(decks, &reg, 2026);
        let (mut h, mut p) = (FNV, FNV);
        let mut decisions = 0;
        let mut actions = Vec::new();
        let result = loop {
            let (player, legal) = match yld {
                EngineYield::PendingDecision { player, legal_actions, .. } => (player, legal_actions),
                EngineYield::GameOver(r) => break r,
            };
            assert!(decisions < 4000, "the fixture game ends");
            // Production choice plays the game; the scorer, in lockstep on the
            // same stream, holds the scores it must be the first maximum of.
            let a = players[player as usize].choose(&state, &reg, player, &legal);
            let next = &mut following[player as usize];
            if !next.is_empty() {
                assert_eq!(a, next.remove(0), "decision {decisions}: the kept play is carried out");
            } else if legal.len() > 1 {
                let sc = scorers[player as usize].score_candidates(&state, &reg, player, &legal);
                let (mut bi, mut bv) = (0, f32::NEG_INFINITY);
                for (k, (cand, v)) in sc.iter().enumerate() {
                    eat(&mut h, format!("{}:{};", cand.index, v.to_bits()).as_bytes());
                    eat(&mut p, format!("{};", cand.index).as_bytes());
                    if *v > bv { bv = *v; bi = k; }
                }
                assert_eq!(a, sc[bi].0.plan[0], "decision {decisions}: choose is not the scores' first maximum");
                *next = sc[bi].0.plan[1..].to_vec();
            }
            eat(&mut h, format!("{a:?};").as_bytes());
            eat(&mut p, format!("{a:?}@{};", players[player as usize].rng.get_word_pos()).as_bytes());
            actions.push(a.clone());
            let (s, y) = step(state, a, &reg);
            state = s; yld = y; decisions += 1;
        };
        (decisions, result, p, h, actions)
    }

    /// Give objects `a` and `b` each other's printed identity, keeping their
    /// ids, owners and zones, as a game setup would have instantiated them.
    fn swap_identity(state: &mut GameState, reg: &CardRegistry, a: ObjectId, b: ObjectId) {
        let (oa, ob) = (state.objects.get(a).unwrap().clone(), state.objects.get(b).unwrap().clone());
        state.objects.remove(a);
        state.objects.remove(b);
        state.objects.insert(arcana_core::engine::instantiate_card_object(reg, a, oa.owner, oa.zone, ob.card_id));
        state.objects.insert(arcana_core::engine::instantiate_card_object(reg, b, ob.owner, ob.zone, oa.card_id));
    }

    /// A state `decider` cannot tell from `state`: an opponent's hand card and
    /// library card trade identities, another opponent hand object and library
    /// object trade places (the hidden ids' division between hand and library),
    /// two of the decider's own library cards trade identities, every library's
    /// hidden positions are reversed, and the engine RNG seed differs. Panics
    /// if a change cannot be made, so a twin that silently equals its original
    /// cannot pass the guard.
    fn visible_twin(state: &GameState, reg: &CardRegistry, decider: PlayerId) -> GameState {
        let view = project(state, decider);
        let mut twin = state.clone();
        let opp = 1 - decider;
        let opp_hand: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Hand(opp)).map(|o| o.id).collect();
        let opp_lib = &state.player(opp).library_top_to_bottom;
        let (h, l) = opp_hand.iter().flat_map(|&h| opp_lib.iter().map(move |&l| (h, l)))
            .find(|&(h, l)| state.objects.get(h).unwrap().card_id != state.objects.get(l).unwrap().card_id)
            .expect("an opponent hand card and library card with different identities");
        swap_identity(&mut twin, reg, h, l);
        let (h2, l2) = (*opp_hand.iter().find(|&&x| x != h).expect("two opponent hand cards"),
                        *opp_lib.iter().find(|&&x| x != l).expect("two opponent library cards"));
        for (id, zone) in [(h2, Zone::Library(opp)), (l2, Zone::Hand(opp))] {
            let mut o = twin.objects.remove(id).unwrap();
            o.zone = zone;
            twin.objects.insert(o);
        }
        for slot in twin.player_mut(opp).library_top_to_bottom.iter_mut() {
            if *slot == l2 { *slot = h2; }
        }
        let own_lib = &state.player(decider).library_top_to_bottom;
        let (a, b) = own_lib.iter().flat_map(|&a| own_lib.iter().map(move |&b| (a, b)))
            .find(|&(a, b)| state.objects.get(a).unwrap().card_id != state.objects.get(b).unwrap().card_id)
            .expect("two own library cards with different identities");
        swap_identity(&mut twin, reg, a, b);
        for p in 0..state.num_players() {
            let lib = &mut twin.player_mut(p).library_top_to_bottom;
            let slots: Vec<usize> = (0..lib.len()).filter(|&k| view.anonymous_ids.contains(&lib[k])).collect();
            let ids: Vec<ObjectId> = slots.iter().rev().map(|&k| lib[k]).collect();
            assert!(ids.len() > 1, "player {p}'s library has hidden positions to reorder");
            for (&k, id) in slots.iter().zip(ids) {
                lib[k] = id;
            }
        }
        twin.rng_seed = state.rng_seed ^ 0x7E57_0F5E_ED00_0001;
        twin
    }

    /// The game at its first decision (the active player's mulligan, where a
    /// mulligan reshuffles through the engine seed), the first decision on
    /// turn 3 or later with at least three legal actions, and decision 87,
    /// where a review found sampled scores at 6/25/10 depending on which hidden
    /// ids sat in the opponent's hand. Reached by random play.
    fn guard_states(reg: &CardRegistry, decks: &[Vec<CardId>]) -> Vec<(GameState, PlayerId, Vec<Action>)> {
        let mut out = Vec::new();
        let (mut state, mut yld) = new_game(decks.to_vec(), reg, 77);
        let mut rand = RandomStatePolicy::new(5);
        let mut turn3 = false;
        for decision in 0..2000 {
            let EngineYield::PendingDecision { player, legal_actions, .. } = yld else { break };
            let first_turn3 = !turn3 && state.turn.turn_number >= 3 && legal_actions.len() >= 3;
            if decision == 0 || first_turn3 || decision == 87 {
                out.push((state.clone(), player, legal_actions.clone()));
                turn3 |= first_turn3;
            }
            if out.len() == 3 { break; }
            let a = rand.choose(&state, reg, player, &legal_actions);
            let (s, y) = step(state, a, reg);
            state = s; yld = y;
        }
        assert_eq!(out.len(), 3, "the three guard states are reached");
        out
    }

    fn bits(scored: &[(usize, f32)]) -> Vec<(usize, u32)> {
        scored.iter().map(|&(i, v)| (i, v.to_bits())).collect()
    }

    /// A2.2's arm-I guard: sampled-state mode's scores and choice depend only
    /// on what the decider can see. A state and its visible twin (different
    /// hidden identities, library order and engine seed) give bit-identical
    /// scores and the same choice at one seed, where true-state mode, which
    /// sees all three, scores them differently. Fails if sampled mode rolls
    /// out from the true state, keeps the game's engine seed in its worlds,
    /// or lets the hidden ids' placement (hand or library, library order)
    /// reach its worlds. Every state is checked at 400-step rollouts, so that
    /// from the mulligan decision they reach played cards and the reshuffle
    /// (at 25 true-state mode cannot tell that twin apart there, and since
    /// A1.5ce, with mana pools emptying as each step ends, not at 150
    /// either, where the guard ran before). The
    /// registered depth is held by
    /// [`sampled_vmc_ignores_which_hidden_ids_sit_in_hand`].
    #[test]
    fn sampled_vmc_depends_only_on_what_the_decider_sees() {
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        for (k, (state, decider, legal)) in guard_states(&reg, &decks).into_iter().enumerate() {
            let twin = visible_twin(&state, &reg, decider);
            let truth = |st: &GameState| {
                ValueMcPolicy::with_budget(Box::new(MaterialValue), 31, 6, 400, 10)
                    .score_actions(st, &reg, decider, &legal)
            };
            assert_ne!(bits(&truth(&state)), bits(&truth(&twin)),
                "state {k}: true-state mode must tell the twin apart, or the guard has no teeth");
            // The worlds themselves, in the arena's iteration order: identical
            // object by object, so no hidden placement reaches them.
            let world = |st: &GameState, seed: u64| {
                let w = sample_world(&project(st, decider), &decks, &reg, seed, seed ^ 1);
                let objs: Vec<String> = w.objects.iter()
                    .map(|o| format!("{}:{}:{:?}", o.id, o.card_id, o.zone)).collect();
                let libs: Vec<Vec<ObjectId>> = (0..w.num_players())
                    .map(|p| w.player(p).library_top_to_bottom.clone()).collect();
                (objs, libs, w.rng_seed)
            };
            for seed in 0..16 {
                assert_eq!(world(&state, seed), world(&twin, seed), "state {k}: world {seed} differs");
            }
            let sampled = || ValueMcPolicy::sampled_with_budget(
                Box::new(MaterialValue), 31, 6, 400, 10, decks.clone());
            assert_eq!(bits(&sampled().score_actions(&state, &reg, decider, &legal)),
                       bits(&sampled().score_actions(&twin, &reg, decider, &legal)),
                       "state {k}: sampled mode scored the twin differently");
            assert_eq!(sampled().choose(&state, &reg, decider, &legal),
                       sampled().choose(&twin, &reg, decider, &legal));
        }
    }

    /// A2.2's arm-I guard at the registered 6/25/10, on decision 87 of the
    /// guard game, where a review found sampled scores depending on which of
    /// the opponent's hidden ids sat in the hand: each single exchange of an
    /// opponent hand object with one of the top twelve hidden library objects,
    /// nothing else changed, leaves the scores bit-identical. Without the
    /// canonical layout 95 of 216 such exchanges (three policy seeds) changed
    /// them.
    #[test]
    fn sampled_vmc_ignores_which_hidden_ids_sit_in_hand() {
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        let (state, decider, legal) = guard_states(&reg, &decks).swap_remove(2);
        let opp = 1 - decider;
        let hand: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Hand(opp)).map(|o| o.id).collect();
        let lib: Vec<ObjectId> = state.player(opp).library_top_to_bottom.iter().take(12).copied().collect();
        assert!(hand.len() >= 2 && lib.len() == 12 && legal.len() >= 2, "decision 87 has the shape the guard needs");
        let scores = |st: &GameState| bits(&ValueMcPolicy::sampled_with_budget(
            Box::new(MaterialValue), 31, 6, 25, 10, decks.clone()).score_actions(st, &reg, decider, &legal));
        let base = scores(&state);
        for &h in &hand {
            for &l in &lib {
                let mut twin = state.clone();
                for (id, zone) in [(h, Zone::Library(opp)), (l, Zone::Hand(opp))] {
                    let mut o = twin.objects.remove(id).unwrap();
                    o.zone = zone;
                    twin.objects.insert(o);
                }
                for slot in twin.player_mut(opp).library_top_to_bottom.iter_mut() {
                    if *slot == l { *slot = h; }
                }
                assert_eq!(scores(&twin), base, "exchanging hand {h} with library {l} changed the scores");
            }
        }
    }

    /// A1.5ci beside A2.2's arm-I guard, whose three states offer no play:
    /// on decisions where plays are among the candidates, a state and its
    /// visible twin give bit-identical sampled scores and the same choice,
    /// plays scored, where true-state mode scores them differently. Plays are
    /// built from the true state's legal list; this holds that what they
    /// carry into the worlds is the decider's own. Two decisions a seat.
    #[test]
    fn sampled_vmc_with_plays_depends_only_on_what_the_decider_sees() {
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        let mut per_seat = [0usize; 2];
        for (state, decider, legal, cands) in decisions_with_plays(&reg, &decks, 12) {
            if per_seat[decider as usize] == 2 { continue; }
            let Ok(twin) = std::panic::catch_unwind(|| visible_twin(&state, &reg, decider)) else { continue };
            let n_plays = cands.iter().filter(|k| k.is_play(legal.len())).count();
            let sampled = || ValueMcPolicy::sampled_with_budget(
                Box::new(MaterialValue), 31, 6, 400, 32, decks.clone());
            let scored = sampled().score_actions(&state, &reg, decider, &legal);
            assert_eq!(scored.iter().filter(|&&(i, _)| i >= legal.len()).count(), n_plays,
                "every play is scored");
            let truth = |st: &GameState| {
                ValueMcPolicy::with_budget(Box::new(MaterialValue), 31, 6, 400, 32)
                    .score_actions(st, &reg, decider, &legal)
            };
            assert_ne!(bits(&truth(&state)), bits(&truth(&twin)),
                "true-state mode must tell the twin apart, or the guard has no teeth");
            assert_eq!(bits(&scored), bits(&sampled().score_actions(&twin, &reg, decider, &legal)),
                "seat {decider}: sampled mode scored the twin differently");
            assert_eq!(sampled().choose(&state, &reg, decider, &legal),
                       sampled().choose(&twin, &reg, decider, &legal));
            per_seat[decider as usize] += 1;
        }
        assert_eq!(per_seat, [2, 2], "two decisions with plays and a visible twin in each seat");
    }

    /// A2.2's arm-I guard: both modes select the same candidates at a first
    /// decision from one seed, since worlds draw from their own stream.
    #[test]
    fn sampled_and_true_state_modes_select_the_same_candidates() {
        let reg = arcana_cards::build_catalog();
        let decks = vec![arcana_cards::sample_deck(&reg, 4), arcana_cards::sample_deck(&reg, 9)];
        let (state, decider, legal) = guard_states(&reg, &decks).swap_remove(1);
        assert!(legal.len() > 2, "a subset is drawn only when legal exceeds the candidate cap");
        for seed in 0..8 {
            let idx = |mut pol: ValueMcPolicy| -> Vec<usize> {
                pol.score_actions(&state, &reg, decider, &legal).iter().map(|&(i, _)| i).collect()
            };
            let t = idx(ValueMcPolicy::with_budget(Box::new(MaterialValue), seed, 2, 25, 2));
            let s = idx(ValueMcPolicy::sampled_with_budget(Box::new(MaterialValue), seed, 2, 25, 2, decks.clone()));
            assert_eq!(t, s, "seed {seed}: the modes selected different candidates");
        }
    }
}
