//! Unexpected Conversion — `{2}{U}` sorcery. "Draw two cards. Then you
//! may exile an instant or sorcery card from your hand. If you do,
//! search your hand and library for any number of cards with the same
//! name, exile them, then shuffle. Seek an instant or sorcery card for
//! each card exiled from your hand this way."
//!
//! The "draw two cards" half is expressible. The optional exile-from-
//! hand, same-name search across hand+library, and Seek (random tutor)
//! rider have no engine primitives.

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
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
    // GAP: optional exile-from-hand of an instant/sorcery, same-name
    // search across hand and library, and Seek (random matching tutor)
    // are not expressible with the available effect catalog.
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
