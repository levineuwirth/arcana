//! Drogskol Captain — `{1}{W}{U}` 2/2 Spirit Soldier (white/blue).
//! Flying.
//! Other Spirit creatures you control get +1/+1 and have hexproof.
//!
//! Flying is a base keyword. The Spirit anthem-plus-hexproof clause is a pure
//! STATIC continuous ability and is not expressible in the
//! MultiAbilityCreature shape — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drogskol Captain");
    let spirit = reg.interner_mut().intern("Spirit");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Other Spirit creatures you control get +1/+1 and have hexproof." (continuous anthem + keyword grant).
    reg.register(CardDefinition::new(name, chars))
}
