//! Invigorating Boon — `{1}{G}` enchantment (Mirage, 1996).
//! "Whenever a player cycles a card, you may put a +1/+1 counter on target
//! creature."
//!
//! There is no cycling-specific trigger condition; the closest is
//! `CardDiscarded` (cycling discards the card) — documented GAP since it
//! also fires on ordinary discards. The "you may" is resolved as a yes.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invigorating Boon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever a player CYCLES a card"; no
                // cycling trigger condition exists, CardDiscarded is the
                // closest (cycling pays a discard) but also fires on plain
                // discards.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: boon_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// "…you may put a +1/+1 counter on target creature."
fn boon_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
