//! Cogwork Grinder — `{6}` 0/0 Artifact Creature — Construct.
//! Draft this card face up. As you draft a card, you may remove it from the
//!   draft face down. (draft-time mechanics: GAP.)
//! This creature enters with X +1/+1 counters, where X is the number of cards
//!   removed from the draft named Cogwork Grinder. (draft state unmodeled: GAP.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cogwork Grinder");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    // GAP: "Draft this card face up" and "As you draft a card, you may remove
    //      it ..." are draft-time mechanics with no engine representation.
    // GAP: "enters with X +1/+1 counters where X = cards removed from the draft
    //      named Cogwork Grinder" — draft state is unmodeled, so X is
    //      uncomputable.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
