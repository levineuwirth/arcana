//! Daybreak Chaplain — `{1}{W}` 1/3 Human Cleric with Lifelink.
//! Core Set 2019 common. Damage dealt by this creature also causes its
//! controller to gain that much life.
//!
//! # Rules references
//!
//! * CR 702.15 — Lifelink. Damage dealt by a creature with lifelink
//!   causes its controller to gain that much life simultaneously with
//!   the damage being dealt. Engine wiring lives in the combat damage
//!   and non-combat damage pipelines.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daybreak Chaplain");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
