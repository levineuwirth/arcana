//! First Response — `{3}{W}` enchantment.
//! "At the beginning of each upkeep, if you lost life last turn,
//! create a 1/1 white Soldier creature token."
//!
//! GAP: intervening-if — "you lost life last turn" is not expressible
//! (the conditions/script surface exposes no last-turn life-loss
//! accessor), so the trigger fires unconditionally; documented here
//! per the prompt's instruction.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("First Response");
    let _soldier = reg.interner_mut().intern("Soldier");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if — "if you lost life last turn" has
                // no available predicate (only this-turn accessors
                // exist); firing unconditionally.
                intervening_if: None,
                effect: make_soldier,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 1/1 white Soldier creature token."
fn make_soldier(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
