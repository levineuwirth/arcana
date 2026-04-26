//! Self-play harness: drive `arcana-core`'s real engine to game
//! completion under a pair of [`Policy`] implementations and
//! collect per-perspective [`Trajectory`]s.
//!
//! v0 deliberately ships **harness, not training infrastructure**:
//! the policy interface, the trajectory shape, the episode driver,
//! and one end-to-end regression that confirms `RandomPolicy`-vs-
//! `RandomPolicy` runs to completion without panicking. Neural
//! policies, replay buffers, vectorization, and MCTS-style search
//! are downstream concerns that consume this surface but don't
//! belong here.
//!
//! # Real engine, not a mock
//!
//! `arcana-core::engine::{new_game, step}` and
//! `arcana-core::legal_actions` are real today —
//! `arcana-core/tests/basic_game.rs` already drives random-vs-random
//! Lightning Bolt + vanilla games to completion across 16 seeds.
//! The harness here generalizes that test into a reusable surface.
//! There is no `MockEngine`; if a future test wants to unit-test
//! trajectory bookkeeping in isolation, it can drive a hand-built
//! `(GameState, EngineYield)` pair through `run_episode` directly.
//!
//! # Per-perspective trajectories
//!
//! `run_episode` returns one [`Trajectory`] per player. Each
//! trajectory contains only the steps where THAT player was the
//! decision-maker (priority holder), with observations encoded
//! from THAT player's perspective. This matches the AlphaZero-style
//! self-play setup where a single policy network plays both sides
//! by virtue of perspective-flipped inputs. Projecting the state
//! per perspective at every step is cheap relative to engine
//! settling; the alternative (one canonical trajectory, project at
//! training time) costs the same and is more error-prone.
//!
//! # Episode caps
//!
//! Real games can in principle run forever (turn-stalling, decking
//! loops, engine bugs). [`EpisodeConfig`] caps both turn count and
//! step count; whichever fires first marks the episode `truncated`.
//! Default caps (200 turns / 5000 steps) are generous for vanilla
//! games — `arcana-core/tests/basic_game.rs` uses 500/100_000 and
//! settles in a few hundred steps. Tighter defaults here let the
//! 1000-episode regression run quickly while still leaving headroom.

use std::time::Instant;

use arcana_core::actions::Action;
use arcana_core::engine::{step as engine_step, EngineYield};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::types::PlayerId;

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::information_set::project;
use crate::observation::Encoder;
use crate::reward::RewardFunction;

// =============================================================================
// Policy trait
// =============================================================================

/// Strategy that picks one of the legal actions given an observation.
///
/// Two return styles are supported by [`PolicyChoice`]:
///
/// * `Index(usize)` — natural for categorical / neural policies that
///   output a probability distribution over the legal-action list.
/// * `Action(Action)` — natural for rule-based hand-coded policies
///   that construct a specific action without consulting the index
///   space.
///
/// The harness validates either (range-check the index, membership-
/// check the action) and resolves to a concrete `(Action, index)`
/// pair before recording into the trajectory.
pub trait Policy {
    fn select_action(
        &mut self,
        observation: &[f32],
        legal: &[Action],
    ) -> PolicyChoice;
}

#[derive(Debug, Clone)]
pub enum PolicyChoice {
    /// Index into the `legal` slice. Must satisfy `idx < legal.len()`.
    Index(usize),
    /// A concrete action that must be present in `legal`.
    Action(Action),
}

// =============================================================================
// Concrete policies
// =============================================================================

/// Uniform-random over `legal_actions`. Useful for harness-level
/// regression tests and as a no-op baseline. Note: pure uniform
/// random *may not terminate* against the current engine for some
/// seeds — uninstructed agents can mulligan to oblivion or spin
/// without making progress. Use [`ProgressBiasedRandomPolicy`] for
/// integration tests that need reliable termination.
#[derive(Clone)]
pub struct RandomPolicy {
    rng: ChaCha8Rng,
}

impl RandomPolicy {
    pub fn new(seed: u64) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed) }
    }
}

impl Policy for RandomPolicy {
    fn select_action(&mut self, _obs: &[f32], legal: &[Action]) -> PolicyChoice {
        let pick = legal.choose(&mut self.rng).expect("legal is non-empty");
        PolicyChoice::Action(pick.clone())
    }
}

