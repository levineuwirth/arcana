//! State-based actions per CR 704.5.
//!
//! Phase 1 Task #14. Depends on tasks 4 (objects), 5 (events),
//! 6 (state), 13 (effects), 16 (combat — for the shared
//! [`GameState::move_object_to_zone`] primitive).
//!
//! # Model (CR 704.3)
//!
//! State-based actions are checked whenever a player would receive
//! priority. All SBAs that apply are performed simultaneously as a
//! single event. The engine loops: check → apply → re-check, until
//! a pass finds nothing pending.
//!
//! [`apply_state_based_actions`] is the public entry point — it's
//! what the engine (Task #20) calls between any two priority windows
//! (including after every spell or ability resolves).
//!
//! # What's implemented
//!
//! | CR      | SBA                                 | Status |
//! |---------|-------------------------------------|--------|
//! | 704.5a  | Life ≤ 0 → player loses             | ✅ |
//! | 704.5b  | Drew from empty library → loses     | ✅ |
//! | 704.5c  | Poison ≥ 10 → player loses          | ✅ |
//! | 704.5f  | Toughness ≤ 0 → graveyard           | ✅ |
//! | 704.5g  | Lethal damage → destroyed           | ✅ |
//! | 704.5i  | Planeswalker loyalty = 0 → graveyard| ✅ |
//! | 704.5j  | Legend rule (name duplicate)        | ✅ (deterministic keep-oldest) |
//! | 704.5p  | ±1/±1 counter annihilation          | ✅ |
//! | 704.5d,e| Tokens/copies in wrong zone         | ✅ |
//! | 704.5q  | Equipment attached to illegal perm  | ✅ |
//! | 704.5r  | Fortification attached to illegal perm | ✅ |
//! | 704.5n  | Aura illegally attached or unattached | ✅ (zone-only; type-filter deferred) |
//! | 704.5s,t,u | Saga/battle/dungeon/ownership    | ⬜ (deferred) |
//!
//! The legend rule per CR 704.5j says "that player chooses one" —
//! for Phase 1 we **deterministically keep the legendary permanent
//! with the lowest `ObjectId`** (i.e. the oldest in the arena) and
//! sacrifice the rest. The engine (Task #20) will upgrade this to a
//! real agent decision.
//!
//! # Game-over side effect
//!
//! After each round of SBAs, if exactly one player remains alive the
//! game result is set to `Win(that_player)`; if zero remain, it's
//! `Draw`. The engine stops stepping once `GameState::is_game_over`.

use crate::events::{GameEvent, LoseReason, MoveCause};
use crate::objects::ObjectId;
use crate::state::{GameResult, GameState};
use crate::types::*;
use crate::zones::Zone;

// =============================================================================
// Public entry points
// =============================================================================

/// Check and apply every pending state-based action. Per CR 704.3,
/// SBAs happen as one simultaneous event — this function loops the
/// per-variant checks until a pass applies nothing, matching that
/// semantics.
///
/// After the SBA loop settles, this function also updates
/// [`GameState::result`] when the game has ended (sole survivor or
/// universal draw).
///
/// Returns the number of loop iterations that applied at least one
/// SBA (`0` means nothing changed).
pub fn apply_state_based_actions(state: &mut GameState) -> u32 {
    let mut iterations = 0;
    loop {
        let mut fired = false;
        // Order within a pass is canonical and deterministic; the
        // loop ensures simultaneity-of-outcome.
        fired |= annihilate_pt_counters(state);       // CR 704.5p
        fired |= check_creature_graveyard(state);     // CR 704.5f, 704.5g
        fired |= check_planeswalker_graveyard(state); // CR 704.5i
        fired |= apply_legend_rule(state);            // CR 704.5j
        // An SBA that yields an agent choice (Legend rule) pushes
        // `pending_choice` and returns. Bail out so the engine can
        // yield — the choice handler will re-enter SBAs on resume.
        if state.pending_choice.is_some() {
            iterations += 1;
            update_game_result(state);
            return iterations;
        }
        // CR 704.5d runs after every zone-movement SBA in this pass,
        // so any token sent to graveyard / exile / hand / library in
        // the steps above ceases to exist on the same iteration. LKI
        // was stored by the preceding zone move, so Dies / LeavesBF
        // triggers still see the pre-cease state.
        fired |= check_token_ceases_to_exist(state);  // CR 704.5d
        fired |= check_attachment_illegal(state);     // CR 704.5q
        fired |= check_fortification_illegal(state);  // CR 704.5r
        fired |= check_aura_illegal(state);           // CR 704.5n
        fired |= check_player_losses(state);          // CR 704.5a, 704.5b, 704.5c
        if !fired { break; }
        iterations += 1;
    }
    update_game_result(state);
    iterations
}

/// Non-mutating variant: does any SBA apply right now? Useful as a
/// cheap check ("is resolution pending interruption?") in the engine.
pub fn has_pending_state_based_actions(state: &GameState) -> bool {
    pending_player_loss(state) || pending_creature_to_graveyard(state)
        || pending_planeswalker_to_graveyard(state)
        || pending_legend_conflict(state)
        || pending_pt_annihilation(state)
        || pending_token_cease(state)
        || pending_attachment_illegal(state)
        || pending_fortification_illegal(state)
        || pending_aura_illegal(state)
}

