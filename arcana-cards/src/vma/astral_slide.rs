//! Astral Slide — `{2}{W}` enchantment.
//! "Whenever a player cycles a card, you may exile target creature.
//! If you do, return that card to the battlefield under its owner's
//! control at the beginning of the next end step."
//!
//! GAP: there is no dedicated "cycles a card" trigger condition;
//! cycling is the engine-synthesized discard-to-draw activation, so
//! the closest condition is `CardDiscarded { player: Any }` — this
//! over-fires on non-cycling discards. The blink itself is exile +
//! a delayed return at the next end step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Astral Slide");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever a player cycles a card" has no
                // dedicated condition; CardDiscarded(Any) is the closest
                // (cycling pays a discard) but also fires on plain discards.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: slide_out,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// "…you may exile target creature. If you do, return that card to the
/// battlefield … at the beginning of the next end step." ("may" is
/// modeled as always taking the action when a target was chosen.)
fn slide_out(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
