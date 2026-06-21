//! Kaheera, the Orphanguard — `{1}{G/W}{G/W}` 3/2 Legendary Creature —
//! Cat Beast. Colors G/W (hybrid pips).
//!
//! Oracle:
//! - "Companion — Each creature card in your starting deck is a Cat, Elemental,
//!   Nightmare, Dinosaur, or Beast card." — GAP: Companion is a deck-building
//!   restriction with no usable KeywordAbility / engine surface.
//! - Vigilance — keyword.
//! - "Each other creature you control that's a Cat, Elemental, Nightmare,
//!   Dinosaur, or Beast gets +1/+1 and has vigilance." — GAP: a static
//!   continuous anthem (no trigger word, no cost) is not a triggered/activated
//!   ability, so it cannot be expressed in this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaheera, the Orphanguard");
    let cat = reg.interner_mut().intern("Cat");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Companion deck-building restriction is not expressible.
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP static: "Each other creature you control that's a Cat, Elemental,
    // Nightmare, Dinosaur, or Beast gets +1/+1 and has vigilance." — a static
    // anthem, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