// =============================================================================
// 704.5a / 704.5b / 704.5c — player losses
// =============================================================================

fn check_player_losses(state: &mut GameState) -> bool {
    let mut any = false;
    for p in 0..state.num_players() {
        let player = state.player(p);
        if player.has_lost { continue; }
        let reason = if player.life <= 0 {
            Some(LoseReason::LifeZero)
        } else if player.has_drawn_from_empty_library {
            Some(LoseReason::Decked)
        } else if player.poison_counters >= 10 {
            Some(LoseReason::PoisonCounters)
        } else {
            None
        };
        if let Some(reason) = reason {
            state.player_mut(p).has_lost = true;
            state.emit(GameEvent::PlayerLoses { player: p, reason });
            any = true;
        }
    }
    any
}

fn pending_player_loss(state: &GameState) -> bool {
    (0..state.num_players()).any(|p| {
        let pl = state.player(p);
        !pl.has_lost && (pl.life <= 0
            || pl.has_drawn_from_empty_library
            || pl.poison_counters >= 10)
    })
}

// =============================================================================
// 704.5f / 704.5g — creature to graveyard
// =============================================================================

/// A creature with toughness ≤ 0 (704.5f) or lethal damage marked
/// (704.5g) is put into its owner's graveyard. Also covers CR 702.2b —
/// any nonzero damage from a deathtouch source counts as lethal.
/// Indestructible creatures (CR 702.12b) are exempt from both the
/// lethal-damage and deathtouch branches (but *not* from 0-toughness —
/// that's what the spec calls out as 704.5f).
fn check_creature_graveyard(state: &mut GameState) -> bool {
    use crate::effects::KeywordAbility;
    let mut to_kill: Vec<ObjectId> = Vec::new();
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if !obj.is_creature() { continue; }
        let Some(t) = state.computed_toughness(obj.id) else { continue; };
        let indestructible = state.has_keyword(obj.id, &KeywordAbility::Indestructible);
        let dies_by_toughness = t <= 0;
        let dies_by_damage = !indestructible
            && t > 0
            && (obj.damage_marked as i32) >= t;
        let dies_by_deathtouch = !indestructible
            && t > 0
            && obj.damage_marked > 0
            && obj.has_deathtouch_damage;
        if dies_by_toughness || dies_by_damage || dies_by_deathtouch {
            to_kill.push(obj.id);
        }
    }
    if to_kill.is_empty() { return false; }
    to_kill.sort();
    for id in to_kill {
        let Some(owner) = state.objects.get(id).map(|o| o.owner) else { continue; };
        // CR 614 — run the die-replacement pipeline before commit.
        match state.replace_die(id) {
            crate::replacement::DieOutcome::ExileInstead => {
                state.move_object_to_zone(
                    id, Zone::Exile, MoveCause::StateBasedAction);
            }
            crate::replacement::DieOutcome::Regenerated => {
                // CR 701.25c — remove damage, tap, remove from combat.
                if let Some(obj) = state.objects.get_mut(id) {
                    obj.clear_damage();
                    obj.tap();
                }
                // Remove from active combat if present.
                if let Some(combat) = state.combat.as_mut() {
                    combat.attackers.retain(|a| a.object_id != id);
                    combat.blockers.retain(|b| b.object_id != id);
                    for atk in combat.attackers.iter_mut() {
                        atk.blocked_by.retain(|b| *b != id);
                    }
                }
            }
            crate::replacement::DieOutcome::StillDies => {
                state.move_object_to_zone(
                    id, Zone::Graveyard(owner), MoveCause::StateBasedAction);
            }
        }
    }
    true
}

fn pending_creature_to_graveyard(state: &GameState) -> bool {
    use crate::effects::KeywordAbility;
    state.objects.objects_in_zone(Zone::Battlefield).any(|obj| {
        if !obj.is_creature() { return false; }
        let indestructible = state.has_keyword(obj.id, &KeywordAbility::Indestructible);
        match state.computed_toughness(obj.id) {
            Some(t) if t <= 0 => true,
            Some(t) if !indestructible && (obj.damage_marked as i32) >= t => true,
            Some(t) if !indestructible && t > 0
                && obj.damage_marked > 0 && obj.has_deathtouch_damage => true,
            _ => false,
        }
    })
}

// =============================================================================
// 704.5i — planeswalker loyalty = 0
// =============================================================================

fn check_planeswalker_graveyard(state: &mut GameState) -> bool {
    let mut to_kill: Vec<ObjectId> = Vec::new();
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if !obj.is_planeswalker() { continue; }
        if planeswalker_loyalty(obj) <= 0 {
            to_kill.push(obj.id);
        }
    }
    if to_kill.is_empty() { return false; }
    to_kill.sort();
    for id in to_kill {
        let Some(owner) = state.objects.get(id).map(|o| o.owner) else { continue; };
        state.move_object_to_zone(
            id, Zone::Graveyard(owner), MoveCause::StateBasedAction);
    }
    true
}

