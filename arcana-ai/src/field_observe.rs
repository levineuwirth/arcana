//! Exploratory field observation for the stage-3 decision (2026-11-09). Not
//! part of any registered experiment; nothing here changes how a game plays.
//!
//! Two ignored tests, run as cluster shards the way the A2.2 gauntlet is
//! (`arcana-ai/scripts/dtu/observe.lsf`):
//!
//! - [`tests::occurrence_shard`] plays seat-swapped `vmc-material` duels on
//!   the pinned field and counts, in the outer game only (never a policy's
//!   rollouts), how often the situations the stage-3 items depend on arise.
//!   The counts are opportunities under this engine and this policy, not
//!   proof that a missing behavior would have been used.
//! - [`tests::fuzz_shard`] plays uniformly random legal actions (never
//!   Concede) and reports every ending kind separately: rule endings, the
//!   action cap, an empty action list, a decision offering only Concede, an
//!   invariant failure, or a panic. Every failure row carries what replay
//!   needs: base seed, pair, game and step.
//!
//! Activations are counted from the chosen `ActivateAbility` action, so a
//! mana ability used inside a cast's payment (Gilded Goose's Food mana) is
//! not seen. Card names are written as indexes into `TRACKED`, with one
//! `card,index,name` row per tracked card, because names contain commas.
//!
//! Both read the decks as the gauntlet does (`KAGGLE_DECKS`, `CORPUS_FORMAT`,
//! `CORPUS_MANIFEST`) and shard pairs by index modulo `K` (`CORPUS_SHARD=k/K`).
//! `OBS_PAIRS=pilot` plays only the disjoint pairs (0, 1), (2, 3), ….

use crate::deckeval_runner::gauntlet_pairs;
use crate::search::StatePolicy;
use arcana_core::actions::Action;
use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::events::GameEvent;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::state::GameResult;
use arcana_core::types::{CardId, PlayerId};
use arcana_core::zones::Zone;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::{BTreeMap, HashMap};

/// The 37 field cards that still carry a high-rated census finding after
/// stage 1, A1.5bw and both stage-2 sweeps (the review's interim count).
pub const TRACKED: [&str; 37] = [
    "Adanto Vanguard", "Bomat Courier", "Chandra, Torch of Defiance", "Charming Prince",
    "Courser of Kruphix", "Curious Obsession", "Despise", "Domri, Anarch of Bolas",
    "Elder Deep-Fiend", "Empyrean Eagle", "Emry, Lurker of the Loch", "Game Trail",
    "Gilded Goose", "Glorybringer", "Goblin Ringleader", "Goblin Warchief",
    "Hazoret the Fervent", "Jace, Vryn's Prodigy", "Kalitas, Traitor of Ghet",
    "Kari Zev, Skyship Raider", "Krenko, Tin Street Kingpin", "Mausoleum Wanderer",
    "Once Upon a Time", "Port Town", "Questing Beast", "Reflector Mage",
    "Rhonas the Indomitable", "Rishkar, Peema Renegade", "Supreme Phantom",
    "Tasigur, the Golden Fang", "The Scarab God", "Torbran, Thane of Red Fell",
    "Verdurous Gearhulk", "Voice of Resurgence", "Voracious Hydra", "Wildgrowth Walker",
    "Xenagos, God of Revels",
];

/// How an outer game ended.
#[derive(Clone, Debug, PartialEq)]
pub enum Ending {
    Rule(GameResult),
    StepCap,
    EmptyActions,
    Panic(String),
}

impl Ending {
    fn label(&self) -> String {
        match self {
            Ending::Rule(GameResult::Win(p)) => format!("win{p}"),
            Ending::Rule(GameResult::Eliminated(p)) => format!("eliminated{p}"),
            Ending::Rule(GameResult::Draw) => "draw".into(),
            Ending::StepCap => "step_cap".into(),
            Ending::EmptyActions => "empty_actions".into(),
            Ending::Panic(_) => "panic".into(),
        }
    }
}

