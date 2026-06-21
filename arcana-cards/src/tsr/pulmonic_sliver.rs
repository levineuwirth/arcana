//! Pulmonic Sliver — `{3}{W}{W}` 3/3 Creature — Sliver.
//! "All Sliver creatures have flying.
//!  All Slivers have 'If this permanent would be put into a graveyard, you may
//!  put it on top of its owner's library instead.'"

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pulmonic Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    // GAP (static): "All Sliver creatures have flying" — a tribe-wide keyword-
    // granting static; no documented static-keyword-grant hook for this class.
    // GAP (static): "All Slivers have 'If this permanent would be put into a
    // graveyard, you may put it on top of its owner's library instead.'" — a
    // tribe-wide granted replacement ability; not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