fn pending_planeswalker_to_graveyard(state: &GameState) -> bool {
    state.objects.objects_in_zone(Zone::Battlefield).any(|obj|
        obj.is_planeswalker() && planeswalker_loyalty(obj) <= 0)
}

/// Current loyalty of a planeswalker object = Loyalty counter count.
/// Returns 0 for non-planeswalkers.
///
/// CR 113.3c — a PW enters the battlefield with Loyalty counters
/// equal to its printed loyalty; afterwards, loyalty = counter count.
/// `Characteristics.loyalty` is only the *printed* starting value,
/// used by [`crate::state::GameState::after_enter_battlefield`] to
/// seed the ETB counter placement. Once on the battlefield the
/// counter count is the sole source of truth; `+N:` / `−N:` costs
/// modify the counter count via place_counters / remove_counters and
/// the CR 704.5i check reads the counter count directly here.
fn planeswalker_loyalty(obj: &crate::objects::GameObject) -> i32 {
    obj.count_counters(CounterKind::Loyalty) as i32
}

// =============================================================================
// 704.5j — legend rule
// =============================================================================

/// CR 704.5j — if a player controls two or more legendary permanents
/// with the same name, "that player chooses one of them, and the rest
/// are put into their owners' graveyards."
///
/// Push a [`ChoiceKind::PickCards`] for the first conflicting group
/// found. Returns `true` to signal the SBA loop "something fired"; the
/// outer loop bails out on the pending choice so the engine can yield.
/// When the agent answers, [`apply_resolution_choice`] sacrifices the
/// non-chosen copies and the SBA loop re-runs to pick up any remaining
/// groups.
///
/// [`apply_resolution_choice`]: crate::engine
/// [`ChoiceKind::PickCards`]: crate::actions::ChoiceKind
fn apply_legend_rule(state: &mut GameState) -> bool {
    use std::collections::BTreeMap;

    // Canonical iteration: sort by (controller, name) so the group we
    // push is deterministic across runs.
    let mut groups: BTreeMap<(PlayerId, SmallString), Vec<ObjectId>> = BTreeMap::new();
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if !obj.characteristics.supertypes.is_legendary() { continue; }
        let key = (obj.controller, obj.characteristics.name);
        groups.entry(key).or_default().push(obj.id);
    }

    for ((controller, _name), mut ids) in groups {
        if ids.len() < 2 { continue; }
        ids.sort();
        // Push the choice: pick exactly one id to keep. The handler
        // sacrifices the rest.
        state.push_pending_choice(
            controller,
            crate::actions::ChoiceContext::Sba,
            crate::actions::ChoiceKind::PickCards {
                candidates: ids,
                min: 1,
                max: 1,
            },
        );
        return true;
    }
    false
}

fn pending_legend_conflict(state: &GameState) -> bool {
    use crate::collections::HashMap;
    let mut counts: HashMap<(PlayerId, SmallString), u32> = HashMap::default();
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if !obj.characteristics.supertypes.is_legendary() { continue; }
        *counts.entry((obj.controller, obj.characteristics.name)).or_insert(0) += 1;
    }
    counts.values().any(|&n| n >= 2)
}

// =============================================================================
// 704.5p — +1/+1 / -1/-1 annihilation
// =============================================================================

fn annihilate_pt_counters(state: &mut GameState) -> bool {
    let ids: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o|
            o.has_counter(CounterKind::PlusOnePlusOne)
            && o.has_counter(CounterKind::MinusOneMinusOne))
        .map(|o| o.id)
        .collect();

    if ids.is_empty() { return false; }
    let mut any = false;
    for id in ids {
        let pairs = state.objects.get_mut(id)
            .map(|o| o.annihilate_pt_counters())
            .unwrap_or(0);
        if pairs > 0 {
            state.emit(GameEvent::CounterRemoved {
                object_id: id,
                kind: CounterKind::PlusOnePlusOne,
                count: pairs,
            });
            state.emit(GameEvent::CounterRemoved {
                object_id: id,
                kind: CounterKind::MinusOneMinusOne,
                count: pairs,
            });
            any = true;
        }
    }
    any
}

fn pending_pt_annihilation(state: &GameState) -> bool {
    state.objects.objects_in_zone(Zone::Battlefield).any(|o|
        o.has_counter(CounterKind::PlusOnePlusOne)
        && o.has_counter(CounterKind::MinusOneMinusOne))
}

// =============================================================================
// 704.5d — tokens cease to exist in zones other than the battlefield
// =============================================================================

/// CR 704.5d — a token not on the battlefield ceases to exist. This
/// runs after every zone-movement SBA in the same loop pass so a
/// token that dies / gets exiled / is bounced is removed from the
/// arena before the next pass sees it. LKI was already captured by
/// the zone move (CR 603.10), so dies / leaves-the-battlefield
/// triggers read the pre-cease state normally.
fn check_token_ceases_to_exist(state: &mut GameState) -> bool {
    let to_remove: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.is_token && !o.zone.is_battlefield())
        .map(|o| o.id)
        .collect();
    if to_remove.is_empty() { return false; }
    for id in to_remove {
        state.objects.remove(id);
    }
    true
}

