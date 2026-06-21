//! Indomitable Archangel — `{2}{W}{W}` 4/4 Angel.
//! Flying.
//! Metalcraft — Artifacts you control have shroud as long as you control
//! three or more artifacts. (GAP — a conditional static that grants
//! shroud to all artifacts you control; no static keyword-grant primitive
//! in this card class. Metalcraft is not a usable KeywordAbility variant.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Indomitable Archangel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Metalcraft — Artifacts you control have shroud as long as you
    // control three or more artifacts" — conditional static that grants a
    // keyword to a set of other permanents; no such static-grant primitive.
    reg.register(CardDefinition::new(name, chars))
}
