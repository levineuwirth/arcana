//! Slinking Serpent — `{2}{U}{B}` 2/3 Serpent with Forestwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. This creature can't be blocked as long as
//!   the defending player controls a land of the named type. Here the
//!   type is Forest (Forestwalk). Scryfall also lists a generic
//!   "Landwalk" umbrella entry which is ignored per engine conventions;
//!   only the specific Forestwalk entry maps to
//!   `KeywordAbility::Landwalk`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slinking Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
