//! Molimo, Maro-Sorcerer — `{4}{G}{G}{G}` */* Legendary Elemental
//! Sorcerer with Trample. Molimo's power and toughness are each equal
//! to the number of lands you control (CDA — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Molimo, Maro-Sorcerer");
    let elemental = reg.interner_mut().intern("Elemental");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(sorcerer);

    // GAP: "*/* — power and toughness each equal to the number of lands
    // you control" is a characteristic-defining static; only
    // PtValue::Fixed is expressible, so the base P/T is stubbed at 0/0.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
