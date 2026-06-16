//! Beastcaller Savant — `{1}{G}` 1/1 Elf Shaman Ally with Haste.
//! "{T}: Add one mana of any color. Spend this mana only to cast a
//! creature spell."
//!
//! The mana-restriction rider ("spend only to cast a creature spell")
//! is not expressible with the demonstrated mana API, so the activated
//! ability is GAP'd — the engine has no way to tag the produced mana
//! with a spend restriction here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beastcaller Savant");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "{T}: Add one mana of any color. Spend this mana only to cast
    // a creature spell." — the any-color choice plus the spend
    // restriction cannot be expressed; AddMana takes a fixed color list
    // and there is no spend-restriction tag on produced mana.
    reg.register(CardDefinition::new(name, chars))
}
