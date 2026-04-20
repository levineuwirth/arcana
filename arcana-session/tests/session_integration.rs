//! Session-layer integration tests.
//!
//! These exercise `GameSession` end-to-end against a real seed
//! registry. They are the v0.2 deliverable gate: a `GameSession`
//! wired with random-policy agents must drive a full MTG game to
//! completion, emit observer events, and produce the same final
//! state that a direct `arcana_core::step` loop would have.

use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use arcana_cards::register_seed;
use arcana_core::{Action, CardRegistry, FormatConfig};
use arcana_core::registry::build_deck;
use arcana_core::state::GameResult;
use arcana_session::{
    GameSession, LogObserver, PlayerAgent,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// The v0.2 "session-layer foundations complete" gate, per spec §40:
/// build a session with two random agents and run it to completion.
#[test]
fn session_random_vs_random_terminates() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);
    let registry = Arc::new(registry);

    let deck = build_deck(&[
        ("Mountain", 12),
        ("Forest", 12),
        ("Lightning Bolt", 12),
        ("Grizzly Bears", 12),
    ], &registry);

    // Capture log output into a Vec so the test can assert events
    // were emitted without spamming stdout on a passing run.
    let log_buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let observer = Box::new(LogObserver::new(SharedBuf(log_buf.clone())));

    let mut session = GameSession::builder()
        .registry(registry.clone())
        .format(FormatConfig::standard_2026())
        .deck(0, deck.clone())
        .deck(1, deck.clone())
        .agent(0, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(42) })
        .agent(1, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(43) })
        .observer(observer)
        .history_depth(4)
        .seed(7)
        .skip_validation()
        .build()
        .expect("valid session config");

    let result = session.run();

    // Exactly one of Win/Draw — Eliminated is multiplayer.
    assert!(
        matches!(result, GameResult::Win(_) | GameResult::Draw),
        "unexpected result {result:?}",
    );
    assert!(session.events_logged() > 0, "no events emitted");
    assert!(
        !log_buf.lock().unwrap().is_empty(),
        "observer received no events",
    );
}

/// `GameSession` must not reshape what `arcana_core::step` does —
/// wrapping the engine should be pure pass-through. Two independent
/// setups with identical seeds — one driven through the session, one
/// driven through raw `step()` — must pick the same actions in the
/// same order and reach the same final state.
///
/// This depends on `arcana-core`'s deterministic hasher (see
/// `tests/determinism.rs` in that crate). With the default
/// `RandomState`-backed `HashMap`, two independently-built games hash
/// object ids into different iteration orders and would legitimately
/// diverge under random agents — this test would fail and the
/// diagnosis would be core, not session.
#[test]
fn session_matches_direct_step_loop() {
    use arcana_core::engine::{new_game_with_format, step, EngineYield};

    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);
    let registry = Arc::new(registry);

    let deck = build_deck(&[
        ("Mountain", 12),
        ("Forest", 12),
        ("Grizzly Bears", 12),
        ("Lightning Bolt", 24),
    ], &registry);

    const SEED: u64 = 99;
    const RNG_A: u64 = 1;
    const RNG_B: u64 = 2;

    // --- Path A: session-driven. Record each action via a mirror
    //             agent pair seeded identically to the ones inside
    //             the session; they'll pick the same thing given the
    //             same (state, legal_actions). ---------------------
    let mut session = GameSession::builder()
        .registry(registry.clone())
        .format(FormatConfig::standard_2026())
        .deck(0, deck.clone())
        .deck(1, deck.clone())
        .agent(0, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(RNG_A) })
        .agent(1, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(RNG_B) })
        .seed(SEED)
        .skip_validation()
        .build()
        .unwrap();

    let mut mirror_agents = [
        PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(RNG_A) },
        PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(RNG_B) },
    ];
    let mut session_actions = Vec::new();
    loop {
        let pending = session.pending().clone();
        match pending {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { player, legal_actions, context } => {
                let has_meaningful = legal_actions.iter()
                    .any(|a| !a.is_pass() && !a.is_concede());
                let action = if !has_meaningful {
                    Action::PassPriority
                } else {
                    mirror_agents[player as usize]
                        .request_decision(session.state(), &legal_actions, &context)
                };
                session_actions.push(action.clone());
                session.apply(action);
            }
        }
    }
    let session_final = session.state().clone();

    // --- Path B: direct step loop with matching agents. -----------
    let mut direct_agents = [
        PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(RNG_A) },
        PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(RNG_B) },
    ];
    let (mut state, mut yld) = new_game_with_format(
        vec![deck.clone(), deck.clone()],
        FormatConfig::standard_2026(), &registry, SEED,
    );
    let mut direct_actions = Vec::new();
    loop {
        match yld {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { player, legal_actions, context } => {
                let has_meaningful = legal_actions.iter()
                    .any(|a| !a.is_pass() && !a.is_concede());
                let action = if !has_meaningful {
                    Action::PassPriority
                } else {
                    direct_agents[player as usize]
                        .request_decision(&state, &legal_actions, &context)
                };
                direct_actions.push(action.clone());
                let (ns, ny) = step(state, action, &registry);
                state = ns;
                yld = ny;
            }
        }
    }

    for (i, (a, b)) in session_actions.iter().zip(&direct_actions).enumerate() {
        assert_eq!(a, b,
            "action {i} diverges: session={a:?}  direct={b:?}");
    }
    assert_eq!(session_actions.len(), direct_actions.len(),
        "action-sequence lengths diverge: session={} direct={}",
        session_actions.len(), direct_actions.len());
    assert_eq!(session_final.result, state.result,
        "final result diverges");
    assert_eq!(session_final.event_log.len(), state.event_log.len(),
        "event-log lengths diverge");
}

