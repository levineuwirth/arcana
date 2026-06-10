//! Mirrormind Crown — `{4}` artifact — Equipment.
//! "As long as this Equipment is attached to a creature, the first time you
//! would create one or more tokens each turn, you may instead create that
//! many tokens that are copies of equipped creature. Equip `{2}`."
//! Only the Equip half is expressible; the token-creation replacement
//! effect is a documented gap.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirrormind Crown");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: 'the first time you would create one or more tokens each turn,
    // you may instead create that many tokens that are copies of equipped
    // creature' — a conditional token-creation replacement effect; not
    // expressible with the demonstrated API (no replacement-effect builder
    // for token substitution).
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost")),
    )
}
