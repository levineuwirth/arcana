//! Siege Elemental — `{4}{R}{R}` 6/6 Creature — Elemental with Trample.
//! "Untapped creatures can't block."
//! "Tapped creatures can block."
//!
//! Only Trample is expressible; both block-restriction lines are static
//! continuous abilities with no triggered/activated decomposition.

// GAP: "Untapped creatures can't block." — static combat-restriction
//      continuous ability, not expressible as a triggered/activated ability.
// GAP: "Tapped creatures can block." — static combat-permission
//      continuous ability, not expressible as a triggered/activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Siege Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
