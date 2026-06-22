//! Nim Shrieker — `{3}{B}` 0/1 black Zombie.
//! Flying.
//! This creature gets +1/+0 for each artifact you control.
//!
//! Flying is a base keyword. The self-buffing static "+1/+0 for each artifact
//! you control" is a continuous characteristic-defining effect with no
//! triggered/activated form to wire — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nim Shrieker");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "This creature gets +1/+0 for each artifact you control" —
    //      a dynamic-P/T continuous static with no triggered/activated form.
    reg.register(CardDefinition::new(name, chars))
}
