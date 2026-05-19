//! Test of Talents — `{1}{U}` instant. "Counter target instant or sorcery spell.
//! Search its controller's graveyard, hand, and library for any number of cards
//! with the same name as that spell and exile them. That player shuffles, then
//! draws a card for each card exiled from their hand this way."
//!
//! # GAP: cross-zone name-match exile and conditional draw (per hand exile) are
//! not expressible. The counter portion is implemented; the search/exile/draw
//! rider is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Test of Talents");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target instant or sorcery spell. Search its controller's graveyard, hand, and library for any number of cards with the same name as that spell and exile them. That player shuffles, then draws a card for each card exiled from their hand this way.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(
                        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
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
    // GAP: cross-zone name-match exile + conditional draw (per hand exile) not expressible
    vec![Effect::Counter { target: *id }]
}
