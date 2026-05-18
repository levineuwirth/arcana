//! Alaborn Grenadier — `{W}{W}` 2/2 Human Soldier with Vigilance.
//! Portal Second Age common (1998); a disciplined white soldier who
//! can attack and still be ready to defend, representing the steadfast
//! military tradition of the Alaborn people.
//!
//! # Rules references
//!
//! * CR 702.20 — Vigilance. Attacking doesn't cause the creature to
//!   tap; engine skips the tap in `apply_declared_attackers`.
//!
//! Vigilance is a base characteristic on this card, so nothing beyond
//! listing it in `keywords` is required — the runtime pipelines do
//! the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alaborn Grenadier");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
