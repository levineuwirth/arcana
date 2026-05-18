//! Vernadi Shieldmate — `{1}{G/W}` 2/2 Human Soldier with Vigilance.
//!
//! Colors: G, W (hybrid mana cost; both colors apply per engine
//! conventions).
//!
//! # Rules references
//!
//! * CR 702.20 — Vigilance. Attacking doesn't cause the creature to
//!   tap.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vernadi Shieldmate");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
