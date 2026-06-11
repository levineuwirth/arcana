//! Monsoon — `{2}{R}{G}` enchantment.
//! "At the beginning of each player's end step, tap all untapped Islands
//! that player controls and this enchantment deals X damage to the player,
//! where X is the number of Islands tapped this way."
//!
//! NOTE: "each player's end step" is modeled as TWO triggers (whose: You /
//! whose: Opponent); the opponent's identity uses the documented 2-player
//! read via `script::opponents`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monsoon");
    let _island = reg.interner_mut().intern("Island");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: monsoon_you,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: monsoon_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn monsoon_you(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    monsoon_for(state, trig, reg, trig.controller)
}

fn monsoon_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    monsoon_for(state, trig, reg, p)
}

/// "…tap all untapped Islands that player controls and this enchantment
/// deals X damage to the player, where X is the number of Islands tapped
/// this way."
fn monsoon_for(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
    p: PlayerId,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Island")
        .controlled_by(ControllerConstraint::You)
        .untapped_only();
    let ids = script::ids_matching(state, &filter, p);
    let n = ids.len() as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Tap {
                target: NULL_OBJECT_ID,
            }),
        },
        Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: n,
            source: trig.source,
        },
    ]
}
