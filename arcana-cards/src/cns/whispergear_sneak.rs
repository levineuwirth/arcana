//! Whispergear Sneak — `{1}` 1/1 Artifact Creature — Construct.
//!
//! Oracle:
//! * Draft this card face up. (GAP'd — draft-format mechanic.)
//! * During the draft, you may turn this card face down. If you do, look
//!   at any unopened booster pack ... (GAP'd — draft-format mechanic.)
//!
//! Both abilities operate entirely during the draft, outside the game's
//! turn structure, with no in-game effect; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Whispergear Sneak");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Draft this card face up" / face-down draft peeking — draft-
    // format mechanics with no in-game representation.

    reg.register(CardDefinition::new(name, chars))
}
