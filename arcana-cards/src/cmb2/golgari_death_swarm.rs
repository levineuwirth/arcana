//! Golgari Death Swarm — `{3}{B}{G}` 4/4 black-green Bat Fungus Horror with
//! Flying and Vigilance.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature to tap.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golgari Death Swarm");
    let bat = reg.interner_mut().intern("Bat");
    let fungus = reg.interner_mut().intern("Fungus");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(fungus);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
