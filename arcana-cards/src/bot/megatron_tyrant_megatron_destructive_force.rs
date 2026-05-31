//! Megatron, Tyrant // Megatron, Destructive Force — {3}{R}{W}{B}
//! Legendary Artifact Creature — Robot (7/5), transforming ("convert") into
//! Legendary Artifact — Vehicle (Megatron, Destructive Force).
//!
//! Front: More Than Meets the Eye {1}{R}{W}{B}. Your opponents can't cast spells during combat.
//!        At the beginning of each of your postcombat main phases, you may convert Megatron;
//!        if you do, add {C} for each 1 life your opponents have lost this turn.
//! Back: Living metal. Whenever Megatron attacks, you may sacrifice another artifact; when you do,
//!       it deals damage equal to the sacrificed artifact's mana value to target creature, with
//!       excess redirected to that creature's controller and you convert Megatron.
//!
//! GAP: keywords "Living metal", "Convert", and "More Than Meets the Eye" are not in the engine's
//!      usable keyword surface — emitting keywords: vec![] for both faces.
//! GAP: front static "Your opponents can't cast spells during combat" is not expressible.
//! GAP: front "At the beginning of each of your postcombat main phases, you may convert Megatron;
//!      if you do, add {C} for each 1 life your opponents have lost this turn" — there is no
//!      script helper for life-lost-this-turn and the optional-convert/dynamic-mana shape is not
//!      expressible. Not authored.
//! GAP: back attack trigger (you may sacrifice another artifact; deal damage equal to its mana
//!      value with excess-damage redirection and a follow-on convert) — the optional-sacrifice
//!      reflexive trigger, sacrificed-object's mana value, and excess-damage redirection are not
//!      expressible. Not authored.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Megatron, Tyrant");
    let robot_sub = reg.interner_mut().intern("Robot");
    let back_name = reg.interner_mut().intern("Megatron, Destructive Force");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");

    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: front_subs,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(vehicle_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT),
            subtypes: back_subs,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
