//! Talara's Battalion — `{1}{G}` 4/3 Elf Warrior with Trample.
//! "Cast this spell only if you've cast another green spell this turn."
//!
//! The cast restriction is a casting-legality condition that this card
//! class has no API to express (no cast-condition field on Characteristics
//! or CardDefinition). Only the keyword line (Trample) is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talara's Battalion");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        // GAP: "Cast this spell only if you've cast another green spell this
        // turn" — a cast-legality restriction with no expressible field.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
