//! Windstorm Drake — `{4}{U}` 3/3 Drake with Flying.
//!
//! Oracle:
//! * Flying — keyword, base characteristic.
//! * "Other creatures you control with flying get +1/+0." — GAP: a static
//!   continuous anthem (no trigger word, no cost); not expressible as a
//!   triggered or activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windstorm Drake");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static anthem "Other creatures you control with flying get +1/+0."
    reg.register(CardDefinition::new(name, chars))
}
