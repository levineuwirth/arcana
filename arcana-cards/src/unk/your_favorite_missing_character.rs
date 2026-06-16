//! Your Favorite Missing Character — `{3}` 2/2 Legendary Creature.
//! A deckbuilding "create your own character" card: during deckbuilding the
//! player chooses two colors, up to three creature types, a new name, and ONE
//! of four listed abilities. None of those choices, nor the chosen ability, is
//! fixed at registration time, so only the bones are expressible here.
//!
//! GAP: "During deckbuilding, choose any two colors, up to three creature
//!      types, a new name, and one of the following abilities" — deckbuilding
//!      customization is not modeled; the four candidate abilities are all
//!      conditional on that choice and cannot be wired statically.
//! GAP: Surveil keyword is not in the usable KeywordAbility surface.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Your Favorite Missing Character");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
