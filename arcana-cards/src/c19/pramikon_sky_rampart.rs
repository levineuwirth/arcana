//! Pramikon, Sky Rampart — `{U}{R}{W}` 1/5 Legendary Wall with Flying and Defender.
//!
//! The ETB "choose left or right" attack-direction restriction is a multiplayer
//! seating/attack-legality static with no expressible primitive, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pramikon, Sky Rampart");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Defender],
        // GAP: "As Pramikon enters, choose left or right" + directional attack restriction —
        // no replacement/static primitive for multiplayer attack-direction limits.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
