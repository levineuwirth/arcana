//! Shellephant — `{1}{G}{G}` Turtle and/or Elephant (green). Base P/T is `?/?`
//! (defined by its ability).
//! {0}: Choose one. You may activate this ability while Shellephant is in any
//! zone.
//! • Shellephant has base power and toughness 1/4.
//! • Shellephant has base power and toughness 3/3.
//!
//! This is a MODAL activated ability ("choose one") that may be activated from
//! ANY zone. Activated abilities have no modal field (modal dispatch is a
//! spell-only mechanism here) and `activation_zone` names a single zone, so the
//! "choose one" + "any zone" + base-P/T-setting ability is not expressible —
//! GAP'd. The printed `?/?` is recorded as 0/0.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shellephant");
    let turtle = reg.interner_mut().intern("Turtle");
    let elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(elephant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "{0}: Choose one … (base P/T 1/4 or 3/3); may activate from any zone"
    // — activated abilities have no modal field and activation_zone is a single
    // zone; the choose-one base-P/T-setting ability is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
