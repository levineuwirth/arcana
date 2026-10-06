//! Cross-invocation determinism guard.
//!
//! These tests exist specifically to fail if `arcana-core` ever
//! regresses to a non-deterministic hasher (`std::collections::HashMap`
//! with `RandomState`). They run the same game twice from independent
//! starting states with identically-seeded random drivers and assert
//! that every decision point — and thus every final state — matches.
//!
//! The underlying invariant from spec principle P5: *given the same
//! initial state and action sequence, the engine always produces the
//! same result*. A random agent closes the loop from the other side:
//! if the hasher is deterministic, the agent's `rng.gen_range(0..n)`
//! over legal actions will pick the same index, because `legal_actions`
//! returns them in the same order on every run.
//!
//! If this file starts failing, do **not** weaken the assertions. The
//! right diagnosis is that a `HashMap` / `HashSet` somewhere on the
//! decision path is iterating in a non-deterministic order — either a
//! new `std::collections::HashMap` slipped in, or a collection's
//! ordering isn't respecting the hasher (e.g., a `Vec::extend` over a
//! `HashMap::iter()`).

use std::hash::Hasher;
use std::process::Command;

use arcana_cards::register_seed;
use arcana_core::engine::{new_game_with_format, step, EngineYield};
use arcana_core::events::GameEvent;
use arcana_core::objects::GameObject;
use arcana_core::registry::{build_deck, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ManaColor};
use arcana_core::zones::Zone;
use arcana_core::{Action, FormatConfig, ObjectId};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Run one full game, recording every action picked. Deterministic
/// given the seeds.
fn run_game(
    registry: &CardRegistry,
    deck: Vec<arcana_core::types::CardId>,
    format: FormatConfig,
    game_seed: u64,
    rng_seeds: [u64; 2],
) -> (Vec<Action>, arcana_core::state::GameState) {
    let mut rngs = [
        ChaCha8Rng::seed_from_u64(rng_seeds[0]),
        ChaCha8Rng::seed_from_u64(rng_seeds[1]),
    ];
    let (mut state, mut yld) = new_game_with_format(
        vec![deck.clone(), deck], format, registry, game_seed,
    );

    let mut actions = Vec::new();
    loop {
        match yld {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                let action = pick(&mut rngs[player as usize], &legal_actions);
                actions.push(action.clone());
                let (ns, ny) = step(state, action, registry);
                state = ns;
                yld = ny;
            }
        }
    }
    (actions, state)
}

fn pick(rng: &mut ChaCha8Rng, actions: &[Action]) -> Action {
    if actions.iter().any(|a| matches!(a, Action::MulliganKeep)) {
        return Action::MulliganKeep;
    }
    let interesting: Vec<&Action> = actions.iter()
        .filter(|a| !a.is_pass() && !a.is_concede()).collect();
    if !interesting.is_empty() {
        let idx = rng.gen_range(0..interesting.len());
        return interesting[idx].clone();
    }
    actions.iter()
        .find(|a| a.is_pass())
        .cloned()
        .unwrap_or_else(|| actions[0].clone())
}

/// Two independent runs with identical seeds must produce identical
/// action sequences and final states. This would have failed with
/// `std::collections::HashMap::new()` because each map's `RandomState`
/// differs per-invocation → `legal_actions` iteration order differs
/// → random agent picks different indices.
#[test]
fn same_seed_same_actions_across_independent_games() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);

    let deck = build_deck(&[
        ("Mountain", 12),
        ("Forest", 12),
        ("Grizzly Bears", 12),
        ("Lightning Bolt", 24),
    ], &registry);

    let (actions_a, final_a) = run_game(
        &registry, deck.clone(),
        FormatConfig::standard_2026(), 42, [7, 13],
    );
    let (actions_b, final_b) = run_game(
        &registry, deck.clone(),
        FormatConfig::standard_2026(), 42, [7, 13],
    );

    // Point failure: diff the first divergent action rather than
    // just comparing lengths.
    for (i, (a, b)) in actions_a.iter().zip(actions_b.iter()).enumerate() {
        assert_eq!(a, b,
            "action {i} diverges under identical seeds:\n  \
             A = {a:?}\n  B = {b:?}\n\
             Likely cause: non-deterministic HashMap hasher somewhere \
             on the legal-action / SBA / trigger path.");
    }
    assert_eq!(actions_a.len(), actions_b.len(),
        "action sequence length diverges: {} vs {}",
        actions_a.len(), actions_b.len());
    assert_eq!(final_a.result, final_b.result,
        "final result diverges");
    assert_eq!(final_a.event_log.len(), final_b.event_log.len(),
        "event-log length diverges: {} vs {}",
        final_a.event_log.len(), final_b.event_log.len());
}

