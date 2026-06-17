//! Honored Hierarch — `{G}` 1/1 Human Druid.
//! Renown 1.
//! "As long as this creature is renowned, it has vigilance and
//!  '{T}: Add one mana of any color.'"
//!
//! GAP: the renowned-conditional static (granting vigilance + a mana
//! ability only while renowned) is a continuous static ability with no
//! expressible primitive here; omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honored Hierarch");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Renown(1)],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
