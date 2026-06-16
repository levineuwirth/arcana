//! Plague Engineer — `{2}{B}` 2/2 Creature — Phyrexian Carrier with Deathtouch.
//!
//! * Deathtouch (keyword).
//! * As this creature enters, choose a creature type.
//! * Creatures of the chosen type your opponents control get -1/-1.
//!
//! Both non-keyword clauses depend on a chosen-creature-type mechanic that the
//! engine has no primitive for (no "choose a creature type" replacement / static
//! anchor). The static -1/-1 to opponents' chosen-type creatures is also a pure
//! static continuous ability. Only the Deathtouch keyword is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plague Engineer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let carrier = reg.interner_mut().intern("Carrier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(carrier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "As this creature enters, choose a creature type" — no
    // choose-a-creature-type primitive.
    // GAP: "Creatures of the chosen type your opponents control get -1/-1" —
    // pure static continuous ability gated on the chosen type; not expressible.
    reg.register(CardDefinition::new(name, chars))
}