/// Replay parity: re-executing the recorded action sequence against
/// a fresh starting state must reproduce the same trajectory. This
/// catches hidden state in `CardRegistry` or thread-local caches
/// that `step` might implicitly depend on.
#[test]
fn recorded_actions_replay_to_identical_state() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);

    let deck = build_deck(&[
        ("Mountain", 12),
        ("Forest", 12),
        ("Lightning Bolt", 36),
    ], &registry);

    let (actions, original_final) = run_game(
        &registry, deck.clone(),
        FormatConfig::standard_2026(), 99, [1, 2],
    );

    // Replay those exact actions against a fresh game state.
    let (mut state, mut yld) = new_game_with_format(
        vec![deck.clone(), deck],
        FormatConfig::standard_2026(), &registry, 99,
    );
    for (i, a) in actions.into_iter().enumerate() {
        assert!(matches!(yld, EngineYield::PendingDecision { .. }),
            "replay hit a non-pending yield at action {i}");
        let (ns, ny) = step(state, a, &registry);
        state = ns;
        yld = ny;
    }
    assert!(matches!(yld, EngineYield::GameOver(_)),
        "replay did not terminate at the same point");
    assert_eq!(original_final.result, state.result,
        "replay diverges on final result");
    assert_eq!(original_final.event_log.len(), state.event_log.len(),
        "replay diverges on event-log length");
}

// ---------------------------------------------------------------------
// Cross-process replay of last-known-information triggers
// ---------------------------------------------------------------------

/// Set in a re-executed copy of this test binary to make it a child:
/// it plays the scenario, prints its digest and returns.
const LKI_CHILD_ENV: &str = "ARCANA_LKI_REPLAY_CHILD";
const LKI_DIGEST_PREFIX: &str = "lki-replay-digest=";
const LKI_CHILDREN: usize = 4;
const LKI_SEED: u64 = 0x4c4b_4900;

/// Five creatures of one controller, each with a "whenever a creature
/// you control dies" trigger and an effect that leaves a different
/// trace in the event log, die to one Wrath of God. Each death event
/// then fires all five from last-known information, so they reach
/// the stack in `GameState::lki`'s iteration order, and the event
/// log records the order they resolve in. A self-"dies" trigger
/// would not do: each matches only its own event, so its order is
/// the event order whatever the map's.
fn play_lki_scenario() -> (Vec<Action>, GameState) {
    let mut registry = CardRegistry::new();
    let watchers: Vec<CardId> = vec![
        arcana_cards::clu::vindictive_vampire::register(&mut registry),
        arcana_cards::soc::zulaport_cutthroat::register(&mut registry),
        arcana_cards::fdn::midnight_reaper::register(&mut registry),
        arcana_cards::lcc::pitiless_plunderer::register(&mut registry),
        arcana_cards::dmu::elas_il_kor_sadistic_pilgrim::register(&mut registry),
    ];
    let wrath = arcana_cards::cmm::wrath_of_god::register(&mut registry);

    let mut s = GameState::new(2, LKI_SEED);
    for &card in &watchers {
        add_object(&mut s, &registry, card, Zone::Battlefield);
    }
    // Midnight Reaper draws once per death.
    for _ in 0..10 {
        let id = add_object(&mut s, &registry, wrath, Zone::Library(0));
        s.player_mut(0).library_top_to_bottom.push(id);
    }
    let wrath_in_hand = add_object(&mut s, &registry, wrath, Zone::Hand(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::White, 4, 0);
    s.priority.give_to(0);
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;

    let cast = arcana_core::legal_actions::legal_actions(&s, &registry)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, .. }
            if *object_id == wrath_in_hand))
        .expect("Wrath of God is castable from four white mana");
    let mut actions = vec![cast.clone()];
    let (mut s, mut yld) = step(s, cast, &registry);
    while !s.stack_is_empty() {
        let EngineYield::PendingDecision { legal_actions, .. } = &yld else {
            panic!("game ended with the stack non-empty");
        };
        let pass = legal_actions.iter().find(|a| a.is_pass()).cloned()
            .unwrap_or_else(|| panic!("no pass among {legal_actions:?}"));
        actions.push(pass.clone());
        (s, yld) = step(s, pass, &registry);
    }
    (actions, s)
}

