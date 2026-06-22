//! Fallen Cleric — `{4}{B}` 4/2 black Zombie Cleric.
//!
//! Oracle:
//! * Protection from Clerics.
//! * Morph {4}{B}.
//!
//! Both printed abilities fall outside the usable keyword/effect surface,
//! so this file carries only the bones.
//!
//! GAP (keyword): "Protection from Clerics" — Protection is not an
//! available `KeywordAbility` variant.
//! GAP (keyword): "Morph {4}{B}" — Morph is not an available
//! `KeywordAbility` variant and the face-down cast mechanic is not part of
//! the documented surface.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fallen Cleric");
    let zombie = reg.interner_mut().intern("Zombie");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
