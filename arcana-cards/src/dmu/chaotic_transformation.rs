//! Chaotic Transformation — `{5}{R}` sorcery. "Exile up to one target
//! artifact, up to one target creature, up to one target enchantment,
//! up to one target planeswalker, and/or up to one target land. For each
//! permanent exiled this way, its controller reveals cards from the top
//! of their library until they reveal a card that shares a card type
//! with it, puts that card onto the battlefield, then shuffles."
//!
//! The exile portion is expressed as five up-to-one targets, one per
//! card type, exiling each chosen permanent. The reveal-until-shared-
//! card-type / put-onto-battlefield / shuffle rider is per-exiled-
//! permanent and keyed on the exiled object's own card types, which the
//! engine cannot derive from a known id at resolution — see GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaotic Transformation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile up to one target artifact, up to one target creature, up to one target enchantment, up to one target planeswalker, and/or up to one target land. For each permanent exiled this way, its controller reveals cards from the top of their library until they reveal a card that shares a card type with it, puts that card onto the battlefield, then shuffles.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Exile each chosen permanent.
    let mut effects: Vec<Effect> = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ExilePermanent { target: *id });
        }
    }
    // GAP: the per-exiled-permanent rider — each exiled card's controller
    // reveals from the top of their library until revealing a card that
    // shares a card type with the exiled card, puts it onto the
    // battlefield, then shuffles. RevealUntil cannot be keyed on the
    // exiled object's own (dynamic) card types, and there is no primitive
    // to derive the per-object type filter at resolution.
    effects
}
