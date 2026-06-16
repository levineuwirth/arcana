//! Pardic Firecat — `{3}{R}` 2/3 Creature — Elemental Cat with Haste.
//!
//! * Haste (keyword).
//! * "If this card is in a graveyard, effects from spells named Flame Burst
//!   count it as a card named Flame Burst." — a static name-identity ability
//!   active in the graveyard; no primitive expresses card-name aliasing, so it
//!   is GAP'd. Only Haste is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pardic Firecat");
    let elemental = reg.interner_mut().intern("Elemental");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: graveyard name-aliasing static ("counts as a card named Flame
    // Burst") — no primitive for card-name identity changes.
    reg.register(CardDefinition::new(name, chars))
}
