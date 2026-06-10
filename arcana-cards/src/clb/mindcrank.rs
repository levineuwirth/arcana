//! Mindcrank — `{2}` artifact (New Phyrexia, 2011).
//! "Whenever an opponent loses life, that player mills that many
//! cards. (Damage causes loss of life.)"
//!
//! There is no life-lost TriggerCondition; the closest available event
//! that carries both the player and the amount is `DamageDealt` to a
//! player, which covers the dominant (damage-caused) life-loss case.
//! Non-damage life loss (e.g. pure LoseLife effects) does not fire
//! this implementation — documented GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mindcrank");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever an opponent loses life" has
                // no TriggerCondition; DamageDealt-to-a-player is the
                // closest event carrying player + amount (damage causes
                // loss of life). Non-damage life loss is not covered.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: mill_on_life_loss,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player mills that many cards."
fn mill_on_life_loss(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    // Only an OPPONENT losing life fires the printed ability.
    if !script::opponents(state, trig.controller).contains(&p) {
        return Vec::new();
    }
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Mill { player: p, count: n }]
}
