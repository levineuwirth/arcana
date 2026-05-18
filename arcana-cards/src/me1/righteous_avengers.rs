//! Righteous Avengers — `{4}{W}` 3/1 Human Soldier with Plainswalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. Plainswalk: this creature can't be blocked
//!   as long as the defending player controls a Plains. Engine wiring
//!   maps Plainswalk to `KeywordAbility::Landwalk` keyed on the
//!   interned subtype "Plains".
//!
//! The generic Scryfall `Landwalk` umbrella keyword is ignored; only
//! the specific `Plainswalk` entry is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Righteous Avengers");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(plains)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
