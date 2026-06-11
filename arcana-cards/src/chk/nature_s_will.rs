//! Nature's Will — `{2}{G}{G}` enchantment.
//! "Whenever one or more creatures you control deal combat damage to a
//! player, tap all lands that player controls and untap all lands you
//! control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nature's Will");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "one or more creatures you control deal
                // combat damage" is a BATCHED trigger; DamageDealt fires once
                // per damaging creature instead. Tapping/untapping is
                // idempotent, so repeat firings are harmless.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: tap_theirs_untap_yours,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…tap all lands that player controls and untap all lands you control."
fn tap_theirs_untap_yours(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    let lands_you = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let theirs = script::ids_matching(state, &lands_you, p);
    let lands_mine = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let mine = script::ids_matching(state, &lands_mine, trig.controller);
    let mut effects = Vec::new();
    if !theirs.is_empty() {
        effects.push(Effect::ForEach {
            targets: theirs,
            effect: Box::new(Effect::Tap {
                target: NULL_OBJECT_ID,
            }),
        });
    }
    if !mine.is_empty() {
        effects.push(Effect::ForEach {
            targets: mine,
            effect: Box::new(Effect::Untap {
                target: NULL_OBJECT_ID,
            }),
        });
    }
    effects
}
