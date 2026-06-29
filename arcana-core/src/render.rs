//! Human-readable [`GameState`] rendering — a compact text view of the board,
//! for self-play debugging, the CLI replay tool, and (later) human play /
//! multiplayer UIs. Read-only; reads card names from the registry's interner
//! and P/T from the layer-computed characteristics.

use std::fmt::Write as _;

use crate::actions::{Action, ChoiceAction, ChoiceResponse};
use crate::objects::ObjectId;
use crate::registry::CardRegistry;
use crate::state::GameState;
use crate::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use crate::types::PlayerId;
use crate::zones::Zone;

fn card_name(state: &GameState, registry: &CardRegistry, id: crate::objects::ObjectId) -> String {
    let Some(obj) = state.objects.get(id) else { return "?".into() };
    registry.interner().resolve(obj.characteristics.name)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "<hidden>".into())
}

/// A human-readable counter label ("+1/+1", "-1/-1", "loyalty", "verse", …).
fn counter_label(kind: &crate::types::CounterKind, registry: &CardRegistry) -> String {
    use crate::types::CounterKind::*;
    match kind {
        PlusOnePlusOne => "+1/+1".into(),
        MinusOneMinusOne => "-1/-1".into(),
        Named(sym) => registry.interner().resolve(*sym).unwrap_or("").to_string(),
        other => format!("{other:?}").to_lowercase(),
    }
}

/// A one-line, player-perspective description of a notable resolved game event,
/// for the play log ("You gain 3 life", "Bolt deals 3 to Grizzly Bears",
/// "Grizzly Bears dies"). `None` for structural/noisy events (phases, taps,
/// spell-cast — casts are already logged as actions). `perspective` decides
/// "You" vs "Opponent".
pub fn describe_event(
    event: &crate::events::GameEvent,
    state: &GameState,
    registry: &CardRegistry,
    perspective: PlayerId,
) -> Option<String> {
    use crate::events::{DamageTarget, GameEvent::*};
    let who = |p: PlayerId| if p == perspective { "You" } else { "Opponent" };
    // subject + correctly-conjugated verb ("You gain" / "Opponent gains")
    let act = |p: PlayerId, base: &str| {
        if p == perspective { format!("You {base}") } else { format!("Opponent {base}s") }
    };
    // LTB events (discard / mill / dies) carry the PRE-move id, which is re-ided
    // on the zone change (CR 400.7) — so fall back to a generic noun rather than
    // the "?" / "<hidden>" placeholders.
    let name = |id: ObjectId| {
        let n = card_name(state, registry, id);
        if n == "?" || n == "<hidden>" { "a card".to_string() } else { n }
    };
    Some(match event {
        LifeGained { player, amount } => format!("{} {amount} life", act(*player, "gain")),
        LifeLost { player, amount } => format!("{} {amount} life", act(*player, "lose")),
        DamageDealt { source, target, amount, .. } => {
            let tgt = match target {
                DamageTarget::Object(id) => name(*id),
                DamageTarget::Player(p) => who(*p).to_string(),
            };
            format!("{} deals {amount} to {tgt}", name(*source))
        }
        DrawCard { player, .. } => act(*player, "draw") + " a card",
        Discarded { player, object_id } => format!("{} {}", act(*player, "discard"), name(*object_id)),
        Milled { player, object_id } => format!("{} {}", act(*player, "mill"), name(*object_id)),
        Dies { object_id } => format!("{} dies", name(*object_id)),
        Sacrifice { player, object_id } => format!("{} {}", act(*player, "sacrifice"), name(*object_id)),
        Exiled { object_id, .. } => format!("{} is exiled", name(*object_id)),
        CounterAdded { object_id, kind, count } => format!(
            "{} gets {count} {} counter{}",
            name(*object_id), counter_label(kind, registry), if *count == 1 { "" } else { "s" }),
        TokenCreated { object_id, controller } => format!("{} {}", act(*controller, "create"), name(*object_id)),
        DieRolled { player, sides, result } => format!("{} a d{sides} → {result}", act(*player, "roll")),
        _ => return None,
    })
}

