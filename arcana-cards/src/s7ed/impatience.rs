//! Impatience — `{2}{R}` enchantment.
//! "At the beginning of each player's end step, if that player didn't
//! cast a spell this turn, this enchantment deals 2 damage to that
//! player."
//!
//! Split into TWO StepBegins triggers (yours / your opponent's end step)
//! so "that player" is identifiable; each carries the intervening-if
//! "didn't cast a spell this turn" for its own player. The opponent read
//! uses the documented 2-player `script::opponents` accessor.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Impatience");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                intervening_if: Some(if_you_cast_no_spell),
                effect: zap_you,
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
                intervening_if: Some(if_opponent_cast_no_spell),
                effect: zap_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Intervening-if for your own end step: you cast no spell this turn.
fn if_you_cast_no_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::spells_cast_this_turn(s, &ObjectFilter::new(), you) == 0
}

/// Intervening-if for an opponent's end step: that player cast no spell
/// this turn (documented 2-player read).
fn if_opponent_cast_no_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::opponents(s, you)
        .first()
        .map(|&p| script::spells_cast_this_turn(s, &ObjectFilter::new(), p) == 0)
        .unwrap_or(false)
}

/// "…this enchantment deals 2 damage to that player" — your end step.
fn zap_you(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DealDamage {
        target: DamageTarget::Player(trig.controller),
        amount: 2,
        source: trig.source,
    }]
}

/// "…this enchantment deals 2 damage to that player" — opponent's end
/// step (2-player read).
fn zap_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(&p) = script::opponents(state, trig.controller).first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(p),
        amount: 2,
        source: trig.source,
    }]
}
