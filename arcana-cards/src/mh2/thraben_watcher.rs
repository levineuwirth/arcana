//! Thraben Watcher — `{2}{W}{W}` 2/2 Angel with Flying and Vigilance.
//! The anthem static (other nontoken creatures you control get +1/+1 and
//! have vigilance) is GAP'd — no static continuous ability in this class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thraben Watcher");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        // GAP: "Other nontoken creatures you control get +1/+1 and have
        // vigilance" is a static continuous ability — not expressible here.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