/// Opportunity counts for one outer game, keyed by (seat, card, kind).
pub struct Observer {
    names: HashMap<CardId, &'static str>,
    pub counts: BTreeMap<(PlayerId, &'static str, &'static str), u32>,
    cast_any: [bool; 2],
    ouat_before_first: [bool; 2],
    turn: u32,
    snapshot: HashMap<arcana_core::ObjectId, (&'static str, PlayerId)>,
    land_types: [Option<arcana_core::types::SmallString>; 4],
}

impl Observer {
    pub fn new(registry: &CardRegistry) -> Self {
        let names = TRACKED
            .iter()
            .filter_map(|&n| registry.card_id_by_name(n).map(|id| (id, n)))
            .collect();
        let it = registry.interner();
        Self {
            names,
            counts: BTreeMap::new(),
            cast_any: [false; 2],
            ouat_before_first: [false; 2],
            turn: 0,
            snapshot: HashMap::new(),
            land_types: [it.lookup("Plains"), it.lookup("Island"), it.lookup("Mountain"), it.lookup("Forest")],
        }
    }

    fn bump(&mut self, seat: PlayerId, card: &'static str, kind: &'static str, n: u32) {
        if n > 0 && seat < 2 {
            *self.counts.entry((seat, card, kind)).or_default() += n;
        }
    }

    fn tracked(&self, state: &GameState, id: arcana_core::ObjectId) -> Option<(&'static str, PlayerId)> {
        if let Some(&hit) = self.snapshot.get(&id) {
            return Some(hit);
        }
        let o = state.object_or_lki(id)?;
        Some((*self.names.get(&o.card_id)?, o.controller))
    }

    /// Before an outer decision is applied: snapshot the tracked objects
    /// (a cost may sacrifice a source before its events are read), count an
    /// activation the decider chose, and record first-spell opportunities.
    pub fn before(&mut self, state: &GameState, decider: PlayerId, action: Option<&Action>) {
        self.snapshot = state.objects.iter()
            .filter_map(|o| self.names.get(&o.card_id).map(|&n| (o.id, (n, o.controller))))
            .collect();
        if let Some(Action::ActivateAbility { source, ability_index, .. }) = action {
            if let Some(&(card, seat)) = self.snapshot.get(source) {
                let kind = match ability_index {
                    0 => "activates_0",
                    1 => "activates_1",
                    2 => "activates_2",
                    3 => "activates_3",
                    _ => "activates_other",
                };
                self.bump(seat, card, kind, 1);
            }
        }
        let d = decider as usize;
        if d < 2 && !self.cast_any[d] && !self.ouat_before_first[d] {
            let held = state.objects.objects_in_zone(Zone::Hand(decider))
                .any(|o| self.names.get(&o.card_id) == Some(&"Once Upon a Time"));
            if held {
                self.ouat_before_first[d] = true;
                self.bump(decider, "Once Upon a Time", "in_hand_before_first_spell", 1);
            }
        }
    }

