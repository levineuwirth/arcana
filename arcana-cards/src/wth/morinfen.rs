//! Morinfen — `{3}{B}{B}` 5/4 Legendary Phyrexian Horror.
//! Flying.
//! Cumulative upkeep—Pay 1 life.
//!
//! Flying is a base keyword. Cumulative upkeep is not an expressible
//! keyword or activation/trigger in the demonstrated API (no age-counter
//! escalating upkeep cost mechanic), so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Morinfen");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Cumulative upkeep—Pay 1 life (escalating age-counter upkeep cost
    // is not an expressible keyword or trigger in the demonstrated API).
    reg.register(CardDefinition::new(name, chars))
}