/// Undo restores the previous state and makes it usable for the next
/// action.
#[test]
fn undo_rolls_back_most_recent_action() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);
    let registry = Arc::new(registry);

    let deck = build_deck(&[
        ("Mountain", 30),
        ("Lightning Bolt", 30),
    ], &registry);

    let mut session = GameSession::builder()
        .registry(registry)
        .deck(0, deck.clone())
        .deck(1, deck.clone())
        .agent(0, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(11) })
        .agent(1, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(22) })
        .history_depth(8)
        .skip_validation()
        .build()
        .unwrap();

    // Resolve mulligan prompts by applying one MulliganKeep apiece.
    for _ in 0..2 {
        session.apply(Action::MulliganKeep);
    }

    let snapshot_depth = session.history_depth();
    let snapshot_events = session.events_logged();
    assert!(snapshot_depth >= 2,
        "expected >=2 history entries, got {snapshot_depth}");

    // Apply a pass, then undo it — events_logged should not rewind
    // (it counts cumulative observer activity, not state), but
    // history_depth should drop by one.
    session.apply(Action::PassPriority);
    let before_undo_events = session.events_logged();
    assert!(before_undo_events >= snapshot_events);

    session.undo().expect("undo with populated history");
    assert_eq!(session.history_depth(), snapshot_depth);
}

/// Default-on deck validation rejects a Standard-illegal deck at
/// session-build time. The fence exists so a malformed deck from an
/// eventual untrusted source (CLI user, arcana-gen output) fails
/// loudly at construction rather than as a silent mid-game
/// empty-library loss.
#[test]
fn build_rejects_deck_failing_validation() {
    use arcana_core::format::DeckValidationError;
    use arcana_session::SessionBuildError;

    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);
    let registry = Arc::new(registry);

    // 30-card all-Mountains deck: fails on both size (min 60) and
    // copy limit (4-of) for Standard.
    let deck = build_deck(&[("Mountain", 30)], &registry);

    let result = GameSession::builder()
        .registry(registry)
        .format(FormatConfig::standard_2026())
        .deck(0, deck.clone())
        .deck(1, deck)
        .agent(0, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(1) })
        .agent(1, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(2) })
        .build();

    let err = result.err().expect("build must reject non-conforming deck");
    match err {
        SessionBuildError::InvalidDeck { player, errors } => {
            assert_eq!(player, 0,
                "should fail on first non-conforming deck");
            assert!(errors.iter().any(|e| matches!(e,
                DeckValidationError::TooFewCards { minimum: 60, actual: 30 })),
                "expected TooFewCards in {errors:?}");
            assert!(errors.iter().any(|e| matches!(e,
                DeckValidationError::TooManyCopies { .. })),
                "expected TooManyCopies in {errors:?}");
        }
        other => panic!("expected InvalidDeck, got {other:?}"),
    }
}

/// A standard-legal 60-card 4-of deck passes validation and builds
/// a session without `.skip_validation()`. Counterpart to the
/// rejection test: confirms the fence is not over-restrictive.
#[test]
fn build_accepts_standard_legal_deck() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);
    let registry = Arc::new(registry);

    // 15 distinct cards × 4 copies = 60. Standard-legal.
    let deck = build_deck(&[
        ("Plains", 4), ("Island", 4), ("Swamp", 4),
        ("Mountain", 4), ("Forest", 4),
        ("Grizzly Bears", 4), ("Lightning Bolt", 4),
        ("Counterspell", 4), ("Murder", 4), ("Elvish Visionary", 4),
        ("Glorious Anthem", 4), ("Disintegrate", 4),
        ("Serra Angel", 4), ("Giant Spider", 4), ("Typhoid Rats", 4),
    ], &registry);
    assert_eq!(deck.len(), 60, "test precondition: 60-card deck");

    let session = GameSession::builder()
        .registry(registry)
        .format(FormatConfig::standard_2026())
        .deck(0, deck.clone())
        .deck(1, deck)
        .agent(0, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(3) })
        .agent(1, PlayerAgent::Random { rng: ChaCha8Rng::seed_from_u64(4) })
        .build();

    assert!(session.is_ok(), "valid deck must build: {:?}", session.err());
}

// --- test harness ------------------------------------------------------------

struct SharedBuf(Arc<Mutex<Vec<u8>>>);

impl Write for SharedBuf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
