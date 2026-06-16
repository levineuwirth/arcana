//! Maze Abomination — `{5}{B}` 4/5 Elemental with Deathtouch.
//! "Multicolored creatures you control have deathtouch."
//!
//! Deathtouch is a base keyword. The board-wide static granting
//! deathtouch to multicolored creatures you control is a continuous
//! ability with no triggered/activated decomposition available.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maze Abomination");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: static — "Multicolored creatures you control have deathtouch";
    // no board-wide keyword-granting Effect available.
    reg.register(CardDefinition::new(name, chars))
}
