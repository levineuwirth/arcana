//! Roaming Throne — `{4}` 4/4 Artifact Creature — Golem with Ward {2}.
//! "As this creature enters, choose a creature type. This creature is the
//!  chosen type in addition to its other types. If a triggered ability of
//!  another creature you control of the chosen type triggers, it triggers
//!  an additional time."
//!
//! The choose-a-type, the type-addition static, and the trigger-doubling
//! static have no expressible triggered/activated form. Bones + Ward only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roaming Throne");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: "As this creature enters, choose a creature type" + "is the chosen
    // type in addition to its other types" + "if a triggered ability of
    // another creature you control of the chosen type triggers, it triggers an
    // additional time" — all static/replacement, not triggered/activated.
    reg.register(CardDefinition::new(name, chars))
}
