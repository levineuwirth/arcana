//! Squawkroaster — `{3}{R}` */4 Elemental. Double strike.
//! "Vivid — Squawkroaster's power is equal to the number of colors among permanents you
//! control." (Power is `*`; the characteristic-defining ability that sets it is GAP'd —
//! no static CDA primitive in the shown surface.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Squawkroaster");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: "Squawkroaster's power is equal to the number of colors among permanents you
    // control." — characteristic-defining ability (dynamic power) has no static primitive.
    reg.register(CardDefinition::new(name, chars))
}
