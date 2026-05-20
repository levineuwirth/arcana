//! Dina's Guidance — `{1}{B}{G}` instant. "Search your library for a
//! creature card, reveal it, put it into your hand or graveyard, then
//! shuffle." Modeled as a tutor-to-hand of a creature card (the
//! hand-or-graveyard player choice has no primitive — the hand branch
//! is taken; reveal is implicit).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dina's Guidance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a creature card, reveal it, put it into your hand or graveyard, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // "put it into your hand or graveyard" — the to-graveyard branch
    // has no tutor primitive; the hand branch is modeled.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        reveal: true,
    }]
}
