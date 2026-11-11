//! Fixed registered-card workload for stage 3. Run unchanged before/after.
//! Setup and cloning are excluded from the timed regions. No policy or RNG
//! choices change with machine speed; each row uses exactly 100 iterations.
use std::{hint::black_box, time::{Duration, Instant}};
use arcana_core::{Action, CardRegistry, GameState};
use arcana_core::effects::Effect;
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::CounterKind;
use arcana_core::zones::Zone;

fn add(s: &mut GameState, r: &CardRegistry, name: &str) -> ObjectId {
    let card = r.card_id_by_name(name).expect("registered benchmark card");
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, 0, Zone::Hand(0), card,
        r.get(card).unwrap().initial_characteristics().clone()));
    s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution).unwrap()
}

fn settle(mut s: GameState, r: &CardRegistry) -> GameState {
    // First pass collects the events produced by the fixture's zone moves.
    s = step(s, Action::PassPriority, r).0;
    for _ in 0..200 {
        if s.stack_is_empty() && s.pending_choice.is_none()
            && s.pending_trigger_queue.is_empty() { return s; }
        let actions = legal_actions(&s, r);
        let action = actions.iter().find(|a| a.is_pass())
            .or_else(|| actions.iter().find(|a| !a.is_concede()))
            .expect("setup decision").clone();
        s = step(s, action, r).0;
    }
    panic!("benchmark setup did not settle");
}

fn fixture(r: &CardRegistry, size: usize) -> GameState {
    let mut s = GameState::new(2, 20261110);
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    for name in ["Supreme Phantom", "Empyrean Eagle", "Domri, Anarch of Bolas",
        "Goblin Warchief", "Rishkar, Peema Renegade"] {
        let id = add(&mut s, r, name);
        if name == "Domri, Anarch of Bolas" {
            s.objects.get_mut(id).unwrap().add_counters(CounterKind::Loyalty, 3);
        }
    }
    let voice = add(&mut s, r, "Voice of Resurgence");
    for i in 6..size {
        add(&mut s, r, ["Supreme Phantom", "Goblin Piker", "Grizzly Bears"][i % 3]);
    }
    s = settle(s, r);
    Effect::SacrificePermanent { target: voice }.execute(&mut s);
    s = settle(s, r);
    let ids: Vec<_> = s.objects.objects_in_zone(Zone::Battlefield).map(|o| o.id).collect();
    assert_eq!(ids.len(), size, "fixture board size");
    assert!(s.objects.objects_in_zone(Zone::Battlefield).any(|o| o.is_token),
        "Voice must create its registered token");
    for id in ids {
        s.objects.get_mut(id).unwrap().add_counters(CounterKind::PlusOnePlusOne, 1);
    }
    s.priority.give_to(0);
    s
}

fn main() {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    println!("workload=stage3-statics-v1 seed=20261110 iterations=100 policy=none");
    println!("permanents,ns_per_characteristic,ns_per_step");
    for size in [10, 50, 200] {
        let s = fixture(&r, size);
        let ids: Vec<_> = s.objects.objects_in_zone(Zone::Battlefield).map(|o| o.id).collect();
        let mut compute = Duration::ZERO;
        let mut stepping = Duration::ZERO;
        // Fixed warmup count, independent of the measured throughput.
        for _ in 0..10 {
            for &id in &ids { black_box(s.compute_characteristics(id)); }
            black_box(step(s.clone(), Action::PassPriority, &r));
        }
        for _ in 0..100 {
            let start = Instant::now();
            for &id in &ids { black_box(s.compute_characteristics(id)); }
            compute += start.elapsed();
            let copy = s.clone();
            let start = Instant::now();
            black_box(step(copy, Action::PassPriority, &r));
            stepping += start.elapsed();
        }
        println!("{size},{:.1},{:.1}", compute.as_nanos() as f64 / (100 * size) as f64,
            stepping.as_nanos() as f64 / 100.0);
    }
}
