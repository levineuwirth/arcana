//! Second Chance — `{2}{U}` enchantment.
//! "At the beginning of your upkeep, if you have 5 or less life, sacrifice
//! this enchantment and take an extra turn after this one."

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Second Chance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_life_five_or_less),
                effect: sacrifice_for_extra_turn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// Intervening-if: "if you have 5 or less life".
fn if_life_five_or_less(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::life_at_most(s, you, 5)
}

/// "…sacrifice this enchantment and take an extra turn after this one."
fn sacrifice_for_extra_turn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Self-sacrifice via a name-filtered Sacrifice (no sacrifice-by-id
    // effect exists).
    let nm = reg.interner().lookup("Second Chance");
    // GAP: "take an extra turn after this one" — no extra-turn Effect
    // variant.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        count: 1,
    }]
}
