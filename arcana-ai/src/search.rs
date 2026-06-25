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
use arcana_core::types::{CardId, PlayerId};
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

/// Terminal/heuristic value of `state` from `player`'s perspective, in
/// roughly [-1, 1]. A decided game is ±1 / 0; an undecided (rollout-capped)
/// state falls back to a squashed life differential, kept strictly inside
/// ±1 so a real win always dominates a heuristic estimate.
pub fn value(state: &GameState, player: PlayerId) -> f32 {
    match state.result {
        Some(GameResult::Win(p)) => if p == player { 1.0 } else { -1.0 },
        Some(GameResult::Draw) => 0.0,
        Some(GameResult::Eliminated(p)) => if p == player { -1.0 } else { 1.0 },
        None => {
            let me = state.player(player).life as f32;
            let opp = state.opponents_of(player)
                .map(|o| state.player(o).life)
                .max()
                .unwrap_or(0) as f32;
            ((me - opp) / 40.0).clamp(-0.9, 0.9)
        }
    }
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

/// Play `state`/`yld` forward with progress-biased random choices until the
/// game is decided or `step_cap` steps elapse, returning the final state. The
/// shared rollout engine for both flat-MC and IS-MCTS simulations.
fn play_out(mut state: GameState, mut yld: EngineYield, registry: &CardRegistry,
            step_cap: u32, rng: &mut ChaCha8Rng) -> GameState {
    let mut steps = 0u32;
    loop {
        match yld {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { legal_actions, .. } => {
                if steps >= step_cap || legal_actions.is_empty() { break; }
                let i = progress_pick(&legal_actions, rng);
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

impl StatePolicy for PimcPolicy {
    fn choose(&mut self, state: &GameState, registry: &CardRegistry,
              decider: PlayerId, legal: &[Action]) -> Action {
        if legal.len() <= 1 { return legal[0].clone(); }
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
        let mut best = 0usize;
        for i in 1..cands.len() {
            if sums[i] > sums[best] { best = i; }
        }
        cands[best].clone()
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
