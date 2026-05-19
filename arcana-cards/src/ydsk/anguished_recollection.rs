//! Anguished Recollection — `{1}{R}` sorcery. "Discard a card. If you do, seek two cards
//! that don't share a card type with the discarded card."
//!
//! # GAP: Seek mechanic (random card search filtered by type exclusion) not in engine catalog.
//! # GAP: "if you do" conditional on a discard action not in engine catalog.
//! Partial: Discard 1 card (controller chooses) only.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anguished Recollection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard a card. If you do, seek two cards that don't share a card type with the discarded card.".into(),
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
    // GAP: Seek mechanic not in engine catalog
    // GAP: conditional on discard success not in engine catalog
    vec![Effect::Discard {
        player: entry.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
