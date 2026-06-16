//! Nobilis of War — `{R/W}{R/W}{R/W}{R/W}{R/W}` 3/4 Spirit Avatar.
//! Flying.
//! Attacking creatures you control get +2/+0. (A pure static continuous
//! ability — not a triggered/activated ability, GAP'd below.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nobilis of War");
    let spirit = reg.interner_mut().intern("Spirit");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/W}{R/W}{R/W}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "Attacking creatures you control get +2/+0" is a pure
    // continuous anthem (no trigger word, no cost); not expressible as a
    // triggered/activated ability with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
