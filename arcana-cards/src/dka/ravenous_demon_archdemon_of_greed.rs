//! Ravenous Demon // Archdemon of Greed — `{3}{B}{B}` Demon creature 4/4.
//! Front: Sacrifice a Human: Transform this creature. Activate only as a sorcery.
//! Back: Flying, trample. At the beginning of your upkeep, sacrifice a Human. If you can't,
//!       tap this creature and it deals 9 damage to you.
//!
//! GAP: "Sacrifice a Human" as an activation COST — ActivationCost.sacrifice_other supports
//! a filter but the front-face activated ability requires sorcery speed AND a sacrifice cost.
//! The sacrifice_other field is modeled on ActivationCost but "Sacrifice a Human" specifically
//! requires a Human filter. Authored below with sacrifice_other.
//! GAP: Back-face upkeep trigger "sacrifice a Human. If you can't, tap this creature and
//! it deals 9 damage to you" — the conditional "if you can't sacrifice" check is not
//! expressible. GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ravenous Demon");
    let demon_sub = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon_sub);

    // Pre-intern "Human" for sacrifice_other filter
    let human_sub = reg.interner_mut().intern("Human");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Archdemon of Greed");
    let mut back_subtypes = SubtypeSet::default();
    let back_demon_sub = reg.interner_mut().intern("Demon");
    back_subtypes.0.insert(back_demon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
            power: Some(PtValue::Fixed(9)),
            toughness: Some(PtValue::Fixed(9)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Build human filter for sacrifice_other cost
    let human_filter = ObjectFilter::creature().with_subtypes_any(vec![human_sub]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: Sacrifice a Human: Transform this creature. (Sorcery speed)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Human: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(human_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            })
            // GAP: back-face-only triggered ability not modeled
            // (upkeep: sacrifice a Human; if you can't, tap + 9 damage to you)
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
