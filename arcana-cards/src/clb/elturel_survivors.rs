//! Elturel Survivors — `{3}{R}` 0/4 Tiefling Peasant.
//! Trample. (Myriad: GAP — not a usable KeywordAbility variant.)
//! As long as this creature is attacking, it gets +X/+0 where X is the number
//! of lands defending player controls. (static: GAP.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elturel Survivors");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(peasant);

    // GAP: Myriad keyword is not an available KeywordAbility variant.
    // GAP: "As long as this creature is attacking, it gets +X/+0 ..." is a
    //      conditional continuous static, not a triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
