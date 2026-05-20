//! Herald's Reveille — `{U}` sorcery. "Draw a card. If a permanent
//! you controlled explored this turn, seek a Merfolk card instead."
//! The explore-history condition and the seek mechanic are not
//! modeled; we draw a card (the base, non-conditional effect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Herald's Reveille");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw a card. If a permanent you controlled explored this turn, seek a Merfolk card instead.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: explore-history condition and the seek mechanic are not
    // modeled. The base "draw a card" is emitted.
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
