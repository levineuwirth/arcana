//! Flamespeaker's Will — `{R}` enchantment — Aura.
//! "Enchant creature you control. Enchanted creature gets +1/+1. Whenever
//!  enchanted creature deals combat damage to a player, you may sacrifice
//!  this Aura. If you do, destroy target artifact."
//!
//! Buff Aura: the +1/+1 is an ETB-installed `attached_pt`. The combat-damage
//! host trigger is keyed on "deals COMBAT damage to a player", which is not a
//! `Self*` condition wrappable by AttachedCreatureDoes — GAP that trigger.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Flamespeaker's Will");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    // NOTE: controller wording approximated by caster's choice
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: combat-damage-to-a-player host trigger (not a Self* condition).
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(trig.source, 1, 1, Duration::WhileSourceOnBattlefield),
    }]
}