/// The printed text of `source`'s activated ability `index`, trimmed to a
/// readable length (so an action label is self-describing rather than
/// "[ability N]"). `None` if the source/def/ability isn't found.
fn ability_text(state: &GameState, registry: &CardRegistry,
                source: crate::objects::ObjectId, index: usize) -> Option<String> {
    let obj = state.objects.get(source)?;
    let def = registry.get(obj.card_id)?;
    let text = def.activated_abilities.get(index)?.text.trim();
    if text.is_empty() { return None; }
    const MAX: usize = 64;
    Some(if text.chars().count() > MAX {
        format!("{}…", text.chars().take(MAX - 1).collect::<String>().trim_end())
    } else {
        text.to_string()
    })
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

/// Like [`render`] but also lists `perspective`'s HAND contents (card names) —
/// what a human player needs to see to choose. Plain [`render`] shows only hand
/// counts (right for a spectator/log, not for the player to move). Hidden info
/// is respected upstream: callers pass a perspective-projected state, so other
/// players' hidden cards are already anonymized.
pub fn render_for(state: &GameState, registry: &CardRegistry, perspective: PlayerId) -> String {
    let mut out = render(state, registry);
    let hand: Vec<String> = state.objects.objects_in_zone(Zone::Hand(perspective))
        .map(|o| o.id)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|id| card_name(state, registry, id))
        .collect();
    let _ = writeln!(out, "  P{perspective} hand: {}",
        if hand.is_empty() { "(empty)".to_string() } else { hand.join(", ") });
    out
}

/// A brief label for one object: "Name P/T" (P/T only for creatures). For
/// interactive combat menus (listing attackers/blockers). Public wrapper over
/// the internal permanent renderer minus the tapped marker.
pub fn render_object_brief(state: &GameState, registry: &CardRegistry,
                           id: crate::objects::ObjectId) -> String {
    let mut s = card_name(state, registry, id);
    if let Some(obj) = state.objects.get(id) {
        if obj.characteristics.types.is_creature() {
            let p = state.computed_power(id).unwrap_or(0);
            let t = state.computed_toughness(id).unwrap_or(0);
            let _ = write!(s, " {p}/{t}");
        }
    }
    s
}

fn render_target(t: &TargetChoice, state: &GameState, registry: &CardRegistry) -> String {
    match t {
        TargetChoice::Object(id) | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) =>
            card_name(state, registry, *id),
        TargetChoice::Player(p) | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) =>
            format!("P{p}"),
    }
}

/// " → t1, t2" for a non-empty target list, else "".
fn render_targets(sel: &TargetSelection, state: &GameState, registry: &CardRegistry) -> String {
    if sel.targets.is_empty() { return String::new(); }
    let names: Vec<String> = sel.targets.iter().map(|t| render_target(t, state, registry)).collect();
    format!(" → {}", names.join(", "))
}

fn render_distribution(d: &[(ObjectId, u32)], state: &GameState, registry: &CardRegistry) -> String {
    d.iter()
        .map(|(id, n)| format!("{n}×{}", card_name(state, registry, *id)))
        .collect::<Vec<_>>().join(", ")
}

/// A resolution choice ([`Action::MakeChoice`]) — common variants get friendly
/// text; rarer ones fall back to debug.
fn render_choice_action(c: &ChoiceAction, state: &GameState, registry: &CardRegistry) -> String {
    match c {
        ChoiceAction::ChooseObject(id) => format!("Choose {}", card_name(state, registry, *id)),
        ChoiceAction::ChoosePlayer(p) => format!("Choose P{p}"),
        ChoiceAction::ChooseColor(col) => format!("Choose {col:?}"),
        ChoiceAction::ChooseManaColor(col) => format!("Choose {col:?}"),
        ChoiceAction::ChooseNumber(n) => format!("Choose {n}"),
        ChoiceAction::ChooseYesNo(b) => if *b { "Yes".into() } else { "No".into() },
        ChoiceAction::Distribute(d) => format!("Distribute {}", render_distribution(d, state, registry)),
        other => format!("{other:?}"),
    }
}

/// A reply to a pending resolution choice ([`Action::SubmitResolutionChoice`]).
fn render_choice_response(r: &ChoiceResponse, state: &GameState, registry: &CardRegistry) -> String {
    match r {
        ChoiceResponse::YesNo { answer } => if *answer { "Yes".into() } else { "No".into() },
        ChoiceResponse::PayOrDecline { pay } | ChoiceResponse::OptionalCost { pay } =>
            if *pay { "Pay".into() } else { "Decline".into() },
        ChoiceResponse::PickCards { picked } if picked.is_empty() => "Pick nothing".into(),
        ChoiceResponse::PickCards { picked } => format!("Pick {}",
            picked.iter().map(|id| card_name(state, registry, *id)).collect::<Vec<_>>().join(", ")),
        ChoiceResponse::DistributeDamage { distribution }
        | ChoiceResponse::DistributeCounters { distribution } =>
            format!("Distribute {}", render_distribution(distribution, state, registry)),
        ChoiceResponse::ChooseTargets { selection } if selection.targets.is_empty() =>
            "Choose no targets".to_string(),
        ChoiceResponse::ChooseTargets { selection } => {
            let names: Vec<String> = selection.targets.iter()
                .map(|t| render_target(t, state, registry)).collect();
            format!("Target {}", names.join(", "))
        }
        ChoiceResponse::PickPlayer { picked } => format!("Choose P{picked}"),
        ChoiceResponse::ChooseColor { color } => format!("Choose {color:?}"),
        ChoiceResponse::OrderCards { placements } =>
            format!("Order {} card(s)", placements.len()),
    }
}

