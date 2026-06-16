//! Optimus Prime, Hero // Optimus Prime, Autobot Leader — transforming ("convert") DFC
//!
//! Front: {3}{U}{R}{W} Legendary Artifact Creature — Robot 4/8, Trample
//!   More Than Meets the Eye {2}{U}{R}{W} (alternate "converted" cast).
//!   At the beginning of each end step, bolster 1.
//!   When Optimus Prime dies, return it to the battlefield converted under its owner's control.
//! Back: Optimus Prime, Autobot Leader — Legendary Artifact — Vehicle
//!   Living metal (During your turn, this Vehicle is also a creature.)
//!   Trample
//!   Whenever you attack, bolster 2. The chosen creature gains trample until end of turn.
//!   When that creature deals combat damage to a player this turn, convert Optimus Prime.
//!
//! "Convert" is modeled as Effect::Transform.
//! GAP: "More Than Meets the Eye" alternate converted cast is not modeled (no convert-cast modifier).
//! GAP: "Living metal" (Vehicle-is-also-a-creature on your turn) is not a usable keyword.
//! GAP: "Bolster N" (choose creature you control with least toughness, add N +1/+1 counters)
//!   is not expressible — no bolster effect / least-toughness selection — so both bolster bodies
//!   are emitted empty.
//! GAP: the back-face attack rider ("gains trample, then convert on combat damage") and the
//!   dies "return converted" payload are not modeled; their trigger bodies are emitted empty.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Optimus Prime, Hero");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // Back face: Optimus Prime, Autobot Leader — Legendary Artifact — Vehicle
    let back_name = reg.interner_mut().intern("Optimus Prime, Autobot Leader");
    let vehicle = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(8)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: at the beginning of each end step, bolster 1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: bolster_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front: when Optimus Prime dies, return it to the battlefield converted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: whenever you attack, bolster 2 (+ trample + delayed convert).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 1),
    )
}

fn bolster_noop(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Bolster 1" — choose creature you control with least toughness, add a +1/+1 counter.
    // No bolster effect / least-toughness selection in the API.
    Vec::new()
}

fn dies_noop(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "return it to the battlefield converted under its owner's control" — return-as-back-face
    // reanimation of self is not modeled.
    Vec::new()
}

fn attack_noop(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "bolster 2; the chosen creature gains trample; when that creature deals combat damage to
    // a player this turn, convert Optimus Prime" — bolster + chosen-creature delayed-convert rider
    // not modeled.
    Vec::new()
}
