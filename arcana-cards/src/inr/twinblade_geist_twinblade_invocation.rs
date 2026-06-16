//! Twinblade Geist // Twinblade Invocation
//!
//! Front face: {1}{W} Creature — Spirit Warrior 1/1, Double strike.
//!   Disturb {2}{W} (You may cast this card from your graveyard transformed for its
//!   disturb cost.)
//!
//! Back face: Enchantment — Aura.
//!   Enchant creature. Enchanted creature has double strike.
//!   If Twinblade Invocation would be put into a graveyard from anywhere, exile it
//!   instead.
//!
//! GAP: Disturb (cast from graveyard transformed) not modeled by engine transform
//! primitive; registered as front face only.
//! GAP: The Aura back face's "enchant creature" attachment is not modeled via
//! transform (back face would need Aura type + enchant behavior). Back face
//! registered as Enchantment creature stats placeholder with DoubleStrike keyword.
//! GAP: "If Twinblade Invocation would be put into a graveyard, exile it instead" —
//! replacement effect not modeled.
//! GAP: back-face-only Aura grant ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twinblade Geist");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);
    subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Twinblade Invocation");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    // GAP: back face is an Enchantment — Aura (not a creature). The engine's
    // transform back face accepts Characteristics; we model it as Enchantment
    // with the Aura subtype and no P/T.
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Disturb cost {2}{W} (cast from graveyard transformed) not modeled.
    // GAP: "enchanted creature has double strike" back-face Aura effect not modeled.
    // GAP: "exile instead of graveyard" replacement on back face not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
