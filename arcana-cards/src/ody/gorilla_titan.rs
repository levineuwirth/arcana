//! Gorilla Titan — `{3}{G}{G}` 4/4 Ape with Trample.
//!
//! Oracle:
//! * Trample — base keyword.
//! * "This creature gets +4/+4 as long as there are no cards in your
//!   graveyard." — GAP: a conditional continuous static (no
//!   triggered/activated form to express the graveyard-gated pump).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gorilla Titan");
    let ape = reg.interner_mut().intern("Ape");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
