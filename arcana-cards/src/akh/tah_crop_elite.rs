//! Tah-Crop Elite — `{3}{W}` 2/2 Bird Warrior with Flying.
//! You may exert this creature as it attacks. When you do, creatures you control
//! get +1/+1 until end of turn. (GAP — Exert not modeled)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tah-Crop Elite");
    let bird = reg.interner_mut().intern("Bird");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(warrior);
    // GAP: "You may exert this creature as it attacks. When you do, ..." — Exert is
    // not a modeled mechanic; the optional exert (with its no-untap cost) and the
    // reflexive "when you do" pump are not expressible. Firing the pump on every
    // attack would be materially wrong.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
