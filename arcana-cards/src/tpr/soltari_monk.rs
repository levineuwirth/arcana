//! Soltari Monk — `{W}{W}` 2/1 white Soltari Monk Cleric.
//!
//! Protection from black. (GAP — see below.)
//! Shadow.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soltari Monk");
    let soltari = reg.interner_mut().intern("Soltari");
    let monk = reg.interner_mut().intern("Monk");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soltari);
    subtypes.0.insert(monk);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Protection from black" — Protection is not in the usable
        // keyword surface for this card class.
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
