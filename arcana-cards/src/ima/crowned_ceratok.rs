//! Crowned Ceratok — `{3}{G}` 4/3 Rhino with Trample.
//!
//! * Trample (keyword).
//! * "Each creature you control with a +1/+1 counter on it has trample." —
//!   a static continuous granted ability; GAP'd (no triggered/activated form).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crowned Ceratok");
    let rhino = reg.interner_mut().intern("Rhino");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "Each creature you control with a +1/+1 counter on it has
    // trample." — a counter-conditioned continuous keyword-grant static, not a
    // triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
