//! Monte-Carlo search baseline + a state-aware policy interface and an
//! evaluation harness (win-rate between two policies).
//!
//! This is the first rung of the RL ladder: a policy that actually *searches*
//! and beats random play, plus the empirical feedback loop to measure it. It
//! is PERFECT-INFORMATION — [`FlatMonteCarloPolicy`] rolls out from the true
//! [`GameState`] via the engine's `step`, ignoring hidden information. That's a
//! legitimate baseline and the search infrastructure; the principled
//! imperfect-information upgrade (ISMCTS over [`crate::information_set::
//! determinize`]) is the next phase and needs a deck-list type first.
//!
//! The [`Policy`](crate::selfplay::Policy) trait in `selfplay` takes an ENCODED
//! observation (for reactive neural policies). A search policy instead needs
//! the live state to simulate from, so it uses the [`StatePolicy`] trait here.

use arcana_core::actions::Action;
use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::registry::CardRegistry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::types::{CardId, PlayerId};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

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
    fn rollout_value(&mut self, mut state: GameState, mut yld: EngineYield,
                     registry: &CardRegistry, decider: PlayerId) -> f32 {
        let mut steps = 0u32;
        loop {
            match yld {
                EngineYield::GameOver(_) => break,
                EngineYield::PendingDecision { legal_actions, .. } => {
                    if steps >= self.rollout_step_cap || legal_actions.is_empty() { break; }
                    let i = progress_pick(&legal_actions, &mut self.rng);
                    let (s, y) = step(state, legal_actions[i].clone(), registry);
                    state = s; yld = y; steps += 1;
                }
            }
        }
        value(&state, decider)
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
}
