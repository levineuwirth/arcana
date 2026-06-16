//! Soulmates — `{2}{G}` enchantment — Aura.
//! "Enchant two creatures
//!  Enchanted creatures each get +1/+1 and have hexproof.
//!  When one of the enchanted creatures dies, destroy the other."
//!
//! "Enchant two creatures" has no multi-attachment form — the engine attaches
//! to a single object. Widened to a single enchanted creature: +1/+1 and
//! hexproof are installed on the one host. The two-creature death-link
//! ("destroy the other") is not expressible — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Soulmates");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "enchant two creatures" widened to a single enchanted creature.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: two-creature death-link ("when one dies, destroy the other") — no
    // multi-attachment / paired-object machinery. +1/+1 and hexproof wired.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(trig.source, 1, 1, Duration::WhileSourceOnBattlefield),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Hexproof,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
