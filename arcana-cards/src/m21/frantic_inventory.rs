//! Frantic Inventory — `{1}{U}` instant. "Draw a card, then draw cards
//! equal to the number of cards named Frantic Inventory in your
//! graveyard."
//!
//! The second draw count is dynamic (a graveyard count by name), so it
//! is computed at resolution with `script::graveyard_matching` over an
//! `ObjectFilter` constrained by the card's interned name.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frantic Inventory");
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
                text: "Draw a card, then draw cards equal to the number of cards named Frantic Inventory in your graveyard.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Frantic Inventory");
    let filter = ObjectFilter { name: nm, ..ObjectFilter::default() };
    let extra = script::graveyard_matching(state, &filter, entry.controller, entry.controller);
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::DrawCards { player: entry.controller, count: extra },
    ]
}
