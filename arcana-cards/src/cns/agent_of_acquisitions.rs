//! Agent of Acquisitions — `{2}` 2/1 Artifact Creature — Construct.
//!
//! * Draft this card face up.
//! * Instead of drafting a card from a booster pack, you may draft each card in
//!   that booster pack, one at a time. (Draft-environment ability.)
//!
//! Its entire rules text governs the draft, not gameplay on the battlefield;
//! there are no triggered/activated/static gameplay abilities to express.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Agent of Acquisitions");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: all rules text is draft-environment behavior (drafting cards from
        // booster packs), which has no on-battlefield ability representation.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
