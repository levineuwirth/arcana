//! Forked-Branch Garami — `{3}{G}{G}` 4/4 Spirit with Soulshift.
//! Betrayers of Kamigawa uncommon; a green Spirit with two instances
//! of Soulshift 4, allowing return of up to two Spirit cards when it
//! dies.
//!
//! # Rules references
//!
//! * CR 702.45 — Soulshift. When this creature dies, you may return
//!   target Spirit card with mana value 4 or less from your graveyard
//!   to your hand. This card has two instances of Soulshift 4, each
//!   triggering independently; the engine recognizes
//!   `KeywordAbility::Soulshift` as a unit variant. Both instances are
//!   represented by a single `Soulshift` entry in the keyword vec
//!   (the engine records the unit marker; the numeric N and instance
//!   count are not encoded in the variant).
//!
//! The keyword is a base characteristic; listing it in `keywords` is
//! sufficient — the runtime pipeline does the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forked-Branch Garami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Soulshift],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
