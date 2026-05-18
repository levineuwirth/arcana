//! Benalish Hero — `{W}` 1/1 Human Soldier with Banding.
//! Alpha common (1993); one of the original Banding creatures. A cheap
//! white weenie whose rules text is entirely the Banding keyword.
//!
//! # Rules references
//!
//! * CR 702.21 — Banding. Any creatures with banding, and up to one
//!   without, can attack in a band. Bands are blocked as a group. If
//!   any creatures with banding you control are blocking or being blocked
//!   by a creature, you divide that creature's combat damage, not its
//!   controller, among any of the creatures it's being blocked by or is
//!   blocking.
//!
//! # Engine note
//!
//! `KeywordAbility::Banding` is not present in the demonstrated API set.
//! This card is emitted as a best-effort vanilla creature (no keywords
//! field entry) so the verify pipeline can flag the missing variant.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benalish Hero");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
