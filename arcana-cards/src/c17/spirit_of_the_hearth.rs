//! Spirit of the Hearth — `{4}{W}{W}` 4/5 Cat Spirit with Flying.
//! "You have hexproof."
//!
//! Flying is a base keyword. The "you have hexproof" static (granting
//! the controller player hexproof) is a player-targeting continuous
//! ability with no triggered/activated decomposition available here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit of the Hearth");
    let cat = reg.interner_mut().intern("Cat");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "You have hexproof" — grants the controlling PLAYER
    // hexproof; no triggered/activated decomposition and no player-level
    // hexproof-granting Effect in the available catalog.
    reg.register(CardDefinition::new(name, chars))
}
