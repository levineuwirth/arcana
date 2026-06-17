//! Pearl Lake Ancient — `{5}{U}{U}` 6/7 Leviathan with Flash.
//! "This spell can't be countered." Prowess. "Return three lands you
//! control to their owner's hand: Return this creature to its owner's
//! hand."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pearl Lake Ancient");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Prowess — not in the supported keyword surface.
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    // GAP: "This spell can't be countered." — no can't-be-countered static in
    //      the demonstrated API.
    // GAP: "Return three lands you control to their owner's hand: Return this
    //      creature to its owner's hand." — "return permanents you control" is
    //      not an expressible ActivationCost field.
    reg.register(CardDefinition::new(name, chars))
}
