//! Prowl, Stoic Strategist // Prowl, Pursuit Vehicle — `{3}{W}` Legendary Artifact Creature
//! — Robot 3/3.
//!
//! Front face abilities:
//! - More Than Meets the Eye {2}{W}: cast converted (GAP — MTTE alternate cast not modeled).
//! - Whenever Prowl attacks, exile up to one other target tapped creature or Vehicle. For as
//!   long as that card remains exiled, its owner may play it.
//!   (GAP: exile-with-play-permission not modeled; ExilePermanent used without the rider.)
//! - Whenever a player plays a card exiled with Prowl, you draw a card and convert Prowl.
//!   (GAP: "exiled with Prowl" tracking not modeled; trigger omitted.)
//!
//! Back face (Prowl, Pursuit Vehicle) — Legendary Artifact — Vehicle.
//! - Living Metal: GAP — not modeled.
//! - Whenever another creature or Vehicle you control enters, put a +1/+1 counter on Prowl.
//!   If this is the second time this ability has resolved this turn, convert Prowl.
//!   The +1/+1-counter half is wired (back-face only), as two disjoint ETB triggers (creatures
//!   you control, and non-creature Vehicles you control — disjoint so a creature-Vehicle counts
//!   once). GAP: the "if this is the second time this ability has resolved this turn, convert"
//!   rider is not expressible — no per-turn ability-resolution counter is accessible to a script
//!   effect; that rider is omitted.
//!
//! # GAP notes
//! - More Than Meets the Eye alternate cast mechanic: not modeled.
//! - Convert keyword: Transform used as stand-in for face-flip, but convert semantics differ.
//! - Living Metal (Vehicle-as-creature-during-your-turn): not modeled.
//! - Exile-with-play-permission on attack: ExilePermanent used without the rider.
//! - "Exiled with Prowl" tracking and draw-on-play trigger: not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prowl, Stoic Strategist");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Prowl, Pursuit Vehicle — Legendary Artifact — Vehicle
    let back_name = reg.interner_mut().intern("Prowl, Pursuit Vehicle");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // Living metal: GAP — not modeled.
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face attack trigger: exile up to one other target tapped creature or Vehicle.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: prowl_attack_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .tapped_only(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // Back face only: whenever another creature you control enters, put a +1/+1 counter
            // on Prowl. ("Creature or Vehicle" is a type-OR-subtype that one ObjectFilter can't
            // express; split into two disjoint ETB triggers — creatures, and non-creature
            // Vehicles — so a creature-Vehicle counts exactly once.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: add_counter_to_prowl,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face only: whenever another (non-creature) Vehicle you control enters, put a
            // +1/+1 counter on Prowl.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .with_subtype_sym(vehicle_sub)
                        .without_types(TypeLine::CREATURE.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: add_counter_to_prowl,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "if this is the second time this ability has resolved this turn, convert Prowl"
            // — no per-turn ability-resolution counter accessible to a script effect; omitted.
            // Trigger 1 fires only on the front face; triggers 2 and 3 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1)
    )
}

fn add_counter_to_prowl(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn prowl_attack_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: exile-with-play-permission not modeled; bare exile only.
    vec![Effect::ExilePermanent { target: *id }]
}
