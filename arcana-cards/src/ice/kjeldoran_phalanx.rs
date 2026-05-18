//! Kjeldoran Phalanx — `{5}{W}` 2/5 Human Soldier with First Strike and Banding.
//! Ice Age uncommon (1995); a high-cost white defensive wall combining
//! first-strike punishment with banding's damage-assignment control.
//!
//! # Rules references
//!
//! * CR 702.7 — First Strike. Deals combat damage in the first combat
//!   damage step; opposing creatures without first/double strike deal
//!   damage in the second step.
//! * CR 702.21 — Banding. Any creatures with banding, and up to one
//!   without, can attack in a band. Bands are blocked as a group. If
//!   any creatures with banding you control are blocking or being
//!   blocked by a creature, you divide that creature's combat damage,
//!   not its controller, among any of the creatures it's being blocked
//!   by or is blocking.
//!
//! Both keywords are base characteristics on this card; listing them
//! in `keywords` is sufficient — the runtime pipeline handles the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kjeldoran Phalanx");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
