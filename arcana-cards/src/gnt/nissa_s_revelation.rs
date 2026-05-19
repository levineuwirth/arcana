//! Nissa's Revelation — `{5}{G}{G}` sorcery. "Scry 5, then reveal the top
//! card of your library. If it's a creature card, you draw cards equal to
//! its power and you gain life equal to its toughness."
//!
//! # GAP: conditional draw/life based on revealed card's P/T requires
//! reading game-object state mid-resolution. Best-effort: scry 5 only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa's Revelation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry 5, then reveal the top card of your library. If it's a creature \
                       card, you draw cards equal to its power and you gain life equal to its \
                       toughness."
                    .into(),
                target_requirements: vec![],
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
    // GAP: reveal-top-card + conditional draw/life based on revealed card's P/T
    vec![Effect::Scry { player: entry.controller, count: 5 }]
}
