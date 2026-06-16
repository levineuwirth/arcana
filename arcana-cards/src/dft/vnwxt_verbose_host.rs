//! Vnwxt, Verbose Host — `{1}{U}` 0/4 Legendary Homunculus.
//! Start your engines! / Max speed mechanics, "no maximum hand size",
//! and the Max-speed draw-replacement are all static/keyword text the
//! usable surface does not express, so this is bones-only.
//!
//! GAP: "Start your engines!" / "Max speed" — not usable KeywordAbility
//! variants (speed mechanic unmodeled).
//! GAP: "You have no maximum hand size." — static, no primitive.
//! GAP: "Max speed — If you would draw a card, draw two cards instead."
//! — draw-replacement gated on max speed, no primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vnwxt, Verbose Host");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