fn pending_token_cease(state: &GameState) -> bool {
    state.objects.iter().any(|o| o.is_token && !o.zone.is_battlefield())
}

// =============================================================================
// 704.5q — Equipment attached to illegal permanent
// =============================================================================

/// CR 704.5q — an Equipment attached to an illegal permanent (non-
/// creature or not on the battlefield) or to a nonpermanent becomes
/// unattached. It stays on the battlefield; this is a pure detach.
///
/// Equipment detection is heuristic in Phase 2: any artifact (not also
/// an enchantment) with `attached_to.is_some()` that isn't flagged as
/// a Fortification is treated as an Equipment-style attacher.
/// Fortifications have their own SBA ([`check_fortification_illegal`])
/// because their legal target is a land, not a creature.
fn check_attachment_illegal(state: &mut GameState) -> bool {
    let to_detach: Vec<(ObjectId, ObjectId)> = state.objects.iter()
        .filter(|o| is_equipment_style(o) && o.zone.is_battlefield())
        .filter_map(|o| o.attached_to.map(|t| (o.id, t)))
        .filter(|(_, target)| !is_legal_equipment_target(state, *target))
        .collect();
    if to_detach.is_empty() { return false; }
    for (equip_id, prior_target) in to_detach {
        if let Some(obj) = state.objects.get_mut(equip_id) {
            obj.detach();
        }
        if let Some(holder) = state.objects.get_mut(prior_target) {
            holder.attachments.retain(|&id| id != equip_id);
        }
        state.emit(GameEvent::Detached {
            equipment_or_aura: equip_id,
            from: prior_target,
        });
    }
    true
}

fn pending_attachment_illegal(state: &GameState) -> bool {
    state.objects.iter()
        .filter(|o| is_equipment_style(o) && o.zone.is_battlefield())
        .filter_map(|o| o.attached_to)
        .any(|target| !is_legal_equipment_target(state, target))
}

fn is_equipment_style(obj: &crate::objects::GameObject) -> bool {
    obj.characteristics.types.is_artifact()
        && !obj.characteristics.types.is_enchantment()
        && !obj.characteristics.is_fortification
        && obj.attached_to.is_some()
}

fn is_legal_equipment_target(state: &GameState, target: ObjectId) -> bool {
    state.objects.get(target).is_some_and(|t|
        t.zone.is_battlefield() && t.is_creature())
}

// =============================================================================
// 704.5r — Fortification attached to an illegal permanent
// =============================================================================

/// CR 704.5r — a Fortification attached to an illegal permanent
/// (non-land or off the battlefield) or to a nonpermanent becomes
/// unattached. Like the Equipment SBA this is a pure detach — the
/// Fortification stays on the battlefield.
///
/// Detection rides on the [`Characteristics::is_fortification`] flag
/// (synthesized by [`crate::registry::CardRegistry::register`] from
/// the printed subtypes), which keeps this SBA registry-free.
fn check_fortification_illegal(state: &mut GameState) -> bool {
    let to_detach: Vec<(ObjectId, ObjectId)> = state.objects.iter()
        .filter(|o| o.characteristics.is_fortification && o.zone.is_battlefield())
        .filter_map(|o| o.attached_to.map(|t| (o.id, t)))
        .filter(|(_, target)| !is_legal_fortification_target(state, *target))
        .collect();
    if to_detach.is_empty() { return false; }
    for (fort_id, prior_target) in to_detach {
        if let Some(obj) = state.objects.get_mut(fort_id) {
            obj.detach();
        }
        if let Some(holder) = state.objects.get_mut(prior_target) {
            holder.attachments.retain(|&id| id != fort_id);
        }
        state.emit(GameEvent::Detached {
            equipment_or_aura: fort_id,
            from: prior_target,
        });
    }
    true
}

fn pending_fortification_illegal(state: &GameState) -> bool {
    state.objects.iter()
        .filter(|o| o.characteristics.is_fortification && o.zone.is_battlefield())
        .filter_map(|o| o.attached_to)
        .any(|target| !is_legal_fortification_target(state, target))
}

fn is_legal_fortification_target(state: &GameState, target: ObjectId) -> bool {
    state.objects.get(target).is_some_and(|t|
        t.zone.is_battlefield() && t.is_land())
}

// =============================================================================
// 704.5n — Aura attached to an illegal object or unattached
// =============================================================================

