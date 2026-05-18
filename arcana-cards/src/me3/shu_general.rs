//! Shu General — `{3}{W}` 2/2 Human Soldier with Vigilance and Horsemanship.
//! Portal Three Kingdoms uncommon; a commanding officer of the Shu kingdom
//! who remains alert while mounted.
//!
//! # Rules references
//!
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature to tap.
//! * CR 702.103 — Horsemanship. This creature can't be blocked except
//!   by creatures with horsemanship.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shu General");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Horsemanship],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
