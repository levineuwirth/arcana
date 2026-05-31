//! Feral Encounter — `{G}{G}` sorcery. "Look at the top five cards of your
//! library. You may exile a creature card from among them. Put the rest on the
//! bottom of your library in a random order. You may cast the exiled card this
//! turn. At the beginning of the next combat phase this turn, target creature
//! you control deals damage equal to its power to up to one target creature you
//! don't control."
//!
//! Best-effort: the first sentence ("look at the top five, you may put a
//! creature card into your hand, rest on the bottom in a random order") is
//! modeled with `Effect::DigTopN`. The faithful card EXILES the chosen card and
//! lets you CAST it this turn (impulse exile-and-play), not put it into hand —
//! that impulse path is not exposed in this card class's effect catalog, so the
//! dig-to-hand is an approximation. The delayed combat-phase
//! "deals damage equal to its power" rider (a scheduled, double-targeted fight)
//! is not expressible with the catalog's single-target spell shape.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feral Encounter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Look at the top five cards of your library. You may exile a \
                   creature card from among them. Put the rest on the bottom of \
                   your library in a random order. You may cast the exiled card \
                   this turn. At the beginning of the next combat phase this \
                   turn, target creature you control deals damage equal to its \
                   power to up to one target creature you don't control."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: impulse exile-and-cast of the chosen creature card, and the delayed
    // next-combat-phase "creature you control deals damage equal to its power to
    // up to one creature you don't control" rider, are not expressible in this
    // single-effect spell catalog. Approximate the dig with a to-hand pick.
    vec![Effect::DigTopN {
        player: entry.controller,
        count: 5,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
