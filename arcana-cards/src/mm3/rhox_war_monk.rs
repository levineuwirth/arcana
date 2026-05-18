//! Rhox War Monk — `{G}{W}{U}` 3/4 Rhino Monk with Lifelink.
//! A three-color (G/U/W) creature from Shards of Alara; a Bant creature
//! combining green, white, and blue mana.
//!
//! # Rules references
//!
//! * CR 702.15 — Lifelink. Damage dealt by this creature also causes its
//!   controller to gain that much life.
//!
//! Lifelink is a base characteristic; the runtime pipeline handles enforcement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rhox War Monk");
    let rhino = reg.interner_mut().intern("Rhino");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
