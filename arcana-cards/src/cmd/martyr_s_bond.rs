//! Martyr's Bond — `{4}{W}{W}` enchantment (Commander 2011).
//! "Whenever this enchantment or another nonland permanent you control is
//! put into a graveyard from the battlefield, each opponent sacrifices a
//! permanent of their choice that shares a card type with it."
//!
//! A dies-bound `ZoneChange` trigger over nonland permanents you control
//! (this enchantment matches its own filter). GAP: the "shares a card type
//! with it" constraint on the sacrifice is a dynamic type comparison no
//! `ObjectFilter` can express — each opponent sacrifices any permanent
//! (over-broad).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Martyr's Bond");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: opponents_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…each opponent sacrifices a permanent of their choice that shares a
/// card type with it."
fn opponents_sacrifice(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "shares a card type with it" — a dynamic type comparison against
    // the dying permanent that no ObjectFilter predicate expresses; each
    // opponent sacrifices any permanent of their choice (over-broad).
    let effects = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::permanent(),
            count: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
