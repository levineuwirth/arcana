//! Dauthi Marauder — `{2}{B}` 3/1 Dauthi Minion with Shadow.
//! Tempest common; a fast aggressive shadow creature that can only
//! interact with other shadow creatures in combat.
//!
//! # Rules references
//!
//! * CR 702.27 — Shadow (fully implemented). This creature can block
//!   or be blocked by only creatures with shadow.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauthi Marauder");
    let dauthi = reg.interner_mut().intern("Dauthi");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dauthi);
    subtypes.0.insert(minion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
