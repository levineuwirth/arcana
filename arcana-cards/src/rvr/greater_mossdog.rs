//! Greater Mossdog — `{3}{G}` 3/3 Plant Dog with Dredge 3.
//!
//! # Rules references
//!
//! * CR 702.52 — Dredge. If you would draw a card, you may mill three
//!   cards instead. If you do, return this card from your graveyard to
//!   your hand.
//!
//! Dredge is not in the demonstrated KeywordAbility API; this file is a
//! best-effort stub. The verify pipeline will flag the gap.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greater Mossdog");
    let plant = reg.interner_mut().intern("Plant");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
