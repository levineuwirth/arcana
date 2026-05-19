//! Unexpected Conversion — `{2}{U}` sorcery. "Draw two cards. Then you may
//! exile an instant or sorcery card from your hand. If you do, search your
//! hand and library for any number of cards with the same name, exile them,
//! then shuffle. Seek an instant or sorcery card for each card exiled from
//! your hand this way."
//!
//! GAP: optional hand-exile with name-based multi-zone exile and Seek
//! mechanic are not expressible with the current Effect catalog. Only
//! the draw is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unexpected Conversion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. Then you may exile an instant or sorcery card from your hand. If you do, search your hand and library for any number of cards with the same name, exile them, then shuffle. Seek an instant or sorcery card for each card exiled from your hand this way.".into(),
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
    // GAP: optional hand exile, name-based multi-zone exile, and Seek mechanic
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