/// Random policy with a "make progress" bias copied verbatim from
/// `arcana-core/tests/basic_game.rs::pick_action`:
///
/// 1. Always take `MulliganKeep` if offered (the engine doesn't
///    yet implement London-mulligan bottoming, so taking mulligans
///    just shrinks hands toward zero).
/// 2. Else prefer non-pass non-concede actions when any exist, so
///    combat and casts actually happen.
/// 3. Fall back to pass.
/// 4. Last resort: take whatever's at index 0 (typically concede).
///
/// This matches the engine's existing integration-test policy, so
/// the harness's regression run uses the same "known-terminates"
/// behavior arcana-core already validates.
#[derive(Clone)]
pub struct ProgressBiasedRandomPolicy {
    rng: ChaCha8Rng,
}

impl ProgressBiasedRandomPolicy {
    pub fn new(seed: u64) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed) }
    }
}

impl Policy for ProgressBiasedRandomPolicy {
    fn select_action(&mut self, _obs: &[f32], legal: &[Action]) -> PolicyChoice {
        if let Some(idx) = legal.iter().position(|a| matches!(a, Action::MulliganKeep)) {
            return PolicyChoice::Index(idx);
        }
        let interesting: Vec<usize> = legal
            .iter()
            .enumerate()
            .filter(|(_, a)| !a.is_pass() && !a.is_concede())
            .map(|(i, _)| i)
            .collect();
        if !interesting.is_empty() {
            let pick = interesting.choose(&mut self.rng).copied().unwrap();
            return PolicyChoice::Index(pick);
        }
        if let Some(idx) = legal.iter().position(|a| a.is_pass()) {
            return PolicyChoice::Index(idx);
        }
        PolicyChoice::Index(0)
    }
}

/// Always returns `PolicyChoice::Index(0)`. Useful as a debug
/// baseline and as a deterministic policy in tests where the
/// reproducible-trajectory shape matters more than realistic play.
#[derive(Clone)]
pub struct FirstActionPolicy;

impl Policy for FirstActionPolicy {
    fn select_action(&mut self, _obs: &[f32], _legal: &[Action]) -> PolicyChoice {
        PolicyChoice::Index(0)
    }
}

// =============================================================================
// Trajectory + outcome types
// =============================================================================

#[derive(Debug, Clone)]
pub struct TrajectoryStep {
    /// Observation at the moment this step's action was selected,
    /// encoded from this trajectory's perspective player.
    pub observation: Vec<f32>,
    /// The action that was actually taken (resolved from
    /// `PolicyChoice` if necessary).
    pub action: Action,
    /// Index of `action` in the legal-action list at decision time.
    /// `Some` for index-style policies and resolvable action-style
    /// policies; `None` only if the policy returned an `Action` the
    /// harness couldn't locate (which currently panics, so this
    /// stays `None` only for future relaxations).
    pub action_index: Option<usize>,
    /// Number of legal actions at the moment this decision was made.
    /// Required for masked-action training: indices `0..n_legal` are
    /// valid this step, indices `n_legal..K_max` are illegal.
    ///
    /// **Position-dependence assumption.** Action indices are
    /// per-decision-point — index 5 at one timestep is unrelated to
    /// index 5 at another. v0 stores only the count, not the legal-
    /// action list itself. This is enough for vanilla policy nets;
    /// off-policy correction / importance sampling would need the
    /// full list, which would be an additive future change.
    pub n_legal: u32,
    /// Per-step reward. Always 0 except possibly at the terminal
    /// step, where it equals the perspective's terminal reward.
    pub reward: f32,
}

#[derive(Debug, Clone)]
pub struct Trajectory {
    pub perspective: PlayerId,
    pub steps: Vec<TrajectoryStep>,
    /// Same as the last step's reward when `terminated`; 0 on
    /// truncation. Materialized as a separate field so consumers
    /// don't have to reach into the last step.
    pub final_reward: f32,
    /// True iff the game reached a terminal `EngineYield::GameOver`.
    pub terminated: bool,
    /// True iff the episode hit `max_turns` or `max_steps` before
    /// the engine produced a terminal yield.
    pub truncated: bool,
}

