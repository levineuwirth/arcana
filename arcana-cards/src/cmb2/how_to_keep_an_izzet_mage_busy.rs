//! How to Keep an Izzet Mage Busy — `{U/R}` sorcery. "Return How to Keep an
//! Izzet Mage Busy to its owner's hand." Self-bounce — we know the source id
//! is `entry.source` so we can ReturnToHand it directly.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("How to Keep an Izzet Mage Busy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U/R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return How to Keep an Izzet Mage Busy to its owner's hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Self-bounce: the spell on resolution returns its own stack-id "card" to hand.
    vec![Effect::ReturnToHand {
        target: entry.source,
    }]
}
