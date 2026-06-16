//! Darksteel Mutation — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature is an Insect artifact creature with
//!  base power and toughness 0/1 and has indestructible, and it loses all other
//!  abilities, card types, and creature types."
//!
//! ETB-installs `attached_set_pt(0/1)` + `attached_types(Artifact)` +
//! `attached_subtypes(Insect)` + `attached_keyword(Indestructible)`. The
//! "loses all other abilities/card types/creature types" stripping is not
//! fully expressible with the demonstrated additive builders — noted.

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
    let name = reg.interner_mut().intern("Darksteel Mutation");
    let aura = reg.interner_mut().intern("Aura");
    let _insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // NOTE: "loses all other abilities, card types, and creature types" is not
    // expressible with additive attached_* builders — only the grants are set.
    let mut insect = SubtypeSet::default();
    if let Some(i) = reg.interner().lookup("Insect") {
        insect.0.insert(i);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                0,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_types(
                trig.source,
                TypeLine::ARTIFACT.into(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                insect,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Indestructible,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
