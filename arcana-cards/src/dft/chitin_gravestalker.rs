//! Chitin Gravestalker — `{5}{B}` 5/4 Insect Warrior.
//! "This spell costs {1} less to cast for each artifact and/or creature
//!  card in your graveyard."
//! "Cycling {2} ({2}, Discard this card: Draw a card.)"
//!
//! Cycling {2} is wired as the parametrized keyword (the engine
//! synthesizes the discard-to-draw activation). GAP: the "costs {1}
//! less for each artifact/creature card in your graveyard" cast-cost
//! reduction has no engine surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chitin Gravestalker");
    let insect = reg.interner_mut().intern("Insect");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(warrior);

    // GAP: "This spell costs {1} less to cast for each artifact and/or
    // creature card in your graveyard." — no cast-cost-reduction surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
