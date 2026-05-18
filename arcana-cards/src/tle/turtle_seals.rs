//! Turtle-Seals — `{3}{U}` 2/4 Turtle Seal with Vigilance.
//! A sturdy blue creature that attacks without tapping, combining
//! defensive stats with offensive flexibility.
//!
//! # Rules references
//!
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature
//!   to tap; engine skips the tap step in `apply_declared_attackers`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Turtle-Seals");
    let turtle = reg.interner_mut().intern("Turtle");
    let seal = reg.interner_mut().intern("Seal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(seal);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
