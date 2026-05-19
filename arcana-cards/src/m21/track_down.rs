//! Track Down — `{1}{G}` sorcery. "Scry 3, then reveal the top card of your
//! library. If it's a creature or land card, draw a card."
//! The 'reveal top card and conditionally draw' clause cannot be expressed
//! with the current Effect catalog.
//! GAP: Effect::RevealTopAndConditionalDraw not available.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Track Down");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry 3, then reveal the top card of your library. If it's a creature or land card, draw a card.".into(),
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
    // GAP: reveal-top-then-conditional-draw not in catalog
    vec![Effect::Scry { player: entry.controller, count: 3 }]
}
