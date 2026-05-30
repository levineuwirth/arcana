//! Pyretic Prankster // Glistening Goremonger — `{1}{R}` Devil creature 2/1.
//! Front face: {3}{B/P}: Transform this creature. Activate only as a sorcery.
//! ({B/P} can be paid with either {B} or 2 life.)
//! Back face: Phyrexian Devil with "When this creature dies, each opponent
//! sacrifices an artifact or creature of their choice."
//!
//! GAP: Hybrid Phyrexian mana cost {B/P} for activated ability not expressible —
//! activated abilities with custom payment shapes not modeled; wired as a triggered
//! ability (ZoneChange to battlefield) approximating ETB instead.
//! GAP: "sorcery speed only" restriction on activated ability not modeled.
//! GAP: back-face-only triggered ability (dies trigger) not modeled.
//! GAP: "each opponent sacrifices an artifact or creature of their choice" — opponent
//! choice targeting and artifact-or-creature filter not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pyretic Prankster");
    let devil_sub = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Glistening Goremonger");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_devil_sub = reg.interner_mut().intern("Devil");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(back_devil_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: {3}{B/P} activated ability (transform, sorcery-speed) not modeled —
    // OptionalPaymentKind does not support hybrid-Phyrexian mana and activated
    // abilities with custom cost shapes are not in scope.
    // GAP: back-face-only triggered ability (dies trigger: each opponent sacrifices
    // an artifact or creature) not modeled.

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
