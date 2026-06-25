//! Human-readable [`GameState`] rendering — a compact text view of the board,
//! for self-play debugging, the CLI replay tool, and (later) human play /
//! multiplayer UIs. Read-only; reads card names from the registry's interner
//! and P/T from the layer-computed characteristics.

use std::fmt::Write as _;

use crate::registry::CardRegistry;
use crate::state::GameState;
use crate::zones::Zone;

fn card_name(state: &GameState, registry: &CardRegistry, id: crate::objects::ObjectId) -> String {
    let Some(obj) = state.objects.get(id) else { return "?".into() };
    registry.interner().resolve(obj.characteristics.name)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "<hidden>".into())
}

/// One battlefield permanent: "Name P/T (tapped)" — P/T only for creatures.
fn render_permanent(state: &GameState, registry: &CardRegistry,
                    id: crate::objects::ObjectId) -> String {
    let mut s = card_name(state, registry, id);
    if let Some(obj) = state.objects.get(id) {
        if obj.characteristics.types.is_creature() {
            let p = state.computed_power(id).unwrap_or(0);
            let t = state.computed_toughness(id).unwrap_or(0);
            let _ = write!(s, " {p}/{t}");
        }
        if obj.is_tapped() { s.push_str(" (T)"); }
    }
    s
}

/// A compact, multi-line snapshot of `state` from the table's perspective.
pub fn render(state: &GameState, registry: &CardRegistry) -> String {
    let mut out = String::new();
    let active = state.active_player();
    let prio = state.priority_player();
    let _ = writeln!(out,
        "── Turn {} — {:?}/{:?} — active P{active}, priority P{prio} ──",
        state.turn.turn_number, state.turn.phase, state.turn.step);

    for p in 0..state.num_players() {
        let ps = state.player(p);
        let hand = state.objects.objects_in_zone(Zone::Hand(p)).count();
        let lib = state.objects.objects_in_zone(Zone::Library(p)).count();
        let gy = state.objects.objects_in_zone(Zone::Graveyard(p)).count();
        let marker = if p == active { "*" } else { " " };
        let poison = if ps.poison_counters > 0 {
            format!(" poison {}", ps.poison_counters) } else { String::new() };
        let _ = writeln!(out,
            "{marker}P{p}: {} life | hand {hand} | lib {lib} | gy {gy}{poison}",
            ps.life);
    }

    // Battlefield grouped by controller.
    for p in 0..state.num_players() {
        let perms: Vec<String> = state.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.controller == p)
            .map(|o| o.id)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|id| render_permanent(state, registry, id))
            .collect();
        if perms.is_empty() {
            let _ = writeln!(out, "  P{p} board: (empty)");
        } else {
            let _ = writeln!(out, "  P{p} board: {}", perms.join(", "));
        }
    }

    let stack = state.objects.objects_in_zone(Zone::Stack).count()
        + state.stack.len();
    if stack > 0 {
        let names: Vec<String> = state.stack.iter()
            .filter_map(|e| e.card_id().map(|_| e.id))
            .map(|id| card_name(state, registry, id))
            .collect();
        if names.is_empty() {
            let _ = writeln!(out, "  Stack: {stack} entr{}",
                if stack == 1 { "y" } else { "ies" });
        } else {
            let _ = writeln!(out, "  Stack: {}", names.join(", "));
        }
    }
    out
}

/// One-line summary (turn + each player's life) — for terse per-step logs.
pub fn render_oneline(state: &GameState) -> String {
    let lives: Vec<String> = (0..state.num_players())
        .map(|p| format!("P{p} {}", state.player(p).life))
        .collect();
    format!("T{} {:?}/{:?} [{}]",
        state.turn.turn_number, state.turn.phase, state.turn.step, lives.join(" "))
}

// Tests live in arcana-ai (catalog access; see record.rs note on the dev-dep
// type-unification gotcha).
