//! Artificer's Epiphany — `{2}{U}` instant. "Draw two cards. If you control
//! no artifacts, discard a card."
//!
//! # GAP: "if you control no artifacts" conditional discard requires
//! battlefield state check not supported by the catalog's Conditional
//! variant (no artifact-count condition type shown). Best-effort: draw 2,
//! no discard.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artificer's Epiphany");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. If you control no artifacts, discard a card.".into(),
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
    // GAP: conditional discard ("if you control no artifacts") not expressible
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
