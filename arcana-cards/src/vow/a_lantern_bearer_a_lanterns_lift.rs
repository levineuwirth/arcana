//! A-Lantern Bearer // A-Lanterns' Lift — `{U}` Creature — Spirit 1/1 with Flying.
//! Disturb {1}{U}: cast from graveyard transformed (GAP: Disturb keyword not in engine).
//! Back face (A-Lanterns' Lift): Enchantment — Aura.
//!   Enchant creature. Enchanted creature gets +1/+1 and has flying.
//!   If Lanterns' Lift would be put into a graveyard from anywhere, exile it instead.
//!
//! # GAPs
//! - Disturb keyword and graveyard-cast-transformed mechanic not in engine API — omitted.
//! - "If Lanterns' Lift would be put into a graveyard from anywhere, exile it instead"
//!   is a replacement effect (zone-change replacement). Not expressible via Effect catalog.
//! - Back face Aura enchant-creature bonus (+1/+1, flying) is a static layer ability
//!   on the back face — not expressible on a CardFace (no static ability field).
//!   The back face is registered as a bare Aura shell.
//! - GAP: back-face-only triggered abilities and static abilities not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Lantern Bearer");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Disturb {1}{U} not expressible (keyword not in engine)
        ..Default::default()
    };

    // Back face: A-Lanterns' Lift — Enchantment — Aura
    let back_name = reg.interner_mut().intern("A-Lanterns' Lift");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
        // GAP: enchant creature (+1/+1, flying) static layer ability not expressible on back face.
        // GAP: "if this would be put into a graveyard, exile instead" replacement effect not modeled.
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
