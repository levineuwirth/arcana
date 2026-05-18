//! Forked-Branch Garami — `{3}{G}{G}` 4/4 Spirit with Soulshift(4).
//! Betrayers of Kamigawa uncommon; a green Spirit whose oracle text
//! prints "Soulshift 4, soulshift 4", each instance independently
//! allowing the return of a Spirit card with mana value 4 or less when
//! it dies. The two identical instances collapse to a single
//! `KeywordAbility::Soulshift(4)` entry in the keyword vec; the
//! multi-return semantics are handled at the rules layer.
//!
//! # Rules references
//!
//! * CR 702.45 — Soulshift N. "When this creature dies, you may return
//!   target Spirit card with mana value N or less from your graveyard
//!   to your hand." N is 4 for this card; encoded as
//!   `KeywordAbility::Soulshift(4)` where the `u8` argument carries the
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
        keywords: vec![KeywordAbility::Soulshift(4)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
