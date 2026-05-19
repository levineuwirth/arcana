//! Elvish Elegy — `{B/G}` Kindred Sorcery — Elf. "Mill three cards,
//! then each creature card in your graveyard perpetually gets +1/+1.
//! You may put an Elf or land card from among the milled cards into
//! your hand."
//!
//! Type line: Kindred Sorcery — Elf maps to SORCERY (Kindred supertype
//! not in engine; Elf subtype is tribal but TypeLine has no KINDRED
//! const — using SORCERY.into() as closest match).
//!
//! GAP: PerpetualPlusOnePlusOne (perpetually gives +1/+1 to creature
//! cards in graveyard) — no catalog variant.
//! GAP: ChooseFromMilledCards (choose an Elf or land from the milled
//! cards to put into hand) — no catalog variant.
//! The Mill effect is expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elvish Elegy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mill three cards, then each creature card in your graveyard perpetually gets +1/+1. You may put an Elf or land card from among the milled cards into your hand.".into(),
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
    // GAP: PerpetualPlusOnePlusOne (each creature card in graveyard perpetually gets +1/+1)
    // GAP: ChooseFromMilledCards (choose Elf or land from milled cards to hand)
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
