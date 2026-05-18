//! Venomthrope — `{1}{G}{U}` 2/2 Tyranid with Flying, Deathtouch, and Hexproof.
//!
//! Green-blue multicolor creature.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.2 — Deathtouch. Any amount of damage this creature deals is
//!   enough to destroy a creature.
//! * CR 702.11 — Hexproof. This creature can't be the target of spells or
//!   abilities your opponents control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Venomthrope");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let colors = ColorSet::green() | ColorSet::blue();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors,
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
            KeywordAbility::Hexproof,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
