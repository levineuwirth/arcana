//! Relive the Past — `{5}{G}{W}` sorcery. "Return up to one target artifact card, up to one
//! target land card, and up to one target non-Aura enchantment card from your graveyard to the
//! battlefield. They are 5/5 Elemental creatures in addition to their other types."
//!
//! GAP: Post-return type-change (become 5/5 Elementals in addition to other types) not in catalog;
//! multi-filter graveyard targeting (artifact OR land OR non-Aura enchantment) not expressible.
//! Emitting ReturnFromGraveyardToBattlefield for each of up to 3 targets as best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Relive the Past");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to one target artifact card, up to one target land card, and up to one target non-Aura enchantment card from your graveyard to the battlefield. They are 5/5 Elemental creatures in addition to their other types.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: post-return type-change to 5/5 Elemental creature in addition to other types
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
        } else {
            None
        }
    }).collect()
}
