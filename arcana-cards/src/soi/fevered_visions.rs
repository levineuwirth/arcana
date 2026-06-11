//! Fevered Visions — `{1}{U}{R}` enchantment.
//! "At the beginning of each player's end step, that player draws a
//! card. If the player is your opponent and has four or more cards in
//! hand, this enchantment deals 2 damage to that player."
//!
//! "Each player's end step" is split into two StepBegins triggers (You
//! / Opponent) because no accessor identifies whose step fired; the
//! opponent in the 2-player documented read is
//! `script::opponents(...).first()`.

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
    let name = reg.interner_mut().intern("Fevered Visions");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
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
                effect: you_draw,
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
                effect: opponent_draws_and_burns,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Your end step: you draw a card (no damage branch for yourself).
fn you_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

/// Opponent's end step: they draw; if they'll have 4+ cards in hand,
/// this deals 2 damage to them.
fn opponent_draws_and_burns(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(opp) = script::opponents(state, trig.controller).first().copied()
    else {
        return Vec::new();
    };
    let mut effects = vec![Effect::DrawCards { player: opp, count: 1 }];
    // The oracle checks hand size after the draw resolves; the pending
    // draw above hasn't applied yet, so 3+ now means 4+ after drawing.
    if script::hand_size(state, opp) >= 3 {
        effects.push(Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 2,
            source: trig.source,
        });
    }
    effects
}
