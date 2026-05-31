//! Nissa's Encouragement — `{4}{G}` sorcery. "Search your library and
//! graveyard for a card named Forest, a card named Brambleweft Behemoth,
//! and a card named Nissa, Genesis Mage. Reveal those cards, put them
//! into your hand, then shuffle."
//!
//! Each named card is fetched to hand via `Effect::TutorToHand` filtered
//! by exact name (with `reveal: true`); the engine shuffles after a
//! library search automatically.
//!
//! GAP: the graveyard half of the search ("Search your library AND
//! graveyard") is not expressible — `TutorToHand` searches the library
//! only. The library search by name is emitted faithfully.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa's Encouragement");
    // Intern the three named targets so `resolve` can look them up.
    let _forest = reg.interner_mut().intern("Forest");
    let _behemoth = reg.interner_mut().intern("Brambleweft Behemoth");
    let _nissa = reg.interner_mut().intern("Nissa, Genesis Mage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library and graveyard for a card named Forest, a card named Brambleweft Behemoth, and a card named Nissa, Genesis Mage. Reveal those cards, put them into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest");
    let behemoth = reg.interner().lookup("Brambleweft Behemoth");
    let nissa = reg.interner().lookup("Nissa, Genesis Mage");
    vec![
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter { name: forest, ..ObjectFilter::default() },
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter { name: behemoth, ..ObjectFilter::default() },
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter { name: nissa, ..ObjectFilter::default() },
            reveal: true,
        },
    ]
}
