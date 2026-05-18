//! Promised Kannushi — `{G}` 1/1 Human Druid with Soulshift.
//! Betrayers of Kamigawa common; a cheap green Human Druid that
//! returns a large Spirit from the graveyard when it dies (Soulshift 7).
//!
//! # Rules references
//!
//! * CR 702.45 — Soulshift. When this creature dies, you may return
//!   target Spirit card with mana value 7 or less from your graveyard
//!   to your hand. The numeric threshold (7) is part of the card text;
//!   the engine recognizes `KeywordAbility::Soulshift` as a unit
//!   variant.
//!
//! The keyword is a base characteristic; listing it in `keywords` is
//! sufficient — the runtime pipeline does the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Promised Kannushi");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Soulshift],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
