//! Archangel of Tithes — `{1}{W}{W}{W}` 3/5 Angel with Flying.
//! Two attack/block taxes gated on its own tapped/attacking status.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archangel of Tithes");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As long as ~ is untapped, creatures can't attack you/your PWs
    // unless their controller pays {1} for each" — a static, conditional
    // attack tax; no static-ability primitive in the demonstrated API.
    // GAP: "As long as ~ is attacking, creatures can't block unless their
    // controller pays {1} for each" — likewise a conditional static block tax.

    reg.register(CardDefinition::new(name, chars))
}
