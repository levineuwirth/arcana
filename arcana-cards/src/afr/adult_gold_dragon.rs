//! Adult Gold Dragon — `{3}{R}{W}` 4/3 Dragon with Flying, Lifelink, and Haste.
//! D&D Adventures in the Forgotten Realms uncommon; a red-white dragon
//! that flies in immediately and gains life for every point of damage dealt.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.15 — Lifelink. Damage dealt by this creature causes its controller
//!   to gain that much life.
//! * CR 702.10 — Haste. This creature can attack and activate abilities with
//!   {T} the turn it enters the battlefield.
//!
//! Colors: R, W.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adult Gold Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Lifelink,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
