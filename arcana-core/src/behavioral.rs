//! Behavioral smoke-probe — does a card's resolver actually DO anything?
//!
//! Layer-2 verify certifies a card's bones; the L3 stub check is purely
//! TEXTUAL (it greps the source for `Vec::new()` / `// GAP`). Neither
//! catches the dangerous middle case: a resolver that returns a
//! NON-EMPTY effect list which, when executed, changes nothing
//! observable. That's exactly what hid the `Effect::ForEach`
//! no-substitution bug across ~294 catalog cards — they compiled, had
//! "some effect", and silently no-op'd.
//!
//! This probe closes that blind spot. It builds a generously-populated
//! game state, drives the card's spell resolver, executes the resulting
//! effects, and reports whether anything in a broad state fingerprint
//! moved. A card whose resolver yields effects but produces NO delta is
//! a silent-no-op suspect.
//!
//! Scope: spell abilities (instants/sorceries — the cleanest path, no
//! permanent/timing setup). Triggered/activated resolvers can reuse the
//! same `Snapshot` machinery later.

use crate::effects::Effect;
use crate::objects::{Characteristics, GameObject, ObjectId};
use crate::registry::CardRegistry;
use crate::stack::StackEntry;
use crate::state::GameState;
use crate::targets::{ObjectOrPlayer, TargetChoice, TargetFilter, TargetSelection};
use crate::types::{CardId, PlayerId, PtValue, TypeLine};
use crate::zones::Zone;

/// A broad fingerprint of everything a resolving spell could plausibly
/// change. Two snapshots differing anywhere = the resolver did
/// something observable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Snapshot {
    life: Vec<i32>,
    energy: Vec<u32>,
    poison: Vec<u32>,
    /// (battlefield, hand, graveyard, library) object counts per player.
    zones: Vec<(usize, usize, usize, usize)>,
    exile: usize,
    total_objects: usize,
    total_counters: u32,
    total_damage: u32,
    tapped: usize,
    pending_choice: bool,
    stack_len: usize,
    /// Installed continuous effects (pump, anthems, keyword/type/colour
    /// grants) — without this, every combat-trick / anthem spell looks
    /// like a no-op because P/T and granted abilities live in layers,
    /// not on the object's printed characteristics.
    continuous_effects: usize,
    dungeons: Vec<bool>,
}

impl Snapshot {
    pub fn capture(state: &GameState) -> Self {
        let n = state.num_players();
        let mut life = Vec::with_capacity(n as usize);
        let mut energy = Vec::with_capacity(n as usize);
        let mut poison = Vec::with_capacity(n as usize);
        let mut zones = Vec::with_capacity(n as usize);
        let mut dungeons = Vec::with_capacity(n as usize);
        for p in 0..n {
            life.push(state.player(p).life);
            energy.push(state.player(p).energy);
            poison.push(state.player(p).poison_counters);
            dungeons.push(state.player(p).dungeon.is_some());
            zones.push((
                state.objects.objects_in_zone(Zone::Battlefield)
                    .filter(|o| o.controller == p).count(),
                state.objects.count_in_zone(Zone::Hand(p)),
                state.objects.count_in_zone(Zone::Graveyard(p)),
                state.objects.count_in_zone(Zone::Library(p)),
            ));
        }
        let total_counters: u32 = state.objects.iter()
            .map(|o| {
                let kinds: Vec<_> = o.counters.keys().copied().collect();
                kinds.iter().map(|k| o.count_counters(*k)).sum::<u32>()
            })
            .sum();
        let total_damage: u32 = state.objects.iter().map(|o| o.damage_marked).sum();
        let tapped = state.objects.iter().filter(|o| o.is_tapped()).count();
        Self {
            life, energy, poison, zones,
            exile: state.objects.count_in_zone(Zone::Exile),
            total_objects: state.objects.iter().count(),
            total_counters,
            total_damage,
            tapped,
            pending_choice: state.pending_choice.is_some(),
            stack_len: state.stack_size(),
            continuous_effects: state.continuous_effects.len(),
            dungeons,
        }
    }
}

/// Outcome of probing one card.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProbeResult {
    /// The resolver returned at least one `Effect`.
    pub had_effects: bool,
    /// Executing those effects changed the state fingerprint.
    pub observable_delta: bool,
}

impl ProbeResult {
    /// A silent-no-op suspect: produced effects, but nothing moved.
    pub fn is_silent_noop(&self) -> bool {
        self.had_effects && !self.observable_delta
    }
}

/// Probe a spell card. Returns `None` if the card has no spell ability
/// (not in scope for this probe).
pub fn probe_spell(reg: &CardRegistry, card_id: CardId) -> Option<ProbeResult> {
    let def = reg.get(card_id)?;
    let spell = def.spell_ability.as_ref()?;

    let mut state = populated_state();
    // The spell's source object, sitting on the stack.
    let src = state.allocate_object_id();
    state.objects.insert(GameObject::new(
        src, 0, Zone::Stack, card_id, Characteristics::default()));
    state.currently_resolving = Some(src);

    // Build a target selection that satisfies each requirement with a
    // legal-shaped referent from the populated board.
    let dummy = first_battlefield_creature(&state, 0);
    let targets = selection_for(&spell.target_requirements, dummy);
    let entry = StackEntry::new_spell(
        src, 0, card_id, Characteristics::default(),
        targets, Vec::new(), None);

    let before = Snapshot::capture(&state);
    let effects: Vec<Effect> = (spell.effect)(&state, &entry, reg);
    let had_effects = !effects.is_empty();
    for eff in &effects {
        eff.execute(&mut state);
        // Real resolution PARKS when an effect posts a choice and
        // resumes after the answer; executing further effects
        // straight-line would trip the single-pending-choice invariant.
        // A posted choice is itself an observable delta, so stop here.
        if state.pending_choice.is_some() { break; }
    }
    let after = Snapshot::capture(&state);
    Some(ProbeResult { had_effects, observable_delta: before != after })
}

