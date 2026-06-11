//! Infernal Genesis — `{4}{B}{B}` enchantment.
//! "At the beginning of each player's upkeep, that player mills a card.
//! Then they create X 1/1 black Minion creature tokens, where X is the
//! milled card's mana value."
//!
//! NOTE: "each player's upkeep" is modeled as TWO triggers (whose: You /
//! whose: Opponent); the opponent's identity uses the documented 2-player
//! read via `script::opponents`.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infernal Genesis");
    let _minion = reg.interner_mut().intern("Minion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: genesis_you,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: genesis_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…that player mills a card. Then they create X 1/1 black Minion
/// creature tokens, where X is the milled card's mana value."
fn genesis_for(p: PlayerId) -> Vec<Effect> {
    // GAP: "X is the MILLED card's mana value" — the identity of the card
    // milled by this resolution is not readable, so the dynamic token half
    // is omitted; only the mill is emitted.
    vec![Effect::Mill { player: p, count: 1 }]
}

fn genesis_you(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    genesis_for(trig.controller)
}

fn genesis_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    genesis_for(p)
}
