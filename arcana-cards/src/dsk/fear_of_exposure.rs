//! Fear of Exposure — `{2}{G}` 5/4 Enchantment Creature — Nightmare.
//! As an additional cost to cast this spell, tap two untapped creatures
//! and/or lands you control. (additional cast cost — GAP)
//! Trample.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of Exposure");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: "As an additional cost to cast this spell, tap two untapped
    // creatures and/or lands you control." — additional casting costs are not
    // expressible (no cast-cost hook on a creature CardDefinition; ActivationCost
    // only governs activated abilities).
    reg.register(CardDefinition::new(name, chars))
}
