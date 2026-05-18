//! Thousand-legged Kami — `{6}{G}{G}` 6/6 Spirit with Soulshift(7).
//! Saviors of Kamigawa uncommon; a large green Spirit that returns a
//! Spirit card with mana value 7 or less from the graveyard when it
//! dies.
//!
//! # Rules references
//!
//! * CR 702.45 — Soulshift N. "When this creature dies, you may return
//!   target Spirit card with mana value N or less from your graveyard
//!   to your hand." N is 7 for this card; encoded as
//!   `KeywordAbility::Soulshift(7)` where the `u8` argument carries the
//!   mana-value threshold.
//!
//! The keyword is a base characteristic; the runtime soulshift pipeline
//! reads the `(N)` argument to enforce the mana-value cap.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thousand-legged Kami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Soulshift(7)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
