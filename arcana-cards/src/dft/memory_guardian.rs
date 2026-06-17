//! Memory Guardian — `{4}{U}` 3/4 Artifact Creature — Robot Artificer.
//! Affinity for artifacts.
//! Flying.
//!
//! Affinity is a cost-reduction keyword not in the usable keyword surface
//! for this card class — see GAP. Flying is a base keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Memory Guardian");
    let robot = reg.interner_mut().intern("Robot");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(artificer);

    // GAP: Affinity for artifacts (cost reduction) is not in the usable
    // keyword surface for this card class; omitted.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
