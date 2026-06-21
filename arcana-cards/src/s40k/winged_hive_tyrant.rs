//! Winged Hive Tyrant — `{3}{U}{R}` 4/4 Tyranid with Flying and Haste.
//!
//! Oracle:
//! * Flying, haste — keyword line.
//! * The Will of the Hive Mind — Other creatures you control with
//!   counters on them have flying and haste. (A static continuous
//!   ability granting keywords to a filtered board — not a triggered or
//!   activated ability, so it is GAP'd here.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Winged Hive Tyrant");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "The Will of the Hive Mind — Other creatures you control with
    // counters on them have flying and haste." is a static continuous
    // ability (filtered keyword grant), not a triggered/activated
    // ability — not expressible in this card class.
    reg.register(CardDefinition::new(name, chars))
}
