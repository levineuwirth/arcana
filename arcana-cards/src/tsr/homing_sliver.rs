//! Homing Sliver — `{2}{R}` 2/2 Sliver.
//! "Each Sliver card in each player's hand has slivercycling {3}."
//! "Slivercycling {3} ({3}, Discard this card: Search your library for
//!  a Sliver card, reveal it, put it into your hand, then shuffle.)"
//!
//! Slivercycling is a typecycling variant — per convention it is emitted
//! as the generic Cycling keyword with its printed {3} cost (the engine
//! synthesizes the discard-to-draw activation; the search-for-Sliver
//! type variant is not separately modeled). GAP: the static granting
//! "each Sliver card in each player's hand has slivercycling {3}" is a
//! continuous hand-affecting ability with no engine surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Homing Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    // GAP: static "Each Sliver card in each player's hand has
    // slivercycling {3}." — continuous hand-wide grant, no surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{3}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