/// A 2-player state stocked so most effects have something to act on:
/// libraries with cards (draw/mill), creatures on each battlefield
/// (targets + board-wide), cards in hand and graveyard.
fn populated_state() -> GameState {
    let mut state = GameState::new(2, 0);
    for p in 0..2 {
        for _ in 0..3 { let c = make_creature(&mut state, p, Zone::Battlefield); let _ = c; }
        // Stock the library (top-to-bottom order matters for draw/dig).
        let mut lib = Vec::new();
        for _ in 0..6 { lib.push(make_card(&mut state, p, Zone::Library(p))); }
        state.player_mut(p).library_top_to_bottom = lib;
        for _ in 0..2 { make_card(&mut state, p, Zone::Hand(p)); }
        for _ in 0..2 { make_card(&mut state, p, Zone::Graveyard(p)); }
    }
    state
}

fn make_creature(state: &mut GameState, controller: PlayerId, zone: Zone) -> ObjectId {
    let id = state.allocate_object_id();
    let chars = Characteristics {
        types: TypeLine::CREATURE.into(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    state.objects.insert(GameObject::new(id, controller, zone, 0, chars));
    id
}

fn make_card(state: &mut GameState, owner: PlayerId, zone: Zone) -> ObjectId {
    let id = state.allocate_object_id();
    state.objects.insert(GameObject::new(id, owner, zone, 0, Characteristics::default()));
    id
}

fn first_battlefield_creature(state: &GameState, controller: PlayerId) -> Option<ObjectId> {
    state.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.controller == controller && o.is_creature())
        .map(|o| o.id)
}

/// One `TargetChoice` per requirement, shaped to the filter: players for
/// player-ish filters, a battlefield creature otherwise. The opponent
/// (player 1) is used for player targets so "target opponent" clauses
/// see a legal referent.
fn selection_for(
    reqs: &[crate::targets::TargetRequirement],
    dummy: Option<ObjectId>,
) -> TargetSelection {
    let mut sel = TargetSelection::new();
    for req in reqs {
        let choice = match (&req.filter, dummy) {
            (TargetFilter::Player, _) => TargetChoice::Player(1),
            (TargetFilter::CreatureOrPlayer, Some(id))
            | (TargetFilter::AnyTarget, Some(id)) =>
                TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)),
            (TargetFilter::CreatureOrPlayer, None)
            | (TargetFilter::AnyTarget, None) =>
                TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(1)),
            (_, Some(id)) => TargetChoice::Object(id),
            (_, None) => TargetChoice::Player(1),
        };
        sel.targets.push(choice);
    }
    sel
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{CardDefinition, SpellAbilityDef};

    fn draws_a_card(_s: &GameState, e: &StackEntry, _r: &CardRegistry) -> Vec<Effect> {
        vec![Effect::DrawCards { player: e.controller, count: 1 }]
    }
    fn noop_nonempty(_s: &GameState, _e: &StackEntry, _r: &CardRegistry) -> Vec<Effect> {
        // Non-empty, but targets nothing real — the ForEach-class bug shape.
        vec![Effect::DestroyPermanent { target: crate::objects::NULL_OBJECT_ID }]
    }
    fn honest_gap(_s: &GameState, _e: &StackEntry, _r: &CardRegistry) -> Vec<Effect> {
        Vec::new()
    }

    fn register(reg: &mut CardRegistry, name: &str, effect: crate::registry::SpellEffectFn) -> CardId {
        let n = reg.interner_mut().intern(name);
        let chars = Characteristics { name: n, types: TypeLine::INSTANT.into(), ..Default::default() };
        reg.register(CardDefinition::new(n, chars).with_spell_ability(SpellAbilityDef {
            text: name.into(),
            target_requirements: Vec::new(),
            modal: None,
            effect,
        }))
    }

    #[test]
    fn live_spell_shows_a_delta() {
        let mut reg = CardRegistry::new();
        let id = register(&mut reg, "Live", draws_a_card);
        let r = probe_spell(&reg, id).unwrap();
        assert!(r.had_effects && r.observable_delta, "drawing a card is observable");
        assert!(!r.is_silent_noop());
    }

    #[test]
    fn silent_noop_is_flagged() {
        // The whole point: non-empty effects, zero delta → caught.
        let mut reg = CardRegistry::new();
        let id = register(&mut reg, "Silent", noop_nonempty);
        let r = probe_spell(&reg, id).unwrap();
        assert!(r.had_effects, "returned an effect");
        assert!(!r.observable_delta, "but destroying NULL changes nothing");
        assert!(r.is_silent_noop(), "flagged as a silent no-op");
    }

    #[test]
    fn honest_gap_is_not_a_noop_suspect() {
        // Empty effects = honest GAP (L3's job), NOT a silent no-op.
        let mut reg = CardRegistry::new();
        let id = register(&mut reg, "Gap", honest_gap);
        let r = probe_spell(&reg, id).unwrap();
        assert!(!r.had_effects);
        assert!(!r.is_silent_noop(), "an honest GAP is not a silent no-op");
    }
}
