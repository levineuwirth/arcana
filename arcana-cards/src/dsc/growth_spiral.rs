//! Growth Spiral — `{G}{U}` instant. "Draw a card. You may put a land card from
//! your hand onto the battlefield."
//!
//! GAP: "you may put a land card from your hand onto the battlefield" (optional
//! hand-to-battlefield for land cards) is not expressible with the catalog's
//! Effect variants (no Effect::PutLandFromHandOntoBattlefield).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Growth Spiral");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card. You may put a land card from your hand onto the battlefield.".into(),
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
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        // GAP: optional put land from hand onto battlefield not expressible
    ]
}
