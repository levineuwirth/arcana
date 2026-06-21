//! Ballynock Cohort — `{2}{W}` 2/2 Kithkin Soldier with First strike.
//!
//! Oracle:
//! * First strike
//! * This creature gets +1/+1 as long as you control another white
//!   creature.  (conditional self static — GAP)
//!
//! Only First strike is expressible; the conditional self-anthem is a
//! static continuous ability with no triggered/activated representation.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ballynock Cohort");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: "+1/+1 as long as you control another white creature." —
    // conditional self static, not a triggered/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
