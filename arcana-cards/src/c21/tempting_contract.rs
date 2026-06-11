//! Tempting Contract — `{4}` artifact.
//! "At the beginning of your upkeep, each opponent may create a
//! Treasure token. For each opponent who does, you create a Treasure
//! token."
//!
//! The upkeep trigger mints via `CreateCommodityToken`. GAP: the
//! per-opponent "may" choice is not expressible — modeled
//! deterministically as every opponent accepting (each opponent gets a
//! Treasure, you get one per opponent).

use arcana_core::effects::{CommodityToken, Effect};
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
    let name = reg.interner_mut().intern("Tempting Contract");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
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
                effect: tempt_opponents,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…each opponent may create a Treasure token. For each opponent who
/// does, you create a Treasure token."
fn tempt_opponents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: fidelity — the per-opponent "may" is not expressible; every
    // opponent is assumed to accept.
    let opps = script::opponents(state, trig.controller);
    let mut effects: Vec<Effect> = opps
        .iter()
        .map(|p| Effect::CreateCommodityToken {
            controller: *p,
            kind: CommodityToken::Treasure,
            count: 1,
        })
        .collect();
    effects.push(Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: opps.len() as u32,
    });
    effects
}