fn add_object(
    s: &mut GameState,
    registry: &CardRegistry,
    card: CardId,
    zone: Zone,
) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = registry.get(card).unwrap().base_characteristics.clone();
    s.objects.insert(GameObject::new(id, 0, zone, card, chars));
    id
}

/// FxHash of the serialized action list and event log: the same
/// bytes give the same digest in any process.
fn lki_digest(actions: &[Action], s: &GameState) -> u64 {
    let mut h = rustc_hash::FxHasher::default();
    h.write(&serde_json::to_vec(actions).unwrap());
    h.write(&serde_json::to_vec(&s.event_log).unwrap());
    h.finish()
}

/// A networked match resumes by replaying its transcript in a fresh
/// process, so trigger order must not depend on anything seeded per
/// process. `recorded_actions_replay_to_identical_state` is the
/// same-process half of this; here the scenario is played in this
/// process and again in several re-executed copies of the test
/// binary, and every digest must agree. With `GameState::lki` on
/// `std::collections::HashMap`, each process orders the five LKI
/// sources by its own random seed.
#[test]
fn lki_trigger_order_replays_identically_across_processes() {
    let (actions, s) = play_lki_scenario();
    let digest = lki_digest(&actions, &s);
    if std::env::var_os(LKI_CHILD_ENV).is_some() {
        println!("{LKI_DIGEST_PREFIX}{digest:016x}");
        return;
    }

    // The scenario must still be the one described above, or the
    // digests would agree vacuously.
    let deaths = s.event_log.iter()
        .filter(|e| matches!(e, GameEvent::Dies { .. })).count();
    assert_eq!(deaths, 5, "Wrath of God should kill all five watchers");
    let resolved = s.event_log.iter()
        .filter(|e| matches!(e, GameEvent::AbilityResolved { .. })).count();
    assert_eq!(resolved, 25,
        "each of the five deaths should trigger all five watchers from LKI");
    // A resolution that finds no effect leaves no trace of its order.
    let draws = s.event_log.iter()
        .filter(|e| matches!(e, GameEvent::DrawCard { .. })).count();
    assert_eq!(draws, 5, "Midnight Reaper should draw once for each death");

    let exe = std::env::current_exe().unwrap();
    for child in 0..LKI_CHILDREN {
        let out = Command::new(&exe)
            .args(["lki_trigger_order_replays_identically_across_processes",
                   "--exact", "--nocapture", "--test-threads=1"])
            .env(LKI_CHILD_ENV, "1")
            .output()
            .expect("re-execute the test binary");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "child {child} failed:\n{stdout}\n{}",
            String::from_utf8_lossy(&out.stderr));
        // libtest prints "test <name> ... " before the child's own line.
        let theirs: Vec<&str> = stdout.lines()
            .filter_map(|l| l.split_once(LKI_DIGEST_PREFIX))
            .map(|(_, d)| d.trim()).collect();
        assert_eq!(theirs.len(), 1, "child {child} printed no single digest:\n{stdout}");
        assert_eq!(theirs[0], format!("{digest:016x}"),
            "child {child} replayed the LKI scenario differently; something \
             on the trigger path iterates in a per-process order");
    }
}
