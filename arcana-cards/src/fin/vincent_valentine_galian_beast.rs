//! Vincent Valentine // Galian Beast
//!
//! Front: {2}{B}{B} Legendary Creature — Assassin 2/2
//! Whenever a creature an opponent controls dies, put a number of +1/+1 counters
//! on Vincent Valentine equal to that creature's power. The dying creature's
//! object id is read from the ZoneChange event and its power looked up in
//! last-known information (`state.lki`), which snapshots the pre-death
//! characteristics — so the power IS accessible for the typical fixed-P/T case.
//! GAP: a dying creature with `*` power (CDA) reads as 0 via LKI base
//! characteristics; the common fixed-power case is faithful.
//! Whenever Vincent Valentine attacks, you may transform it.
//!
//! Back: Legendary Creature — Werewolf Beast
//! Trample, lifelink.
//! "When Galian Beast dies, return it to the battlefield (front face up)." Wired
//! as a back-face SelfDies trigger (gated to face 1) emitting
//! Effect::ReturnFromGraveyardToBattlefield — the returned permanent is a fresh
//! object showing its default (front) face, satisfying "front face up".
//! GAP: "tapped" — ReturnFromGraveyardToBattlefield has no tapped option (same
//! documented gap as Tenacious Dead); the creature returns untapped.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::GameEvent;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vincent Valentine");
    let assassin_sub = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Galian Beast");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let beast_sub = reg.interner_mut().intern("Beast");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub);
    back_subtypes.0.insert(beast_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Lifelink],
            // GAP: "When Galian Beast dies, return it to the battlefield tapped
            // (front face up)" — back-face-only triggered ability not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: whenever a creature an opponent controls dies, put +1/+1
            // counters equal to that creature's power on Vincent Valentine.
            // GAP: power of dying creature not accessible — emitting Vec::new().
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: opponent_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: whenever Vincent Valentine attacks, you may transform it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 3 (back face): when Galian Beast dies, return it to the
            // battlefield (front face up; the returned object is a fresh permanent
            // showing its default front face).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: galian_beast_dies_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Triggers 1 & 2 are front-face abilities; trigger 3 is the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 1),
    )
}

fn opponent_creature_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The dying creature's id is the OLD (battlefield) id carried on the
    // ZoneChange event; its pre-death characteristics live in LKI.
    let GameEvent::ZoneChange { object_id, .. } = &trig.trigger_event else {
        return Vec::new();
    };
    let power = state
        .lki
        .get(object_id)
        .and_then(|o| o.characteristics.power)
        .map(|pt| match pt {
            PtValue::Fixed(p) => p,
            // GAP: `*`/`*+N` power reads as 0 via LKI base characteristics.
            PtValue::Star => 0,
            PtValue::StarPlus(n) => n,
        })
        .unwrap_or(0);
    if power <= 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: power as u32,
    }]
}

fn galian_beast_dies_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tapped" — ReturnFromGraveyardToBattlefield has no tapped option
    // (same as Tenacious Dead); returns untapped, front face up.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}

fn attacks_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
