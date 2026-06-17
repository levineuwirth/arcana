//! Fear of Isolation — `{1}{U}` 2/3 Enchantment Creature — Nightmare with Flying.
//!
//! "As an additional cost to cast this spell, return a permanent you control to
//! its owner's hand." Additional casting costs are not expressible with the
//! demonstrated keyword/ability surface, so only the bones + Flying are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of Isolation");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As an additional cost to cast this spell, return a permanent you
    // control to its owner's hand" — additional casting costs are not modeled.
    reg.register(CardDefinition::new(name, chars))
}
