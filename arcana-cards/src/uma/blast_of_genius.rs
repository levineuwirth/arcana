//! Blast of Genius — `{4}{U}{R}` sorcery. "Choose any target. Draw
//! three cards, then discard a card. Blast of Genius deals damage
//! equal to the discarded card's mana value to that permanent or
//! player."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blast of Genius");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose any target. Draw three cards, then discard a card. Blast of Genius deals damage equal to the discarded card's mana value to that permanent or player.".into(),
            target_requirements: vec![TargetRequirement::any_target()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // The damage amount equals the discarded card's mana value, which the
    // catalog cannot read; only the draw and discard are emitted.
    // GAP: damage equal to the discarded card's mana value.
    vec![
        Effect::DrawCards {
            player: entry.controller,
            count: 3,
        },
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
