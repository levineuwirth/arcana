//! Scurrid Colony — `{1}{G}` 2/2 Squirrel with Reach.
//! "This creature gets +2/+2 as long as you control eight or more lands."
//!
//! The keyword (Reach) is a base characteristic. The static
//! conditional pump is a pure continuous static ability that the
//! MultiAbilityCreature surface (triggered/activated only) cannot
//! express, so it is GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scurrid Colony");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: static "gets +2/+2 as long as you control eight or more lands"
    // is a continuous conditional static buff — not a triggered or
    // activated ability, and not expressible on this card class.
    reg.register(CardDefinition::new(name, chars))
}