    /// After an outer step, with the events that step emitted.
    pub fn after(&mut self, state: &GameState, events: &[GameEvent]) {
        for e in events {
            match e {
                GameEvent::EntersBattlefield { object_id, .. } => {
                    if let Some((card, seat)) = self.tracked(state, *object_id) {
                        self.bump(seat, card, "enters", 1);
                        let wants: &[usize] = match card {
                            "Port Town" => &[0, 1],
                            "Game Trail" => &[2, 3],
                            _ => &[],
                        };
                        if !wants.is_empty() {
                            let types: Vec<_> = wants.iter().filter_map(|&k| self.land_types[k]).collect();
                            let matched = state.objects.objects_in_zone(Zone::Hand(seat)).any(|o| {
                                o.characteristics.subtypes.0.iter().any(|t| types.contains(t))
                            });
                            self.bump(seat, card, "enters_with_matching_land_in_hand", matched as u32);
                        }
                    }
                }
                GameEvent::SpellCast { card_id, controller, .. } => {
                    let c = *controller as usize;
                    if let Some(&card) = self.names.get(card_id) {
                        self.bump(*controller, card, "cast", 1);
                        if card == "Once Upon a Time" && c < 2 && !self.cast_any[c] {
                            self.bump(*controller, card, "cast_as_first_spell", 1);
                        }
                    }
                    if c < 2 {
                        self.cast_any[c] = true;
                    }
                }
                GameEvent::CreatureAttacks { attacker, .. } => {
                    if let Some((card, seat)) = self.tracked(state, *attacker) {
                        self.bump(seat, card, "attacks", 1);
                    }
                }
                GameEvent::CreatureBlocks { blocker, attacker } => {
                    if let Some((card, seat)) = self.tracked(state, *blocker) {
                        self.bump(seat, card, "blocks", 1);
                    }
                    if let Some((card, seat)) = self.tracked(state, *attacker) {
                        self.bump(seat, card, "blocked", 1);
                    }
                }
                GameEvent::Dies { object_id } => {
                    if let Some((card, seat)) = self.tracked(state, *object_id) {
                        self.bump(seat, card, "dies", 1);
                    }
                    // Kalitas: an opponent's nontoken creature dies while Kalitas is out.
                    if let Some(dead) = state.object_or_lki(*object_id) {
                        if !dead.is_token {
                            let owners: Vec<PlayerId> = state.objects.objects_in_zone(Zone::Battlefield)
                                .filter(|o| self.names.get(&o.card_id) == Some(&"Kalitas, Traitor of Ghet"))
                                .map(|o| o.controller)
                                .filter(|&c| c != dead.controller)
                                .collect();
                            for c in owners {
                                self.bump(c, "Kalitas, Traitor of Ghet", "opponent_nontoken_dies", 1);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if state.turn.turn_number != self.turn {
            self.turn = state.turn.turn_number;
            let on_field: Vec<(&'static str, PlayerId)> = state.objects.objects_in_zone(Zone::Battlefield)
                .filter_map(|o| self.names.get(&o.card_id).map(|&n| (n, o.controller)))
                .collect();
            for (card, seat) in on_field {
                self.bump(seat, card, "turns_on_battlefield", 1);
                if card == "Rishkar, Peema Renegade" {
                    let eligible = state.objects.objects_in_zone(Zone::Battlefield)
                        .filter(|o| o.controller == seat && state.is_creature_now(o.id)
                            && o.counters.values().any(|&n| n > 0))
                        .count() as u32;
                    self.bump(seat, card, "counter_creatures_at_turn_start", eligible);
                    self.bump(seat, card, "turns_with_a_counter_creature", (eligible > 0) as u32);
                }
            }
        }
    }
}

/// One outer game under `mk`'s policies, observed. Mirrors
/// `deckeval_runner::play_seated` and `search::play_match`, with the same
/// step cap and empty-action rule, but reports which ending occurred.
#[allow(clippy::too_many_arguments)]
pub fn observe_seated(
    seat_decks: Vec<Vec<CardId>>,
    registry: &CardRegistry,
    engine_seed: u64,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    policy_seed_0: u64,
    policy_seed_1: u64,
    max_steps: u32,
    obs: &mut Observer,
) -> (Ending, u32) {
    let mut p0 = mk(policy_seed_0);
    let mut p1 = mk(policy_seed_1);
    let mut slots: Vec<&mut dyn StatePolicy> = vec![p0.as_mut(), p1.as_mut()];
    let (mut state, mut yld) = new_game(seat_decks, registry, engine_seed);
    let mut steps = 0u32;
    loop {
        match yld {
            EngineYield::GameOver(r) => return (Ending::Rule(r), steps),
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                if legal_actions.is_empty() {
                    return (Ending::EmptyActions, steps);
                }
                if steps >= max_steps {
                    return (Ending::StepCap, steps);
                }
                let action = slots[player as usize].choose(&state, registry, player, &legal_actions);
                obs.before(&state, player, Some(&action));
                let start = state.event_log.len();
                let before = state;
                let (s, y) = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| step(before, action, registry))) {
                    Ok(x) => x,
                    Err(e) => return (Ending::Panic(panic_text(&e)), steps),
                };
                state = s;
                yld = y;
                steps += 1;
                let end = state.event_log.len();
                let events = if start <= end { state.event_log[start..end].to_vec() } else { Vec::new() };
                obs.after(&state, &events);
            }
        }
    }
}

/// How a fuzz game ended. Everything but a rule ending is a finding.
#[derive(Clone, Debug)]
pub struct FuzzOutcome {
    pub ending: String,
    pub steps: u32,
    pub detail: String,
}

fn invariant_problem(state: &GameState) -> Option<String> {
    let n = state.num_players() as PlayerId;
    if let Some(o) = state.objects.iter().find(|o| o.controller >= n || o.owner >= n) {
        return Some(format!("object {} has controller {} owner {}", o.id, o.controller, o.owner));
    }
    None
}

fn one_line(s: &str, cap: usize) -> String {
    s.replace([',', '\n', '\r'], " ").chars().take(cap).collect()
}

/// One random-play game: uniformly random legal actions other than Concede,
/// keeping every opening hand. Panics inside the engine are caught and
/// reported with the step they happened at.
pub fn fuzz_seated(
    seat_decks: Vec<Vec<CardId>>,
    registry: &CardRegistry,
    engine_seed: u64,
    policy_seed: u64,
    max_actions: u32,
) -> FuzzOutcome {
    let mut rng = ChaCha8Rng::seed_from_u64(policy_seed);
    let started = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| new_game(seat_decks, registry, engine_seed)));
    let (mut state, mut yld) = match started {
        Ok(x) => x,
        Err(e) => return FuzzOutcome { ending: "panic".into(), steps: 0, detail: format!("new_game: {}", panic_text(&e)) },
    };
    let mut steps = 0u32;
    loop {
        match &yld {
            EngineYield::GameOver(r) => {
                return FuzzOutcome { ending: Ending::Rule(r.clone()).label(), steps, detail: String::new() };
            }
            EngineYield::PendingDecision { legal_actions, .. } => {
                if legal_actions.is_empty() {
                    return FuzzOutcome { ending: "empty_actions".into(), steps, detail: pending_text(&state) };
                }
                let choices: Vec<&Action> = legal_actions.iter().filter(|a| !a.is_concede()).collect();
                if choices.is_empty() {
                    return FuzzOutcome { ending: "only_concede".into(), steps, detail: pending_text(&state) };
                }
                if steps >= max_actions {
                    return FuzzOutcome { ending: "action_cap".into(), steps, detail: String::new() };
                }
                let action = if let Some(keep) = choices.iter().find(|a| matches!(a, Action::MulliganKeep)) {
                    (*keep).clone()
                } else {
                    choices[rng.gen_range(0..choices.len())].clone()
                };
                let shown = format!("{action:?}");
                let before = state.clone();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| step(before, action, registry))) {
                    Ok((s, y)) => {
                        state = s;
                        yld = y;
                        steps += 1;
                    }
                    Err(e) => {
                        return FuzzOutcome {
                            ending: "panic".into(),
                            steps,
                            detail: format!("{} | action {}", panic_text(&e), one_line(&shown, 160)),
                        };
                    }
                }
                if let Some(problem) = invariant_problem(&state) {
                    return FuzzOutcome {
                        ending: "invariant".into(),
                        steps,
                        detail: format!("{problem} | after {}", one_line(&shown, 160)),
                    };
                }
            }
        }
    }
}

fn panic_text(e: &Box<dyn std::any::Any + Send>) -> String {
    let s = e.downcast_ref::<String>().cloned()
        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "non-string panic".into());
    one_line(&s, 200)
}

