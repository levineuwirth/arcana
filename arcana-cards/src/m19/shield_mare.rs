//! Shield Mare — `{1}{W}{W}` 2/3 Creature — Horse.
//!
//! * "This creature can't be blocked by red creatures." — color-filtered
//!   block restriction. `Effect::CantBeBlocked` is unconditional (no
//!   blocker-color filter), so this evasion static is GAP'd.
//! * "When this creature enters or becomes the target of a spell or ability an
//!   opponent controls, you gain 3 life." — an OR of two trigger events;
//!   emitted as two TriggeredAbilityDefs (enters; becomes the target of an
//!   opponent's spell/ability), each gaining 3 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shield Mare");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: "can't be blocked by red creatures" — CantBeBlocked has no
    // blocker-color filter.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: gain_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: gain_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 3 }]
}
