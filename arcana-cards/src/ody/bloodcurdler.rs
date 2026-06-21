//! Bloodcurdler — `{1}{B}` 1/1 Horror with Flying.
//!
//! "Flying. At the beginning of your upkeep, mill a card. Threshold —
//! As long as there are seven or more cards in your graveyard, this
//! creature gets +1/+1 and has 'At the beginning of your end step,
//! exile two cards from your graveyard.'"
//!
//! Flying is a base keyword (Threshold/Mill are not supported keyword
//! variants — only Flying is emitted). The upkeep trigger mills a card.
//! The Threshold conditional static (+1/+1 and a granted end-step
//! exile ability while gy >= 7) is a continuous conditional static with
//! no primitive in this API surface — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodcurdler");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — Threshold "+1/+1 and gains 'At the beginning of your
    // end step, exile two cards from your graveyard' while gy >= 7" (no
    // conditional continuous static / conditional granted-ability
    // primitive in this API surface).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 1,
    }]
}
