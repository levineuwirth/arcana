//! Taoist Hermit — `{2}{G}` 2/2 Human Mystic with Hexproof.
//! A green human mystic that cannot be targeted by opponents'
//! spells or abilities.
//!
//! # Rules references
//!
//! * CR 702.11 — Hexproof. This creature can't be the target of spells
//!   or abilities your opponents control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Taoist Hermit");
    let human = reg.interner_mut().intern("Human");
    let mystic = reg.interner_mut().intern("Mystic");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mystic);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
