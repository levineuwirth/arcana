//! Become Brutes — `{1}{R}` sorcery. "One or two target creatures each
//! gain haste until end of turn. For each of those creatures, create a
//! Monster Role token attached to it."
//!
//! # GAP
//! - "Role token" (Aura enchantment token with attached bonus) is not
//!   expressible via `TokenDefinition`.
//! - "Double team" (Scryfall keyword) listed in prompt but not in
//!   supported keyword surface; omitted.
//! Haste grant to up to two targets is modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Become Brutes");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "One or two target creatures each gain haste until end of turn. For each of those creatures, create a Monster Role token attached to it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
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
    // GAP: Monster Role token (Aura enchantment token) not expressible via TokenDefinition
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::GrantKeyword {
                    target: *id,
                    keyword: KeywordAbility::Haste,
                    duration: Duration::EndOfTurn,
                })
            } else {
                None
            }
        })
        .collect()
}
