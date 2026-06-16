//! Divinity of Pride — `{W/B}{W/B}{W/B}{W/B}{W/B}` 4/4 Spirit Avatar with Flying
//! and Lifelink.
//! "This creature gets +4/+4 as long as you have 25 or more life." (static — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Divinity of Pride");
    let spirit = reg.interner_mut().intern("Spirit");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/B}{W/B}{W/B}{W/B}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: static "gets +4/+4 as long as you have 25 or more life" — a conditional
    // continuous self-buff (no trigger/cost); not expressible.
    reg.register(CardDefinition::new(name, chars))
}
