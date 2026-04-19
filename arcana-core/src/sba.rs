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
//! | 704.5d,e| Tokens/copies in wrong zone         | ⬜ (no token tracking) |
//! | 704.5k,m,n | Aura/equip attachment rules      | ⬜ (deferred) |
//! | 704.5q,r,s,t | Saga/battle/dungeon/ownership  | ⬜ (deferred) |
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

/// Current loyalty of a planeswalker object = base loyalty + Loyalty
/// counters. Returns 0 for non-planeswalkers.
fn planeswalker_loyalty(obj: &crate::objects::GameObject) -> i32 {
    let base = obj.characteristics.loyalty.unwrap_or(0);
    let counters = obj.count_counters(CounterKind::Loyalty) as i32;
    base + counters
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
}
