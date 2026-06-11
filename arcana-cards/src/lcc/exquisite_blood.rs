//! Exquisite Blood — `{4}{B}` enchantment.
//! "Whenever an opponent loses life, you gain that much life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exquisite Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — oracle is "whenever an opponent LOSES
                // LIFE"; no life-lost condition exists. Closest is damage
                // dealt to a player (damage causes life loss per CR 120.3,
                // the dominant case); pure non-damage life loss does not
                // fire this. The opponent restriction is enforced in the
                // effect fn.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: gain_that_much,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain that much life." — only when the damaged (life-losing)
/// player is an opponent.
fn gain_that_much(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    if p == trig.controller {
        return Vec::new();
    }
    let amount = trig.damage_amount().unwrap_or(0);
    if amount == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife {
        player: trig.controller,
        amount,
    }]
}
