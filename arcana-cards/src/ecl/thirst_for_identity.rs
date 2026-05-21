//! Thirst for Identity — `{2}{U}` instant. "Draw three cards. Then
//! discard two cards unless you discard a creature card."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thirst for Identity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw three cards. Then discard two cards unless you discard a creature card.".into(),
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
    // "Discard two cards unless you discard a creature card" — the
    // engine has no "discard X unless you discard a card of type Y"
    // primitive; we model the always-true clause: draw three, then a
    // controller-chosen discard of two.
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::Discard {
            player: entry.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
    // GAP: "unless you discard a creature card" — a single-creature-
    // card discard substituting for the two-card discard cannot be
    // expressed; the discard count is fixed at two.
}
