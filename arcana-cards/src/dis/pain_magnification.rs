//! Pain Magnification — `{1}{B}{R}` enchantment.
//! "Whenever an opponent is dealt 3 or more damage by a single source,
//! that player discards a card."

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Pain Magnification");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — DamageDealt has no damage-amount threshold and
                // no damaged-player constraint; "an opponent is dealt 3 or more
                // damage" is gated in the effect fn instead.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: discard_on_big_hit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player discards a card." Gated to opponents dealt 3+ damage by
/// the single triggering source.
fn discard_on_big_hit(
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
    if trig.damage_amount().unwrap_or(0) < 3 {
        return Vec::new();
    }
    vec![Effect::Discard {
        player: p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
