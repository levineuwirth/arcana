//! Merfolk of the Depths — `{4}{G/U}{G/U}` 4/2 Merfolk Soldier with Flash.
//! Dissension uncommon; a flash creature that can be cast at instant speed,
//! representing the merfolk ambush tactics in the Simic guild.
//!
//! # Rules references
//!
//! * CR 702.8 — Flash. "You may cast this spell any time you could cast
//!   an instant." Engine checks this during the casting window.
//!
//! Colors: G, U (hybrid cost {G/U}{G/U} — multicolor green+blue per spec).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Merfolk of the Depths");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
