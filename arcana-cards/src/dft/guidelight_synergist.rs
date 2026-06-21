//! Guidelight Synergist — `{3}{W}` 0/4 Artifact Creature — Robot Artificer
//! with Flying.
//!
//! Oracle:
//! * Flying.
//! * This creature gets +1/+0 for each artifact you control.
//!
//! Flying is a base characteristic. The dynamic self-pump ("+1/+0 for each
//! artifact you control") is a static continuous effect on the creature itself
//! with no expressible primitive, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guidelight Synergist");
    let robot = reg.interner_mut().intern("Robot");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "gets +1/+0 for each artifact you control" — self-referential
    // dynamic static pump; no primitive.
    reg.register(CardDefinition::new(name, chars))
}
