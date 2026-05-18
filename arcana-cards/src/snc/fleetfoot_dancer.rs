//! Fleetfoot Dancer — `{1}{R}{G}{W}` 4/4 Elf Druid with Trample, Lifelink,
//! and Haste. A three-color (R/G/W) creature from Streets of New Capenna.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. If a creature with trample would assign damage to
//!   a blocking creature, it may assign excess damage to the defending player.
//! * CR 702.15 — Lifelink. Damage dealt by this creature also causes its
//!   controller to gain that much life.
//! * CR 702.10 — Haste. This creature can attack and use tap abilities the
//!   turn it enters the battlefield.
//!
//! All three keywords are base characteristics; the runtime pipelines handle
//! enforcement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fleetfoot Dancer");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Lifelink,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
