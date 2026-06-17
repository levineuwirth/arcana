//! Skizzik Surger — `{4}{R}{R}` 6/4 red Elemental.
//! Haste.
//! Echo—Sacrifice two lands. (At the beginning of your upkeep, if this came
//! under your control since the beginning of your last upkeep, sacrifice it
//! unless you pay its echo cost.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skizzik Surger");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        // GAP: Echo (non-mana echo cost "Sacrifice two lands") — not expressible
        // as a keyword variant; the echo upkeep-tax mechanic is not modeled.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
