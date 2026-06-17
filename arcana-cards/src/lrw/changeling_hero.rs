//! Changeling Hero — `{4}{W}` 4/4 white Shapeshifter with Lifelink and
//! Changeling.
//!
//! Oracle:
//! * Changeling — every creature type (keyword).
//! * Champion a creature — NOT expressible. Champion is not a supported
//!   `KeywordAbility`, and the two-part mechanic (ETB sacrifice-unless-exile,
//!   leaves-the-battlefield return) needs machinery the catalog can't author.
//! * Lifelink (keyword).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Changeling Hero");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink, KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: "Champion a creature" — Champion is not a supported keyword and the
    // sacrifice-unless-exile / leaves-return mechanic is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
