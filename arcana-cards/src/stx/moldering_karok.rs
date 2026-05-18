//! Moldering Karok — `{2}{B}{G}` 3/3 Zombie Crocodile with Trample and Lifelink.
//! A two-keyword threat in Sultai colours; the combination of Trample
//! and Lifelink rewards attacking through blockers by recovering life
//! equal to all damage dealt, including excess trample damage.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. Excess combat damage is assigned to the
//!   defending player after blockers are assigned lethal damage.
//! * CR 702.15 — Lifelink. Damage dealt by this creature also causes
//!   its controller to gain that much life.
//!
//! Both keywords are base characteristics, so listing them in
//! `keywords` is sufficient — the runtime combat pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moldering Karok");
    let zombie = reg.interner_mut().intern("Zombie");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
