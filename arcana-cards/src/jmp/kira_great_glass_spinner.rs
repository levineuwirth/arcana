//! Kira, Great Glass-Spinner — `{1}{U}{U}` 2/2 Legendary Spirit with Flying.
//! Creatures you control have "Whenever this creature becomes the target of a
//! spell or ability for the first time each turn, counter that spell or ability."
//!
//! Flying is a base keyword. The second line is a STATIC continuous ability that
//! grants a triggered ability to every creature you control — it is not itself a
//! triggered or activated ability of Kira, and there is no continuous
//! ability-granting Effect/static available for this card class. GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kira, Great Glass-Spinner");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static continuous ability — "Creatures you control have '<triggered
    // ability counting target-for-the-first-time-each-turn and countering>'".
    // No continuous ability-granting static is available for this card class.
    reg.register(CardDefinition::new(name, chars))
}
