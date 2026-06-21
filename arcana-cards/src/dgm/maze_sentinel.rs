//! Maze Sentinel — `{5}{W}` 3/6 white Elemental with Vigilance.
//!
//! Oracle:
//! * Vigilance — base keyword.
//! * Multicolored creatures you control have vigilance. — a static continuous
//!   anthem-style ability; not expressible as a triggered/activated ability in
//!   this shape, GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maze Sentinel");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP: static — "Multicolored creatures you control have vigilance"
    // is a continuous keyword-granting static, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