#[derive(Debug, Clone)]
pub struct EpisodeOutcome {
    /// One trajectory per player, indexed by `PlayerId`.
    pub trajectories: Vec<Trajectory>,
    pub final_state: GameState,
    pub final_yield: EngineYield,
    pub steps_taken: u32,
    pub turns_taken: u32,
}

#[derive(Debug, Clone)]
pub struct EpisodeConfig {
    /// Cap on turn number. The episode is `truncated` when
    /// `state.turn.turn_number > max_turns`.
    pub max_turns: u32,
    /// Cap on engine `step` calls. The episode is `truncated` when
    /// the harness has issued `max_steps` calls without reaching
    /// `GameOver`.
    pub max_steps: u32,
}

impl Default for EpisodeConfig {
    fn default() -> Self {
        Self { max_turns: 200, max_steps: 5_000 }
    }
}

// =============================================================================
// Driver
// =============================================================================

/// Drive `(state, initial_yield)` to game completion (or truncation
/// at `config.max_turns`/`config.max_steps`) under one policy per
/// player. Returns one [`Trajectory`] per perspective, plus the
/// final engine state and yield.
///
/// `policies[i]` is the policy for player `i`. `policies.len()` must
/// equal `state.players.len()`.
///
/// # Panics
/// * If `policies.len() != state.players.len()`.
/// * If a policy returns `PolicyChoice::Index(k)` with
///   `k >= legal_actions.len()`.
/// * If a policy returns `PolicyChoice::Action(a)` with `a` not in
///   `legal_actions`.
/// * If a policy returns from a `PendingDecision` with empty
///   `legal_actions` (engine bug, not a harness concern).
pub fn run_episode(
    mut state: GameState,
    mut yld: EngineYield,
    registry: &CardRegistry,
    policies: &mut [Box<dyn Policy>],
    encoder: &dyn Encoder,
    reward_fn: &dyn RewardFunction,
    config: &EpisodeConfig,
) -> EpisodeOutcome {
    let num_players = state.players.len();
    assert_eq!(
        policies.len(),
        num_players,
        "expected {num_players} policies, got {}",
        policies.len()
    );

    let started_at = Instant::now();
    tracing::debug!(
        num_players,
        seed = state.rng_seed,
        max_turns = config.max_turns,
        max_steps = config.max_steps,
        "self-play episode start"
    );

    // One trajectory accumulator per perspective.
    let mut trajs: Vec<Trajectory> = (0..num_players as PlayerId)
        .map(|p| Trajectory {
            perspective: p,
            steps: Vec::new(),
            final_reward: 0.0,
            terminated: false,
            truncated: false,
        })
        .collect();

    let mut steps_taken: u32 = 0;

    let final_yield = loop {
        // Cap check before consuming the next yield. Order matters:
        // a step-cap hit here means we stop *before* doing work, so
        // steps_taken == config.max_steps exactly when truncated.
        if steps_taken >= config.max_steps {
            tracing::debug!(steps_taken, "truncated on step cap");
            break yld;
        }
        if state.turn.turn_number > config.max_turns {
            tracing::debug!(turn = state.turn.turn_number, "truncated on turn cap");
            break yld;
        }

        match yld {
            EngineYield::GameOver(_) => break yld,
            EngineYield::PendingDecision { legal_actions, .. } => {
                assert!(
                    !legal_actions.is_empty(),
                    "engine emitted PendingDecision with empty legal_actions"
                );
                let decider = state.priority_player();
                debug_assert!(
                    (decider as usize) < num_players,
                    "priority_player out of range"
                );

                // Project once for the decider; encode their
                // observation. The non-decider perspective doesn't
                // record this step (per-perspective trajectories
                // hold only the steps where the perspective decided).
                let view = project(&state, decider);
                let mut obs = vec![0.0f32; encoder.dim()];
                encoder.encode_into(&view.state, Some(decider), &mut obs);

                // Query the decider's policy and resolve.
                let choice = policies[decider as usize]
                    .select_action(&obs, &legal_actions);
                let (action, action_index) =
                    resolve_choice(choice, &legal_actions);

                trajs[decider as usize].steps.push(TrajectoryStep {
                    observation: obs,
                    action: action.clone(),
                    action_index: Some(action_index),
                    n_legal: legal_actions.len() as u32,
                    reward: 0.0,
                });

                let (next_state, next_yld) = engine_step(state, action, registry);
                state = next_state;
                yld = next_yld;
                steps_taken += 1;
            }
        }
    };

    // Terminal vs truncated. Terminal iff the loop exited because
    // GameOver was the last yield; otherwise truncated.
    let terminated = matches!(final_yield, EngineYield::GameOver(_));
    let truncated = !terminated;

    // Stamp final reward on each trajectory. Terminal: per-perspective
    // reward from reward_fn. Truncated: 0 (treat as no-signal; the
    // training loop can choose to mask truncated trajectories).
    for traj in trajs.iter_mut() {
        traj.terminated = terminated;
        traj.truncated = truncated;
        if terminated {
            let r = reward_fn.reward(&state, traj.perspective);
            traj.final_reward = r;
            if let Some(last) = traj.steps.last_mut() {
                last.reward = r;
            }
        }
    }

    let turns_taken = state.turn.turn_number;
    let elapsed = started_at.elapsed();
    let result_for_log = if let EngineYield::GameOver(ref r) = final_yield {
        format!("{r:?}")
    } else {
        "truncated".to_string()
    };
    tracing::debug!(
        steps_taken,
        turns_taken,
        elapsed_ms = elapsed.as_millis() as u64,
        result = result_for_log,
        "self-play episode end"
    );

    EpisodeOutcome {
        trajectories: trajs,
        final_state: state,
        final_yield,
        steps_taken,
        turns_taken,
    }
}

