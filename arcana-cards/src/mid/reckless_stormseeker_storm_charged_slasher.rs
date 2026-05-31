//! Reckless Stormseeker // Storm-Charged Slasher (transforming DFC, layout "transform")
//!
//! Front face: Reckless Stormseeker — {2}{R} Creature — Human Werewolf, 2/3.
//!   At the beginning of combat on your turn, target creature you control gets
//!     +1/+0 and gains haste until end of turn.
//!   Daybound.
//! Back face: Storm-Charged Slasher — Creature — Werewolf, 2/3 (same printed stats).
//!   At the beginning of combat on your turn, target creature you control gets
//!     +2/+0 and gains trample and haste until end of turn.
//!   Nightbound.
//!
//! GAP: Daybound / Nightbound automatic day/night transform is not expressible —
//!   there is no exposed day/night-change trigger condition in the demonstrated API
//!   (only `it_is_day` / `it_is_night` predicates and `Effect::SetDayNight`). The
//!   directional combat triggers (the gameplay payload) are authored and face-gated;
//!   the day/night-driven flip itself is engine debt.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Stormseeker");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Storm-Charged Slasher");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    let creature_you_control = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: beginning of combat on your turn — target creature you control
            // gets +1/+0 and gains haste until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_combat,
                trigger_zones: vec![arcana_core::zones::Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![creature_you_control.clone()],
            })
            // Back: beginning of combat on your turn — target creature you control
            // gets +2/+0 and gains trample and haste until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_combat,
                trigger_zones: vec![arcana_core::zones::Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![creature_you_control],
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn front_combat(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        },
    ]
}

fn back_combat(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        },
    ]
}
