//! Elvish Elegy — `{B/G}` Kindred Sorcery — Elf. "Mill three cards,
//! then each creature card in your graveyard perpetually gets +1/+1.
//! You may put an Elf or land card from among the milled cards into
//! your hand." Perpetual buffs and 'reach into milled cards' aren't
//! catalog primitives. We model only the mill.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elvish Elegy");
    let _elf = reg.interner_mut().intern("Elf");
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
    // GAP: 'perpetually +1/+1' on graveyard creature cards and 'pick
    // from milled cards to put in hand' have no catalog primitives.
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