fn pending_text(state: &GameState) -> String {
    match &state.pending_choice {
        Some(pc) => one_line(&format!("{:?} for player {}", pc.kind, pc.choosing_player), 160),
        None => "no pending choice".into(),
    }
}

/// The pairs a shard plays: every gauntlet pair, or the disjoint pilot pairs.
pub fn observed_pairs(n: usize, pilot: bool) -> Vec<(usize, usize)> {
    if pilot {
        (0..n / 2).map(|k| (2 * k, 2 * k + 1)).collect()
    } else {
        gauntlet_pairs(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use crate::deck_corpus::{decks_from_dir, playable_decks, require_manifest};
    use crate::deckeval::Deck;
    use crate::deckeval_runner::{maker_for, mix4, Referee, RefereeBudgets};

    fn shard_from_env() -> (usize, usize) {
        std::env::var("CORPUS_SHARD")
            .ok()
            .map(|s| {
                let (a, b) = s.split_once('/').expect("CORPUS_SHARD is k/K");
                (a.parse::<usize>().expect("k"), b.parse::<usize>().expect("K"))
            })
            .unwrap_or((0, 1))
    }

    fn field_decks(reg: &CardRegistry) -> Vec<Deck> {
        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS");
        let format = std::env::var("CORPUS_FORMAT").unwrap_or_else(|_| "PI".into());
        let dir = std::path::Path::new(&base).join(&format);
        let loaded = decks_from_dir(&dir, &format, reg).expect("read corpus dir");
        let decks = playable_decks(&loaded, 60, 60);
        require_manifest(&decks);
        decks
    }

    fn env_u64(k: &str, default: u64) -> u64 {
        std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
    }

    /// Occurrence counts on the field: one seat-swapped duel per pair under
    /// `vmc-material` at `RefereeBudgets::from_env()` (`CORPUS_POLICIES=capsule`
    /// gives A2.2's 6/25/10), engine seeds as `play_duel` derives them.
    /// Rows: `config`, `deck`, `game,i,j,ab,seat0_deck,seat1_deck,ending,steps,seconds`
    /// and `occ,i,j,ab,seat,deck,card,kind,count`.
    #[test]
    #[ignore = "cluster shard: set KAGGLE_DECKS and CORPUS_SHARD"]
    fn occurrence_shard() {
        let (k, shards) = shard_from_env();
        assert!(k < shards, "CORPUS_SHARD {k}/{shards}: k must be below K");
        let pilot = std::env::var("OBS_PAIRS").as_deref() == Ok("pilot");
        let base_seed = env_u64("CORPUS_BASE_SEED", 0);
        let max_steps = env_u64("OBS_MAX_STEPS", 4000) as u32;
        let budgets = RefereeBudgets::from_env();
        // libtest prints "test <name> ... " before the first row on a one-core
        // task; an empty line keeps that prefix off every row.
        println!();
        let reg = arcana_cards::build_catalog();
        let decks = field_decks(&reg);
        for (c, name) in TRACKED.iter().enumerate() {
            println!("card,{c},{name}");
        }
        for (i, d) in decks.iter().enumerate() {
            println!("deck,{i},{}", d.name);
        }
        println!("config,occurrence,{budgets:?},{base_seed},{max_steps},{},{k}/{shards}", if pilot { "pilot" } else { "all" });
        for (p, &(i, j)) in observed_pairs(decks.len(), pilot).iter().enumerate() {
            if p % shards != k {
                continue;
            }
            let engine_seed = mix4(base_seed, i as u64, j as u64, 0);
            let seed_i = engine_seed ^ 0xA5A5_A5A5_A5A5_A5A5;
            let seed_j = engine_seed ^ 0x5A5A_5A5A_5A5A_5A5A;
            for (ab, (s0, s1), (q0, q1)) in [("A", (i, j), (seed_i, seed_j)), ("B", (j, i), (seed_j, seed_i))] {
                let seats = vec![decks[s0].cards.clone(), decks[s1].cards.clone()];
                let mk = maker_for(Referee::VmcMaterial, &budgets, &seats);
                let mut obs = Observer::new(&reg);
                let t = Instant::now();
                let (ending, steps) = observe_seated(seats, &reg, engine_seed, &*mk, q0, q1, max_steps, &mut obs);
                println!("game,{i},{j},{ab},{s0},{s1},{},{steps},{:.2}", ending.label(), t.elapsed().as_secs_f64());
                if let Ending::Panic(why) = &ending {
                    println!("fail,{base_seed},{i},{j},{ab},panic,{steps},{why}");
                }
                for ((seat, card, kind), n) in &obs.counts {
                    let deck = if *seat == 0 { s0 } else { s1 };
                    let c = TRACKED.iter().position(|t| t == card).expect("a tracked card");
                    println!("occ,{i},{j},{ab},{seat},{deck},{c},{kind},{n}");
                }
            }
        }
    }

    /// Random-play robustness: both seatings of every pair (or the pilot's)
    /// for `FUZZ_SEEDS` consecutive base seeds from `CORPUS_BASE_SEED`.
    /// Rows: `config`, `deck`, `fuzz,seed,i,j,ab,ending,steps,seconds` and,
    /// for every non-rule ending, `fail,seed,i,j,ab,ending,steps,detail`.
    #[test]
    #[ignore = "cluster shard: set KAGGLE_DECKS and CORPUS_SHARD"]
    fn fuzz_shard() {
        let (k, shards) = shard_from_env();
        assert!(k < shards, "CORPUS_SHARD {k}/{shards}: k must be below K");
        let pilot = std::env::var("OBS_PAIRS").as_deref() == Ok("pilot");
        let base_seed = env_u64("CORPUS_BASE_SEED", 0);
        let seeds = env_u64("FUZZ_SEEDS", 1);
        let max_actions = env_u64("FUZZ_MAX_ACTIONS", 20_000) as u32;
        println!();
        let reg = arcana_cards::build_catalog();
        let decks = field_decks(&reg);
        for (i, d) in decks.iter().enumerate() {
            println!("deck,{i},{}", d.name);
        }
        println!("config,fuzz,{base_seed},{seeds},{max_actions},{},{k}/{shards}", if pilot { "pilot" } else { "all" });
        for seed in base_seed..base_seed + seeds {
            for (p, &(i, j)) in observed_pairs(decks.len(), pilot).iter().enumerate() {
                if p % shards != k {
                    continue;
                }
                let engine_seed = mix4(seed, i as u64, j as u64, 0);
                for (ab, (s0, s1), policy) in [("A", (i, j), engine_seed ^ 0xF00D), ("B", (j, i), engine_seed ^ 0xBEEF)] {
                    let seats = vec![decks[s0].cards.clone(), decks[s1].cards.clone()];
                    let t = Instant::now();
                    let out = fuzz_seated(seats, &reg, engine_seed, policy, max_actions);
                    println!("fuzz,{seed},{i},{j},{ab},{},{},{:.3}", out.ending, out.steps, t.elapsed().as_secs_f64());
                    let rule = out.ending.starts_with("win") || out.ending.starts_with("eliminated") || out.ending == "draw";
                    if !rule {
                        println!("fail,{seed},{i},{j},{ab},{},{},{}", out.ending, out.steps, out.detail);
                    }
                }
            }
        }
    }

    /// The observer counts what a hand-built game shows it: a Voracious
    /// Hydra entering, a Port Town entering with and without a matching land
    /// in hand, and the first-spell opportunity for Once Upon a Time.
    #[test]
    fn observer_counts_hand_built_situations() {
        use arcana_core::objects::GameObject;
        let reg = arcana_cards::build_catalog();
        let id = |n: &str| reg.card_id_by_name(n).unwrap();
        let mut s = GameState::new(2, 0);
        let add = |s: &mut GameState, owner: PlayerId, c: CardId, zone: Zone| {
            let oid = s.allocate_object_id();
            s.objects.insert(GameObject::new(oid, owner, zone, c, reg.get(c).unwrap().initial_characteristics().clone()));
            oid
        };
        let hydra = add(&mut s, 0, id("Voracious Hydra"), Zone::Battlefield);
        let town = add(&mut s, 1, id("Port Town"), Zone::Battlefield);
        add(&mut s, 1, id("Island"), Zone::Hand(1));
        add(&mut s, 0, id("Once Upon a Time"), Zone::Hand(0));
        let mut obs = Observer::new(&reg);
        obs.before(&s, 0, None);
        obs.before(&s, 0, None);
        obs.after(&s, &[
            GameEvent::EntersBattlefield { object_id: hydra, from_zone: Zone::Stack, was_cast: true },
            GameEvent::EntersBattlefield { object_id: town, from_zone: Zone::Hand(1), was_cast: false },
        ]);
        let get = |seat: PlayerId, card: &'static str, kind: &'static str| obs.counts.get(&(seat, card, kind)).copied().unwrap_or(0);
        assert_eq!(get(0, "Voracious Hydra", "enters"), 1);
        assert_eq!(get(1, "Port Town", "enters"), 1);
        assert_eq!(get(1, "Port Town", "enters_with_matching_land_in_hand"), 1);
        assert_eq!(get(0, "Once Upon a Time", "in_hand_before_first_spell"), 1, "counted once per game");
        assert_eq!(obs.counts.keys().filter(|(_, c, _)| *c == "Voracious Hydra").count(), 2, "enters plus the turn count");

        // An activation is counted from the action, before a sacrifice cost
        // can move its source away.
        let wanderer = add(&mut s, 1, id("Mausoleum Wanderer"), Zone::Battlefield);
        let sac = Action::ActivateAbility {
            source: wanderer,
            ability_index: 0,
            targets: arcana_core::targets::TargetSelection::new(),
            mana_payment: Default::default(),
            additional_costs: Default::default(),
        };
        obs.before(&s, 1, Some(&sac));
        s.objects.remove(wanderer);
        obs.after(&s, &[GameEvent::Dies { object_id: wanderer }]);
        assert_eq!(obs.counts.get(&(1, "Mausoleum Wanderer", "activates_0")).copied(), Some(1));
        assert_eq!(obs.counts.get(&(1, "Mausoleum Wanderer", "dies")).copied(), Some(1),
            "the pre-step snapshot names a source that has left");
    }
}
