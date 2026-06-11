//! Fae Offering — `{2}{G}` enchantment.
//! "At the beginning of each end step, if you've cast both a creature
//! spell and a noncreature spell this turn, create a Clue token, a
//! Food token, and a Treasure token."
//!
//! An each-end-step trigger with a CR 603.4 intervening-if implemented
//! via `script::spells_cast_this_turn` over a creature filter and a
//! noncreature filter; the payoff mints the three commodity tokens.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fae Offering");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_cast_creature_and_noncreature),
                effect: mint_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if you've cast both a creature spell and a noncreature spell this
/// turn…"
fn if_cast_creature_and_noncreature(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let creature = ObjectFilter::new().with_types(TypeLine::CREATURE.into());
    let noncreature =
        ObjectFilter::new().without_types(TypeLine::CREATURE.into());
    script::spells_cast_this_turn(s, &creature, you) >= 1
        && script::spells_cast_this_turn(s, &noncreature, you) >= 1
}

/// "…create a Clue token, a Food token, and a Treasure token."
fn mint_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Clue,
            count: 1,
        },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Food,
            count: 1,
        },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
    ]
}
