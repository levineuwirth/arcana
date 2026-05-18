//! Drakewing Krasis — `{1}{G}{U}` 3/1 Lizard Drake with Flying and Trample.
//! A green-blue creature from Gatecrash; a Simic hybrid combining aerial
//! agility with ground-breaking momentum.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.19 — Trample. If a creature with trample would assign damage to
//!   a blocking creature, it may assign excess damage to the defending player.
//!
//! Both keywords are base characteristics; the runtime pipelines handle
//! enforcement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drakewing Krasis");
    let lizard = reg.interner_mut().intern("Lizard");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
