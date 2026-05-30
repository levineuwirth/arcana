//! Instigator Gang // Wildblood Pack — `{3}{R}` Human Werewolf 2/3 (front).
//! Front: Attacking creatures you control get +1/+0. At the beginning of each upkeep, if
//! no spells were cast last turn, transform.
//! Back (Wildblood Pack): Trample. Attacking creatures you control get +3/+0. At the
//! beginning of each upkeep, if a player cast two or more spells last turn, transform back.
//!
//! # GAPs
//! - Daybound/Nightbound / "no spells cast last turn" / "two or more spells" trigger
//!   conditions are not modeled. Transform wired to beginning-of-upkeep as approximation.
//! - "Attacking creatures you control get +N/+0" is a continuous static pump that is not
//!   expressible as a one-shot effect — GAP on both faces.
//! - Back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Instigator Gang");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Wildblood Pack");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Trample],
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: front "attacking creatures get +1/+0" — continuous effect, not modeled.
            // GAP: back "attacking creatures get +3/+0" — continuous effect, not modeled.
            // Front->back: GAP: exact condition "no spells cast last turn" not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: transform_front_to_back,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn transform_front_to_back(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire when no spells were cast last turn (front->back condition)
    // GAP: back->front condition (two or more spells) not modeled — back-face-only trigger
    vec![Effect::Transform { target: trig.source }]
}