/// CR 704.5n — an Aura is put into its owner's graveyard if it's
/// attached to an illegal object or player, or isn't attached to any
/// object or player. Detection rides on the [`Characteristics::is_aura`]
/// flag (synthesized by [`crate::registry::CardRegistry::register`]
/// from the printed subtypes), which lets this SBA stay
/// registry-free.
///
/// Legality here is conservative: an attached Aura is legal iff its
/// host object is still on the battlefield. Target-type filters
/// ("enchant creature" vs "enchant land") compose on top once an
/// Aura seed card forces the machinery — for Phase 2 no card
/// distinguishes them in a test.
fn check_aura_illegal(state: &mut GameState) -> bool {
    let to_move: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.characteristics.is_aura && o.zone.is_battlefield())
        .filter(|o| !is_legal_aura_state(state, o))
        .map(|o| o.id)
        .collect();
    if to_move.is_empty() { return false; }
    for id in to_move {
        let owner = state.objects.get(id).map(|o| o.owner);
        if let Some(owner) = owner {
            state.move_object_to_zone(
                id, Zone::Graveyard(owner), MoveCause::StateBasedAction);
        }
    }
    true
}

fn pending_aura_illegal(state: &GameState) -> bool {
    state.objects.iter()
        .filter(|o| o.characteristics.is_aura && o.zone.is_battlefield())
        .any(|o| !is_legal_aura_state(state, o))
}

fn is_legal_aura_state(
    state: &GameState,
    aura: &crate::objects::GameObject,
) -> bool {
    let Some(target) = aura.attached_to else { return false; };
    state.objects.get(target).is_some_and(|t| t.zone.is_battlefield())
}

// =============================================================================
// Game result
// =============================================================================

