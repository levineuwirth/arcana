//! Lost Legacy — `{1}{B}{B}` sorcery. "Choose a nonartifact, nonland card
//! name. Search target player's graveyard, hand, and library for any number
//! of cards with that name and exile them. That player shuffles, then draws a
//! card for each card exiled from their hand this way."
//!
//! Implemented with [`Effect::NameCardAndExile`]: the engine deterministically
//! picks the most-copied card name in the target player's hand + graveyard +
//! library, exiles every copy across those zones, and shuffles. The
//! card-name choice prompt and the nonartifact/nonland constraint are an
//! engine-side approximation (no name-choice prompt is wired); the disruption
//! itself is faithful.
//!
//! GAP: "draws a card for each card exiled from their hand this way" — the
//! number of copies removed specifically from hand is not surfaced as a
//! follow-up amount, so the rider draw is not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost Legacy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose a nonartifact, nonland card name. Search target \
                   player's graveyard, hand, and library for any number of \
                   cards with that name and exile them. That player shuffles, \
                   then draws a card for each card exiled from their hand this \
                   way."
                .into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::NameCardAndExile {
        chooser: entry.controller,
        target: *p,
    }]
}
