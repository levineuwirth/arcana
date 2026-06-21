//! Windbrisk Raptor — `{5}{W}{W}` 5/7 Creature — Bird with Flying.
//!
//! Oracle text:
//! * Flying — base keyword.
//! * "Attacking creatures you control have lifelink." — GAP: a static
//!   continuous keyword-granting anthem (lifelink to a dynamic set of
//!   attacking creatures) is not expressible on the MultiAbilityCreature
//!   shape.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windbrisk Raptor");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Attacking creatures you control have lifelink" — static anthem.
    reg.register(CardDefinition::new(name, chars))
}
