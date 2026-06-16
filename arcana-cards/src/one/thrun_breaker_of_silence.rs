//! Thrun, Breaker of Silence — `{3}{G}{G}` 5/5 Legendary Troll Shaman with
//! Trample. Can't be countered; can't be targeted by nongreen spells/abilities
//! opponents control; has indestructible during your turn.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thrun, Breaker of Silence");
    let troll = reg.interner_mut().intern("Troll");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "This spell can't be countered" — static cast-time replacement, not
    // a triggered/activated ability.
    // GAP: "can't be the target of nongreen spells/abilities opponents control"
    // — static targeting restriction, no expressible primitive.
    // GAP: "During your turn, Thrun has indestructible" — conditional static
    // continuous ability, no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