/// Apply the "last player standing wins / universal draw" rule.
/// Called once per SBA pass after the per-variant checks settle.
fn update_game_result(state: &mut GameState) {
    if state.result.is_some() { return; }
    let alive: Vec<PlayerId> = (0..state.num_players())
        .filter(|&p| !state.player(p).has_lost)
        .collect();
    if alive.is_empty() {
        state.result = Some(GameResult::Draw);
    } else if alive.len() == 1 {
        let winner = alive[0];
        state.result = Some(GameResult::Win(winner));
        state.emit(GameEvent::PlayerWins { player: winner });
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::objects::{Characteristics, GameObject};

    fn creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    fn put_creature(s: &mut GameState, owner: PlayerId, zone: Zone, p: i32, t: i32)
        -> ObjectId
    {
        let id = s.allocate_object_id();
        let mut obj = GameObject::new(id, owner, zone, 1, creature_chars(p, t));
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    fn put_pw(s: &mut GameState, owner: PlayerId, loyalty: i32) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::PLANESWALKER.into(),
            loyalty: Some(loyalty),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    fn put_legendary_creature(
        s: &mut GameState,
        owner: PlayerId,
        name: SmallString,
    ) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            name,
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet::new().with(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    // --- 704.5a / b / c: player loss ---------------------------------------

    #[test]
    fn life_zero_loses_the_game() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).life = 0;
        apply_state_based_actions(&mut s);
        assert!(s.player(0).has_lost);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::PlayerLoses { player: 0, reason: LoseReason::LifeZero })));
    }

    #[test]
    fn negative_life_also_loses() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).life = -3;
        apply_state_based_actions(&mut s);
        assert!(s.player(0).has_lost);
    }

    #[test]
    fn decked_player_loses() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).has_drawn_from_empty_library = true;
        apply_state_based_actions(&mut s);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::PlayerLoses { reason: LoseReason::Decked, .. })));
    }

    #[test]
    fn poison_ten_loses() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).poison_counters = 10;
        apply_state_based_actions(&mut s);
        assert!(s.player(0).has_lost);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::PlayerLoses { reason: LoseReason::PoisonCounters, .. })));
    }

    #[test]
    fn poison_nine_does_not_lose() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).poison_counters = 9;
        apply_state_based_actions(&mut s);
        assert!(!s.player(0).has_lost);
    }

    // --- 704.5f / g: creature graveyard ------------------------------------

    #[test]
    fn creature_with_zero_toughness_dies() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 1);
        // Put a -1/-1 counter on it → effective toughness 0.
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::MinusOneMinusOne, 1);
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Dies { object_id } if *object_id == c)));
    }

    #[test]
    fn creature_with_lethal_damage_dies() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().mark_damage(2);
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Dies { object_id } if *object_id == c)));
    }

    #[test]
    fn creature_below_lethal_survives() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 3);
        s.objects.get_mut(c).unwrap().mark_damage(2);
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn indestructible_survives_lethal_damage() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Indestructible);
        s.objects.get_mut(c).unwrap().mark_damage(20);
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn indestructible_survives_deathtouch_damage() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Indestructible);
        s.objects.get_mut(c).unwrap().mark_damage(1);
        s.objects.get_mut(c).unwrap().has_deathtouch_damage = true;
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn indestructible_still_dies_to_zero_toughness() {
        use crate::effects::KeywordAbility;
        use crate::layers::{ContinuousEffect, ContinuousEffectKind, Layer, Duration};
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Indestructible);
        // Drop toughness to 0 via a -2/-2 pump.
        s.add_continuous_effect(ContinuousEffect {
            source: 0,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration: Duration::Permanent,
            dependency: None,
            kind: ContinuousEffectKind::PumpTarget {
                target: c, power: 0, toughness: -2,
            },
        });
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
    }

    // --- 704.5i: planeswalker ----------------------------------------------

    #[test]
    fn planeswalker_zero_loyalty_goes_to_graveyard() {
        let mut s = GameState::new(2, 0);
        let pw = put_pw(&mut s, 0, 0);
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Dies { object_id } if *object_id == pw)));
    }

    #[test]
    fn planeswalker_with_loyalty_counters_survives() {
        let mut s = GameState::new(2, 0);
        let pw = put_pw(&mut s, 0, 0);
        // Add 3 loyalty counters; base 0 + 3 = 3 total loyalty.
        s.objects.get_mut(pw).unwrap().add_counters(CounterKind::Loyalty, 3);
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(pw).unwrap().zone, Zone::Battlefield);
    }

    // --- 704.5j: legend rule -----------------------------------------------

    /// Legend rule now yields a `PickCards` choice; the SBA loop
    /// breaks out with `pending_choice` set, waiting for the agent.
    #[test]
    fn legend_rule_pushes_pick_cards_choice() {
        use crate::actions::{ChoiceContext, ChoiceKind};
        let mut s = GameState::new(2, 0);
        let name = s.players[0].id as SmallString;
        let first = put_legendary_creature(&mut s, 0, name);
        let second = put_legendary_creature(&mut s, 0, name);
        let third = put_legendary_creature(&mut s, 0, name);

        apply_state_based_actions(&mut s);
        let pc = s.pending_choice.as_ref()
            .expect("legend rule should have pushed a choice");
        assert_eq!(pc.choosing_player, 0);
        assert!(matches!(pc.context, ChoiceContext::Sba));
        match &pc.kind {
            ChoiceKind::PickCards { candidates, min, max } => {
                assert_eq!(*min, 1);
                assert_eq!(*max, 1);
                assert_eq!(candidates, &vec![first, second, third]);
            }
            other => panic!("expected PickCards, got {other:?}"),
        }
        // No graveyard movement until the agent answers.
        assert_eq!(s.objects.get(first).unwrap().zone, Zone::Battlefield);
        assert_eq!(s.objects.get(second).unwrap().zone, Zone::Battlefield);
        assert_eq!(s.objects.get(third).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn legend_rule_ignores_different_names() {
        let mut s = GameState::new(2, 0);
        let name_a = 1 as SmallString;
        let name_b = 2 as SmallString;
        let a = put_legendary_creature(&mut s, 0, name_a);
        let b = put_legendary_creature(&mut s, 0, name_b);
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(a).unwrap().zone, Zone::Battlefield);
        assert_eq!(s.objects.get(b).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn legend_rule_ignores_different_controllers() {
        let mut s = GameState::new(2, 0);
        let name = 5 as SmallString;
        let mine = put_legendary_creature(&mut s, 0, name);
        let theirs = put_legendary_creature(&mut s, 1, name);
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(mine).unwrap().zone, Zone::Battlefield);
        assert_eq!(s.objects.get(theirs).unwrap().zone, Zone::Battlefield);
    }

    // --- 704.5p: +1/+1 / -1/-1 annihilation --------------------------------

    #[test]
    fn annihilation_removes_matching_pairs() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::PlusOnePlusOne, 3);
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::MinusOneMinusOne, 2);
        apply_state_based_actions(&mut s);
        let obj = s.objects.get(c).unwrap();
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 1);
        assert_eq!(obj.count_counters(CounterKind::MinusOneMinusOne), 0);
    }

    #[test]
    fn annihilation_does_not_apply_with_only_one_kind() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::PlusOnePlusOne, 2);
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(c).unwrap()
            .count_counters(CounterKind::PlusOnePlusOne), 2);
    }

    // --- Game-over detection ----------------------------------------------

    #[test]
    fn sole_survivor_wins() {
        let mut s = GameState::new(2, 0);
        s.player_mut(1).life = 0;
        apply_state_based_actions(&mut s);
        assert_eq!(s.result, Some(GameResult::Win(0)));
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::PlayerWins { player: 0 })));
    }

    #[test]
    fn both_losing_simultaneously_is_draw() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).life = 0;
        s.player_mut(1).life = 0;
        apply_state_based_actions(&mut s);
        assert_eq!(s.result, Some(GameResult::Draw));
    }

    #[test]
    fn three_player_last_standing_wins() {
        let mut s = GameState::new(3, 0);
        s.player_mut(0).life = 0;
        s.player_mut(2).life = 0;
        apply_state_based_actions(&mut s);
        assert_eq!(s.result, Some(GameResult::Win(1)));
    }

    // --- Idempotence / cascade ---------------------------------------------

    #[test]
    fn sba_is_idempotent_when_settled() {
        let mut s = GameState::new(2, 0);
        put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        assert_eq!(apply_state_based_actions(&mut s), 0);
        // Second call produces nothing further.
        assert_eq!(apply_state_based_actions(&mut s), 0);
    }

    // --- 704.5d: tokens cease to exist -------------------------------------

    /// A token in a non-battlefield zone ceases to exist in the same
    /// SBA loop pass as the zone move that put it there. Simulates a
    /// dying token: put the 1/1 token on the battlefield with lethal
    /// damage pre-marked, call SBAs once, expect both the move and
    /// the cease in a single iteration pair.
    #[test]
    fn token_dies_and_ceases_to_exist_in_one_sba_call() {
        let mut s = GameState::new(2, 0);
        let t = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(t).unwrap().is_token = true;
        s.objects.get_mut(t).unwrap().damage_marked = 1;

        apply_state_based_actions(&mut s);

        assert!(s.objects.get(t).is_none()
            && s.objects.iter().find(|o| o.id == t + 1).is_none(),
            "token removed from arena entirely after dying");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0,
            "no token residue in graveyard");
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
    }

    /// A non-token creature with the same setup goes to the graveyard
    /// and stays there. Negative control for the token-cease SBA.
    #[test]
    fn nontoken_creature_goes_to_graveyard_and_stays() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(c).unwrap().damage_marked = 1;

        apply_state_based_actions(&mut s);

        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
            "ordinary creature corpse remains in graveyard");
    }

    /// A token placed directly into a graveyard (simulating a side
    /// door that skipped the normal zone-move path) is still removed
    /// on the next SBA pass.
    #[test]
    fn token_in_graveyard_ceases_on_next_sba() {
        let mut s = GameState::new(2, 0);
        let t = put_creature(&mut s, 0, Zone::Graveyard(0), 1, 1);
        s.objects.get_mut(t).unwrap().is_token = true;
        assert!(pending_token_cease(&s));
        apply_state_based_actions(&mut s);
        assert!(s.objects.get(t).is_none(),
            "token in graveyard was removed");
        assert!(!pending_token_cease(&s));
    }

    #[test]
    fn sba_cascades_annihilate_then_die() {
        // A creature with +1/+1 and -1/-1 counters AND matching
        // base toughness: after annihilation the remaining -1/-1
        // counter drops toughness to 0, and the SBA loop catches
        // that in its second check. The cascade happens within a
        // single `apply_state_based_actions` call.
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::PlusOnePlusOne, 1);
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::MinusOneMinusOne, 2);
        // Base (1) + 1 − 2 = 0 toughness after annihilation.
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
    }

    // --- has_pending_state_based_actions -----------------------------------

    #[test]
    fn pending_checks_reflect_dirty_state() {
        let mut s = GameState::new(2, 0);
        assert!(!has_pending_state_based_actions(&s));
        s.player_mut(0).life = 0;
        assert!(has_pending_state_based_actions(&s));
    }

    // --- Full integration: bolt kills 1-toughness creature ----------------

    #[test]
    fn bolt_kills_bear_via_sba() {
        // Simulate Lightning Bolt (3 damage) hitting a Grizzly Bears (2/2).
        let mut s = GameState::new(2, 0);
        let bear = put_creature(&mut s, 1, Zone::Battlefield, 2, 2);
        s.deal_damage(99, crate::events::DamageTarget::Object(bear), 3, false);
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(1)), 1);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Dies { object_id } if *object_id == bear)));
    }

    #[test]
    fn bolt_kills_player_via_sba() {
        // Bolt at player for 3 × ~7 hits takes player from 20 to -1.
        let mut s = GameState::new(2, 0);
        for _ in 0..7 {
            s.deal_damage(99, crate::events::DamageTarget::Player(1), 3, false);
        }
        apply_state_based_actions(&mut s);
        assert!(s.player(1).has_lost);
        assert_eq!(s.result, Some(GameResult::Win(0)));
    }

    // --- 704.5n: Aura attachment -------------------------------------------

    fn put_aura(s: &mut GameState, owner: PlayerId, zone: Zone) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::ENCHANTMENT.into(),
            is_aura: true,
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, zone, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    #[test]
    fn unattached_aura_on_battlefield_goes_to_graveyard() {
        // CR 704.5n — an Aura with no attached_to belongs in the owner's
        // graveyard on the next SBA pass.
        let mut s = GameState::new(2, 0);
        let aura = put_aura(&mut s, 0, Zone::Battlefield);
        assert!(pending_aura_illegal(&s));
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
            "unattached Aura lands in owner's graveyard");
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
        // Original id is LKI; the graveyard object has a fresh id.
        let _ = aura;
    }

    #[test]
    fn aura_whose_host_left_goes_to_graveyard() {
        // Create an Aura attached to a creature that dies. The creature's
        // move to graveyard clears the Aura's attached_to; the SBA catches
        // the unattached Aura in the same pass.
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let aura = put_aura(&mut s, 0, Zone::Battlefield);
        // Wire up the attachment.
        s.objects.get_mut(aura).unwrap().attached_to = Some(creature);
        s.objects.get_mut(creature).unwrap().attachments.push(aura);
        // Lethal damage to creature.
        s.objects.get_mut(creature).unwrap().damage_marked = 3;

        apply_state_based_actions(&mut s);

        assert_eq!(s.zone_count(Zone::Battlefield), 0,
            "creature died and Aura followed");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 2,
            "creature + Aura both in owner's graveyard");
    }

    #[test]
    fn non_aura_enchantment_stays_on_battlefield() {
        // Glorious Anthem-shaped enchantment (not an Aura). attached_to
        // is None permanently; SBA must leave it alone.
        let mut s = GameState::new(2, 0);
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::ENCHANTMENT.into(),
            is_aura: false,  // explicit: not an Aura
            ..Default::default()
        };
        s.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, chars));
        assert!(!pending_aura_illegal(&s));
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Battlefield), 1,
            "non-Aura enchantment stays put");
    }

    #[test]
    fn aura_attached_to_legal_target_stays() {
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let aura = put_aura(&mut s, 0, Zone::Battlefield);
        s.objects.get_mut(aura).unwrap().attached_to = Some(creature);
        s.objects.get_mut(creature).unwrap().attachments.push(aura);

        apply_state_based_actions(&mut s);

        assert_eq!(s.zone_count(Zone::Battlefield), 2,
            "both remain on the battlefield");
        assert!(s.objects.get(aura).is_some_and(|o|
            o.attached_to == Some(creature)));
    }

    // --- 704.5r: Fortification attachment ----------------------------------

    fn put_land(s: &mut GameState, owner: PlayerId) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::LAND.into(),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    fn put_fortification(s: &mut GameState, owner: PlayerId) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::ARTIFACT.into(),
            is_fortification: true,
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    fn put_equipment(s: &mut GameState, owner: PlayerId) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::ARTIFACT.into(),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    #[test]
    fn fortification_on_legal_land_stays_attached() {
        let mut s = GameState::new(2, 0);
        let land = put_land(&mut s, 0);
        let fort = put_fortification(&mut s, 0);
        s.objects.get_mut(fort).unwrap().attached_to = Some(land);
        s.objects.get_mut(land).unwrap().attachments.push(fort);

        assert!(!pending_fortification_illegal(&s));
        apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(Zone::Battlefield), 2);
        assert_eq!(s.objects.get(fort).unwrap().attached_to, Some(land));
    }

    #[test]
    fn fortification_on_creature_detaches() {
        // A Fortification attached to a non-land permanent becomes
        // unattached (stays on the battlefield — unlike an illegally-
        // attached Aura, which is moved to the graveyard).
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let fort = put_fortification(&mut s, 0);
        s.objects.get_mut(fort).unwrap().attached_to = Some(creature);
        s.objects.get_mut(creature).unwrap().attachments.push(fort);

        assert!(pending_fortification_illegal(&s));
        apply_state_based_actions(&mut s);

        assert_eq!(s.zone_count(Zone::Battlefield), 2,
            "both still on the battlefield");
        assert_eq!(s.objects.get(fort).unwrap().attached_to, None,
            "Fortification detached from creature");
        assert!(s.objects.get(creature).unwrap().attachments.is_empty(),
            "creature's attachments list cleared");
    }

    #[test]
    fn fortification_whose_land_left_becomes_unattached() {
        // When the host land leaves the battlefield, the engine clears
        // attached_to on the Fortification; the SBA loop still runs the
        // check, but no-ops because the Fort is already unattached.
        let mut s = GameState::new(2, 0);
        let land = put_land(&mut s, 0);
        let fort = put_fortification(&mut s, 0);
        s.objects.get_mut(fort).unwrap().attached_to = Some(land);
        s.objects.get_mut(land).unwrap().attachments.push(fort);
        // Ship the land to the graveyard directly.
        s.move_object_to_zone(
            land, Zone::Graveyard(0), MoveCause::StateBasedAction);

        apply_state_based_actions(&mut s);

        assert_eq!(s.objects.get(fort).unwrap().attached_to, None,
            "Fortification left unattached once its land was gone");
        assert_eq!(s.zone_count(Zone::Battlefield), 1,
            "Fortification stays on the battlefield");
    }

    #[test]
    fn equipment_sba_ignores_fortification_on_land() {
        // Regression guard: before splitting the SBA, the Equipment
        // heuristic (artifact + !enchantment + attached_to.is_some())
        // would match a Fortification on a land and wrongly detach it
        // because lands aren't creatures. With is_fortification now
        // excluded from is_equipment_style, the Fort stays attached.
        let mut s = GameState::new(2, 0);
        let land = put_land(&mut s, 0);
        let fort = put_fortification(&mut s, 0);
        s.objects.get_mut(fort).unwrap().attached_to = Some(land);
        s.objects.get_mut(land).unwrap().attachments.push(fort);

        assert!(!pending_attachment_illegal(&s),
            "Equipment SBA must ignore Fortifications");
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(fort).unwrap().attached_to, Some(land));
    }

    #[test]
    fn equipment_on_land_still_detaches() {
        // Symmetry check: a plain Equipment (not a Fortification)
        // attached to a land is still illegal under CR 704.5q.
        let mut s = GameState::new(2, 0);
        let land = put_land(&mut s, 0);
        let equip = put_equipment(&mut s, 0);
        s.objects.get_mut(equip).unwrap().attached_to = Some(land);
        s.objects.get_mut(land).unwrap().attachments.push(equip);

        assert!(pending_attachment_illegal(&s));
        apply_state_based_actions(&mut s);
        assert_eq!(s.objects.get(equip).unwrap().attached_to, None);
    }
}
