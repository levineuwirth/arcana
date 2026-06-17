//! Blazethorn Scarecrow — `{5}` 3/3 colorless Artifact Creature —
//! Scarecrow.
//! "This creature has haste as long as you control a red creature."
//! "This creature has wither as long as you control a green creature."
//!
//! Both lines are conditional STATIC continuous abilities (no trigger
//! word, no cost). The provided catalog has no way to express a
//! conditional self-keyword static, so both are GAP'd and only the bones
//! are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blazethorn Scarecrow");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    // GAP: static "has haste as long as you control a red creature".
    // GAP: static "has wither as long as you control a green creature".
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
