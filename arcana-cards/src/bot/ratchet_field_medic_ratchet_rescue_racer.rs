//! Ratchet, Field Medic // Ratchet, Rescue Racer
//!
//! Front: {2}{W} Legendary Artifact Creature — Robot 2/4
//!   More Than Meets the Eye {1}{W} (alternate cast cost — GAP: not in keyword API)
//!   Lifelink
//!   Whenever you gain life, you may convert Ratchet. When you do, return target artifact card
//!   with mana value <= amount of life gained this turn from your graveyard to battlefield tapped.
//!
//! Back: Legendary Artifact — Vehicle (Ratchet, Rescue Racer) 4/6
//!   Living metal (During your turn, this Vehicle is also a creature.) — GAP: not modeled
//!   Lifelink
//!   Whenever one or more nontoken artifacts you control are put into a graveyard from the
//!   battlefield, convert Ratchet. This ability triggers only once each turn.
//!
//! GAP: "More Than Meets the Eye" — alternate casting cost mechanic, not in keyword list.
//! GAP: "Living metal" — Vehicle-becomes-creature during your turn, not modeled.
//! GAP: "Convert" — same as Transform in Transformers context; modeled via Effect::Transform.
//! Front-face trigger "Whenever you gain life" wired via TriggerCondition::LifeGained; the
//!   convert (transform) half is emitted.
//! GAP: the chained "when you do, return target artifact card with mv <= life gained this turn
//!   from your graveyard to the battlefield tapped" rider needs dynamic life-gained-this-turn
//!   tracking + a reflexive "when you do" trigger; not expressible, so it is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ratchet, Field Medic");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ratchet, Rescue Racer");
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
            // Back face P/T for Vehicle form; Living metal makes it a creature during your turn
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Lifelink],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face only: whenever you gain life, you may convert Ratchet.
            // GAP: the "when you do, return target artifact card with mv <= life gained this turn"
            // rider needs dynamic life-gained tracking + a reflexive trigger; convert only.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_gain_life_convert,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face only: whenever one or more nontoken artifacts you control are put into a
            // graveyard from the battlefield, convert Ratchet. Triggers only once each turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .nontoken()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_artifact_dies_convert,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            // Trigger 1 fires only on the front face; trigger 2 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn on_gain_life_convert(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "when you do, return target artifact card with mv <= life gained this turn from your
    // graveyard to the battlefield tapped" — dynamic life-gained-this-turn + reflexive "when you
    // do" trigger not expressible; emitting just the convert (transform).
    vec![Effect::Transform { target: trig.source }]
}

fn on_artifact_dies_convert(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