/// A short, human-readable label for `action` — the menu text in human play.
/// Resolves object ids to card names; common actions get friendly phrasing, the
/// rest fall back to a compact debug form (improved incrementally). Shared by
/// the CLI and any future GUI frontend.
pub fn render_action(action: &Action, state: &GameState, registry: &CardRegistry) -> String {
    match action {
        Action::PassPriority => "Pass".to_string(),
        Action::Concede => "Concede".to_string(),
        Action::MulliganKeep => "Keep this hand".to_string(),
        Action::MulliganAgain => "Mulligan (draw a new hand)".to_string(),
        Action::PlayLand { object_id, mdfc_back } => {
            let name = card_name(state, registry, *object_id);
            if *mdfc_back { format!("Play {name} (back face)") } else { format!("Play {name}") }
        }
        Action::CastSpell { object_id, targets, x_value, .. } => {
            let mut s = format!("Cast {}", card_name(state, registry, *object_id));
            if let Some(x) = x_value { let _ = write!(s, " (X={x})"); }
            s.push_str(&render_targets(targets, state, registry));
            s
        }
        Action::ActivateAbility { source, ability_index, targets, .. } => {
            // Prefer the ability's own text ("{2}: Regenerate this creature.")
            // over the opaque "[ability N]" so the player knows what they're
            // activating / choosing a target for.
            let name = card_name(state, registry, *source);
            let mut s = match ability_text(state, registry, *source, *ability_index) {
                Some(t) => format!("{name} — {t}"),
                None => format!("Activate {name} [ability {ability_index}]"),
            };
            s.push_str(&render_targets(targets, state, registry));
            s
        }
        Action::MakeChoice(c) => render_choice_action(c, state, registry),
        Action::SubmitResolutionChoice { response, .. } =>
            render_choice_response(response, state, registry),
        Action::DeclareAttackers { attackers } if attackers.is_empty() =>
            "Attack with nothing".to_string(),
        Action::DeclareAttackers { attackers } => {
            let names: Vec<String> = attackers.iter()
                .map(|a| card_name(state, registry, a.attacker)).collect();
            format!("Attack with {}", names.join(", "))
        }
        Action::DeclareBlockers { blockers } if blockers.is_empty() =>
            "Block with nothing".to_string(),
        Action::DeclareBlockers { blockers } => {
            let pairs: Vec<String> = blockers.iter()
                .map(|b| format!("{} blocks {}",
                    card_name(state, registry, b.blocker),
                    card_name(state, registry, b.blocking)))
                .collect();
            format!("Block: {}", pairs.join("; "))
        }
        Action::BottomCards(ids) => format!("Bottom {} card(s)", ids.len()),
        // OrderBlockers / AssignCombatDamage / MakeChoice / SubmitResolutionChoice
        // get a debug fallback for now; friendlier phrasing is a polish pass.
        other => format!("{other:?}"),
    }
}

/// One-line summary (turn + each player's life) — for terse per-step logs.
pub fn render_oneline(state: &GameState) -> String {
    let lives: Vec<String> = (0..state.num_players())
        .map(|p| format!("P{p} {}", state.player(p).life))
        .collect();
    format!("T{} {:?}/{:?} [{}]",
        state.turn.turn_number, state.turn.phase, state.turn.step, lives.join(" "))
}

// Most render tests live in arcana-ai (catalog access; see record.rs note on
// the dev-dep type-unification gotcha). `describe_event` needs no catalog, so it
// is tested here directly.
#[cfg(test)]
mod tests {
    use super::describe_event;
    use crate::events::GameEvent;
    use crate::registry::CardRegistry;
    use crate::state::GameState;

    #[test]
    fn describe_event_renders_outcomes_from_perspective() {
        let s = GameState::new(2, 0);
        let reg = CardRegistry::new();
        let d = |e: &GameEvent, p| describe_event(e, &s, &reg, p);
        assert_eq!(d(&GameEvent::LifeGained { player: 0, amount: 3 }, 0).as_deref(),
            Some("You gain 3 life"));
        assert_eq!(d(&GameEvent::LifeLost { player: 1, amount: 2 }, 0).as_deref(),
            Some("Opponent loses 2 life"));
        assert_eq!(d(&GameEvent::DrawCard { player: 0, object_id: 1 }, 0).as_deref(),
            Some("You draw a card"));
        // Perspective flips for the other seat.
        assert_eq!(d(&GameEvent::LifeGained { player: 0, amount: 3 }, 1).as_deref(),
            Some("Opponent gains 3 life"));
        // Structural / noisy events aren't logged.
        assert_eq!(d(&GameEvent::Tapped { object_id: 1 }, 0), None);
    }
}
