//! Jukai Messenger — `{G}` 1/1 Human Monk with Forestwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. Forestwalk: this creature can't be blocked
//!   as long as the defending player controls a Forest. Engine wiring
//!   maps Forestwalk to `KeywordAbility::Landwalk` keyed on the
//!   interned subtype "Forest".
//!
//! The generic Scryfall `Landwalk` umbrella keyword is ignored; only
//! the specific `Forestwalk` entry is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jukai Messenger");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
