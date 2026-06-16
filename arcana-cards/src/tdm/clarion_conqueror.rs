//! Clarion Conqueror — `{2}{W}` 3/3 Dragon with Flying.
//! "Activated abilities of artifacts, creatures, and planeswalkers
//! can't be activated."
//!
//! The activation-lock static has no demonstrated primitive and is
//! GAP'd; only Flying is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clarion Conqueror");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Activated abilities of artifacts, creatures, and
    // planeswalkers can't be activated" — no demonstrated primitive.
    reg.register(CardDefinition::new(name, chars))
}
