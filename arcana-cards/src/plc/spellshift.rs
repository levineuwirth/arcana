//! Spellshift — `{3}{U}` instant, "Counter target instant or sorcery spell.
//! Its controller reveals cards from the top of their library until they
//! reveal an instant or sorcery card. That player may cast that card without
//! paying its mana cost. Then the player shuffles."
//!
//! GAP: "reveal until find" library iteration and "cast without paying mana
//! cost" are not in the Effect catalog. Only the Counter effect is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spellshift");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target instant or sorcery spell. Its controller reveals cards from the top of their library until they reveal an instant or sorcery card. That player may cast that card without paying its mana cost. Then the player shuffles.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(
                        ObjectFilter::new()
                            .without_types(TypeLine::CREATURE.into())
                            .without_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::ENCHANTMENT.into())
                            .without_types(TypeLine::LAND.into())
                            .without_types(TypeLine::PLANESWALKER.into())
                    ),
                    count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "reveal until find instant/sorcery" + "cast without paying mana cost" not in catalog.
    vec![Effect::Counter { target: *id }]
}
