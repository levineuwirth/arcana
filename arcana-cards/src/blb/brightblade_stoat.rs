//! Brightblade Stoat — `{1}{W}` 2/2 Weasel Soldier with First Strike
//! and Lifelink.
//! An aggressive soldier that deals damage before most creatures
//! can strike back and gains its controller life equal to damage dealt.
//!
//! # Rules references
//!
//! * CR 702.7 — First Strike. This creature deals combat damage before
//!   creatures without first or double strike in the first combat
//!   damage step.
//! * CR 702.15 — Lifelink. Damage dealt by this creature causes its
//!   controller to gain that much life.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brightblade Stoat");
    let weasel = reg.interner_mut().intern("Weasel");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weasel);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
