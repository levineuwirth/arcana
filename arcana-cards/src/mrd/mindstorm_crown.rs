//! Mindstorm Crown — `{3}` artifact (Mercadian Masques, 1999).
//! "At the beginning of your upkeep, draw a card if you had no cards in
//! hand at the beginning of this turn. If you had a card in hand, this
//! artifact deals 1 damage to you."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mindstorm Crown");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
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
                intervening_if: None,
                effect: upkeep_draw_or_burn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn upkeep_draw_or_burn(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you had no cards in hand at the beginning of THIS TURN" —
    // turn-start hand size is not recorded; approximated with the hand
    // size when the trigger resolves.
    if script::hand_size(state, trig.controller) == 0 {
        vec![Effect::DrawCards { player: trig.controller, count: 1 }]
    } else {
        vec![Effect::DealDamage {
            target: DamageTarget::Player(trig.controller),
            amount: 1,
            source: trig.source,
        }]
    }
}
