//! Spider-Punk — `{1}{R}` 2/1 Legendary Spider Human Hero with Riot.
//! Static abilities (other Spiders have riot; spells/abilities can't be
//! countered; damage can't be prevented) are GAP'd — none are expressible
//! as triggered/activated abilities or available primitives.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spider-Punk");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    // GAP: "Other Spiders you control have riot" — static keyword grant.
    // GAP: "Spells and abilities can't be countered" — global static.
    // GAP: "Damage can't be prevented" — global static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Riot],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
