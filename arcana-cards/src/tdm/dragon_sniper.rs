//! Dragon Sniper — `{G}` 1/1 Human Archer with Vigilance, Reach, and
//! Deathtouch.
//! A precision hunter that can block flyers, does not tap when
//! attacking, and destroys any creature it damages.
//!
//! # Rules references
//!
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature
//!   to tap.
//! * CR 702.17 — Reach. This creature can block creatures with flying.
//! * CR 702.2 — Deathtouch. Any amount of damage dealt to a creature
//!   by this source is enough to destroy it.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragon Sniper");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Reach,
            KeywordAbility::Deathtouch,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
