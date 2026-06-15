//! Gift of the Woods — `{G}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature blocks or becomes blocked,
//!  it gets +0/+3 until end of turn and you gain 1 life."
//!
//! Two host triggers — one for SelfBlocks, one for SelfBecomesBlocked — each
//! pumps the host +0/+3 until end of turn and gains the Aura's controller
//! 1 life. No ETB grant install needed.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gift of the Woods");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBlocks),
                },
                intervening_if: None,
                effect: on_block_or_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesBlocked),
                },
                intervening_if: None,
                effect: on_block_or_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_block_or_blocked(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: host,
            power: 0,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}
