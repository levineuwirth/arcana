//! Rabbit Battery — `{R}` 1/1 Artifact Creature — Equipment Rabbit with Haste.
//! "Equipped creature gets +1/+1 and has haste." (Equipment static — GAP).
//! "Reconfigure {R}" — Reconfigure is not in the usable keyword surface (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rabbit Battery");
    let equipment = reg.interner_mut().intern("Equipment");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(rabbit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Equipped creature gets +1/+1 and has haste" — Equipment static buff
    // (depends on an attached host); not expressible here.
    // GAP: "Reconfigure {R}" — Reconfigure is not in the usable keyword surface.
    reg.register(CardDefinition::new(name, chars))
}
