//! Iroas's Champion — `{1}{R}{W}` 2/2 Human Soldier with Double Strike.
//! Born of the Gods uncommon; a red-white human soldier champion of Iroas,
//! god of victory, dealing both first-strike and regular combat damage.
//!
//! # Rules references
//!
//! * CR 702.4 — Double Strike. This creature deals both first-strike and
//!   regular combat damage. Engine handles this in the two-step combat
//!   damage process.
//!
//! Colors: R, W.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iroas's Champion");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
