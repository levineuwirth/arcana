//! Voldaren Pariah // Abolisher of Bloodlines
//!
//! Front: {3}{B}{B} Creature — Vampire Horror 3/3.
//! Flying.
//! Sacrifice three other creatures: Transform this creature. (Activated ability)
//! GAP: Madness {B}{B}{B} — madness mechanic not modeled.
//! GAP: Sacrifice-three-creatures activated cost uses choice-bearing sacrifice cost
//! (sacrifice_other × 3) — not expressible as OptionalPaymentKind; activation not modeled.
//!
//! Back: Creature — Eldrazi Vampire. Flying.
//! When this creature transforms into Abolisher of Bloodlines, target opponent sacrifices
//! three creatures of their choice.
//! GAP: back-face-only triggered ability not modeled (opponent sacrifice 3 on transform).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voldaren Pariah");
    let sub_vampire = reg.interner_mut().intern("Vampire");
    let sub_horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_vampire);
    subtypes.0.insert(sub_horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Abolisher of Bloodlines");
    let sub_eldrazi = reg.interner_mut().intern("Eldrazi");
    let sub_vampire_b = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_eldrazi);
    back_subtypes.0.insert(sub_vampire_b);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Sacrifice three other creatures activation cost (choice-bearing sacrifice cost)
    // is not expressible as OptionalPaymentKind. Transform activation not modeled.
    // GAP: Madness {B}{B}{B} not modeled.
    // GAP: back-face-only triggered ability not modeled (opponent sacrifices 3 creatures on transform).
    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