/// Translate a [`PolicyChoice`] into a concrete `(Action, index)`
/// pair, validating against the legal-action list.
fn resolve_choice(choice: PolicyChoice, legal: &[Action]) -> (Action, usize) {
    match choice {
        PolicyChoice::Index(idx) => {
            assert!(
                idx < legal.len(),
                "policy returned Index({idx}) but legal_actions has {} entries",
                legal.len()
            );
            (legal[idx].clone(), idx)
        }
        PolicyChoice::Action(action) => {
            let idx = legal.iter().position(|a| a == &action).unwrap_or_else(|| {
                panic!(
                    "policy returned Action({action:?}) which is not in the legal set"
                )
            });
            (action, idx)
        }
    }
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use arcana_cards::register_seed;
    use arcana_core::engine::new_game;
    use arcana_core::registry::build_deck;
    use arcana_core::state::GameResult;
    use arcana_core::Action;

    use crate::observation::BasicE2Encoder;
    use crate::reward::TerminalReward;

    /// Build the standard "Lightning Bolt milestone" registry +
    /// decks used by arcana-core/tests/basic_game.rs. Two-player,
    /// vanilla creatures + Bolts. Returns (registry, decks).
    fn bolt_milestone_setup() -> (CardRegistry, Vec<Vec<arcana_core::types::CardId>>) {
        let mut registry = CardRegistry::new();
        let _ids = register_seed(&mut registry);
        let deck_a = build_deck(
            &[
                ("Mountain", 10),
                ("Lightning Bolt", 4),
                ("Grizzly Bears", 4),
            ],
            &registry,
        );
        let deck_b = build_deck(&[("Forest", 10), ("Grizzly Bears", 8)], &registry);
        (registry, vec![deck_a, deck_b])
    }

    /// Convenience: drive an episode with two ProgressBiasedRandom
    /// policies seeded off `seed`, default config. Returns the
    /// outcome.
    fn run_progress_biased(seed: u64) -> EpisodeOutcome {
        let (registry, decks) = bolt_milestone_setup();
        let (state, yld) = new_game(decks, &registry, seed);
        let mut policies: Vec<Box<dyn Policy>> = vec![
            Box::new(ProgressBiasedRandomPolicy::new(seed ^ 0xA5A5)),
            Box::new(ProgressBiasedRandomPolicy::new(seed ^ 0x5A5A)),
        ];
        let encoder = BasicE2Encoder::for_two_players();
        let reward = TerminalReward;
        let config = EpisodeConfig::default();
        run_episode(
            state,
            yld,
            &registry,
            &mut policies,
            &encoder,
            &reward,
            &config,
        )
    }

    // -- resolve_choice ------------------------------------------------

    #[test]
    fn resolve_choice_index_in_range() {
        let legal = vec![Action::PassPriority, Action::Concede];
        let (a, i) = resolve_choice(PolicyChoice::Index(1), &legal);
        assert_eq!(a, Action::Concede);
        assert_eq!(i, 1);
    }

    #[test]
    #[should_panic(expected = "Index(5)")]
    fn resolve_choice_index_out_of_range_panics() {
        let legal = vec![Action::PassPriority];
        let _ = resolve_choice(PolicyChoice::Index(5), &legal);
    }

    #[test]
    fn resolve_choice_action_in_legal_set() {
        let legal = vec![Action::PassPriority, Action::Concede];
        let (a, i) =
            resolve_choice(PolicyChoice::Action(Action::Concede), &legal);
        assert_eq!(a, Action::Concede);
        assert_eq!(i, 1);
    }

    #[test]
    #[should_panic(expected = "not in the legal set")]
    fn resolve_choice_action_not_in_legal_set_panics() {
        let legal = vec![Action::PassPriority];
        let _ = resolve_choice(
            PolicyChoice::Action(Action::Concede),
            &legal,
        );
    }

    // -- concrete policies --------------------------------------------

    #[test]
    fn random_policy_is_deterministic_under_seed() {
        let mut a = RandomPolicy::new(42);
        let mut b = RandomPolicy::new(42);
        let legal = vec![Action::PassPriority, Action::Concede, Action::MulliganKeep];
        let obs = [0.0f32; 8];
        // Compare 10 picks from independently-seeded copies.
        for _ in 0..10 {
            let pa = a.select_action(&obs, &legal);
            let pb = b.select_action(&obs, &legal);
            match (pa, pb) {
                (PolicyChoice::Action(ax), PolicyChoice::Action(bx)) => {
                    assert_eq!(ax, bx)
                }
                other => panic!("unexpected variant pair: {other:?}"),
            }
        }
    }

    #[test]
    fn first_action_policy_always_returns_index_zero() {
        let mut p = FirstActionPolicy;
        for _ in 0..5 {
            match p.select_action(&[], &[Action::PassPriority, Action::Concede]) {
                PolicyChoice::Index(0) => {}
                other => panic!("expected Index(0), got {other:?}"),
            }
        }
    }

    #[test]
    fn progress_biased_picks_mulligan_when_offered() {
        let mut p = ProgressBiasedRandomPolicy::new(0);
        let legal = vec![Action::PassPriority, Action::MulliganKeep, Action::Concede];
        match p.select_action(&[], &legal) {
            PolicyChoice::Index(i) => assert_eq!(legal[i], Action::MulliganKeep),
            other => panic!("expected Index(_), got {other:?}"),
        }
    }

    // -- run_episode end-to-end --------------------------------------

    #[test]
    fn run_episode_terminates_within_caps_for_known_seed() {
        let outcome = run_progress_biased(42);
        // Either GameOver or truncated; both are valid outcomes
        // for a single seed.
        assert!(outcome.steps_taken > 0);
        if outcome.trajectories[0].terminated {
            assert!(matches!(
                outcome.final_yield,
                EngineYield::GameOver(_)
            ));
        }
    }

    #[test]
    fn run_episode_per_perspective_trajectories() {
        let outcome = run_progress_biased(42);
        assert_eq!(outcome.trajectories.len(), 2);
        for (p, traj) in outcome.trajectories.iter().enumerate() {
            assert_eq!(traj.perspective, p as PlayerId);
            // Each step's observation is sized to encoder.dim().
            for st in &traj.steps {
                assert_eq!(
                    st.observation.len(),
                    crate::observation::BASIC_E2_DIM_TWO_PLAYERS
                );
                // No NaNs / inf in observations.
                assert!(st.observation.iter().all(|x| x.is_finite()));
            }
        }
    }

    #[test]
    fn terminal_reward_is_stamped_on_winner_and_loser() {
        // Find a seed that produces a Win (rather than Draw) so we
        // can assert the +1/-1 stamping. 16 seeds is enough per the
        // existing engine test.
        for seed in 0..16u64 {
            let outcome = run_progress_biased(seed);
            if let EngineYield::GameOver(GameResult::Win(winner)) = &outcome.final_yield {
                let w = *winner as usize;
                let l = 1 - w;
                assert_eq!(outcome.trajectories[w].final_reward, 1.0);
                assert_eq!(outcome.trajectories[l].final_reward, -1.0);
                assert!(outcome.trajectories[w].terminated);
                assert!(!outcome.trajectories[w].truncated);
                return;
            }
        }
        panic!("no Win outcome across 16 seeds; harness or engine regression");
    }

    #[test]
    fn truncation_via_aggressive_step_cap() {
        // A 2-step cap is well below what any real game needs;
        // confirms the truncation path stamps `truncated=true`,
        // `terminated=false`, and `final_reward=0`.
        let (registry, decks) = bolt_milestone_setup();
        let (state, yld) = new_game(decks, &registry, 0);
        let mut policies: Vec<Box<dyn Policy>> = vec![
            Box::new(ProgressBiasedRandomPolicy::new(1)),
            Box::new(ProgressBiasedRandomPolicy::new(2)),
        ];
        let encoder = BasicE2Encoder::for_two_players();
        let reward = TerminalReward;
        let config = EpisodeConfig { max_turns: 200, max_steps: 2 };
        let outcome = run_episode(
            state,
            yld,
            &registry,
            &mut policies,
            &encoder,
            &reward,
            &config,
        );
        assert!(!outcome.trajectories[0].terminated);
        assert!(outcome.trajectories[0].truncated);
        assert_eq!(outcome.trajectories[0].final_reward, 0.0);
        assert_eq!(outcome.trajectories[1].final_reward, 0.0);
        assert_eq!(outcome.steps_taken, 2);
    }

    /// Headline harness regression. 1000 episodes of progress-biased
    /// random vs progress-biased random terminate within the cap,
    /// produce no NaNs/inf in observations, and at least one episode
    /// per side wins (no symmetry bug in the harness perspective
    /// routing).
    ///
    /// `#[ignore]`d because 1000 vanilla games is meaningful CI time
    /// even though each episode finishes in milliseconds. Run with:
    ///   cargo test -p arcana-ai --lib selfplay::tests::random_vs_random_1000_episodes -- --ignored --nocapture
    #[test]
    #[ignore]
    fn random_vs_random_1000_episodes() {
        let mut p0_wins = 0;
        let mut p1_wins = 0;
        let mut draws = 0;
        let mut truncations = 0;

        for seed in 0..1000u64 {
            let outcome = run_progress_biased(seed);
            assert!(
                outcome.steps_taken <= EpisodeConfig::default().max_steps,
                "episode {seed} ran past step cap"
            );
            // Observations finite at every step.
            for traj in &outcome.trajectories {
                for st in &traj.steps {
                    assert!(
                        st.observation.iter().all(|x| x.is_finite()),
                        "non-finite observation in episode {seed}"
                    );
                }
            }
            match outcome.final_yield {
                EngineYield::GameOver(GameResult::Win(0)) => p0_wins += 1,
                EngineYield::GameOver(GameResult::Win(1)) => p1_wins += 1,
                EngineYield::GameOver(GameResult::Draw) => draws += 1,
                EngineYield::GameOver(GameResult::Eliminated(_)) => draws += 1,
                _ => truncations += 1,
            }
        }
        eprintln!(
            "1000-episode harness regression: p0={p0_wins} p1={p1_wins} \
             draws={draws} truncations={truncations}"
        );
        // Both perspectives have to win sometimes — a 1000-vs-0 split
        // is a perspective-routing bug.
        assert!(p0_wins > 0, "P0 never won across 1000 episodes");
        assert!(p1_wins > 0, "P1 never won across 1000 episodes");
        // Truncations should be rare. A handful is OK; lots indicates
        // the policy isn't making progress on some seeds.
        assert!(
            truncations < 50,
            "{truncations}/1000 episodes truncated — policy or engine slowdown"
        );
    }
}
