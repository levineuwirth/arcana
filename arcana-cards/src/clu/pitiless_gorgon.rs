//! Pitiless Gorgon — `{1}{B/G}{B/G}` 2/2 Gorgon with Deathtouch.
//! Guilds of Ravnica common; a hybrid black-green gorgon whose gaze is
//! lethal — any damage it deals destroys the target creature.
//!
//! # Rules references
//!
//! * CR 702.2 — Deathtouch. Any amount of damage dealt by this creature
//!   is enough to destroy the damaged creature.
//!
//! Colors: B, G (hybrid cost — multicolor black+green per spec).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pitiless Gorgon");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
