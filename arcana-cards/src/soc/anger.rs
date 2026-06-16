//! Anger — `{3}{R}` 2/2 Incarnation with Haste.
//! "As long as this card is in your graveyard and you control a
//! Mountain, creatures you control have haste."
//!
//! The graveyard-static anthem is a pure continuous ability (no trigger,
//! no cost) and is not expressible as a triggered/activated ability —
//! GAP'd. Only Haste and the bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anger");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "As long as this card is in your graveyard and you control a
    // Mountain, creatures you control have haste." — graveyard-static
    // anthem, not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
