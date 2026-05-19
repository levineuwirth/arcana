//! Brilliant Spectrum — `{3}{U}` sorcery, "Converge — Draw X cards, where X
//! is the number of colors of mana spent to cast this spell. Then discard two
//! cards."
//!
//! GAP: Converge (X = colors of mana spent to cast) is not expressible.
//! Partial: draw 1 card, discard 2 cards.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brilliant Spectrum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Converge — Draw X cards, where X is the number of colors of mana spent to cast this spell. Then discard two cards.".into(),
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
    // GAP: Converge — X = number of colors of mana spent to cast
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::Discard { player: entry.controller, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}
