//! Lifetap — `{U}{U}` enchantment (Legends, 1994).
//! "Whenever a Forest an opponent controls becomes tapped, you gain 1
//! life."
//!
//! Wired via `TriggerCondition::BecomesTapped` with a Forest-subtype
//! filter constrained to opponent-controlled permanents.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lifetap");
    let forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // "Whenever a Forest an opponent controls becomes tapped"
                // — filtered BecomesTapped over opponent-controlled Forests.
                trigger_condition: TriggerCondition::BecomesTapped {
                    filter: ObjectFilter::permanent()
                        .with_subtype_sym(forest)
                        .controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: gain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain 1 life."
fn gain_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}
