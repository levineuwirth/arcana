//! False Memories — `{1}{U}` instant. "Mill seven cards. At the beginning
//! of the next end step, exile seven cards from your graveyard."
//!
//! # GAP: delayed triggered ability (at beginning of next end step)
//! The engine has no `TriggeredAbilityDef` trigger for "at the beginning
//! of the next end step" that fires from a spell resolution rather than
//! from a permanent on the battlefield. The mill resolves; the exile
//! rider is not expressible and is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("False Memories");
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
                text: "Mill seven cards. At the beginning of the next end step, exile seven cards from your graveyard.".into(),
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
    // GAP: delayed triggered ability (exile 7 from graveyard at next end step) not expressible
    vec![
        Effect::Mill { player: entry.controller, count: 7 },
    ]
}
