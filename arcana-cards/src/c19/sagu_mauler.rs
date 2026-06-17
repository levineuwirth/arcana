//! Sagu Mauler — {4}{G}{U} 6/6 Creature — Beast.
//! Trample, hexproof.
//! Morph {3}{G}{U} (GAP — Morph is not an expressible KeywordAbility variant
//! and the face-down cast mechanic is unmodeled).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sagu Mauler");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Morph is not supported; only Trample and Hexproof are emitted.
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
