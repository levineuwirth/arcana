//! Vivien Reid — `{3}{G}{G}` Legendary Planeswalker — Vivien, starting
//! loyalty 5.
//!
//! +1: Look at the top four cards of your library. You may reveal a
//!     creature or land card from among them and put it into your hand.
//!     Put the rest on the bottom of your library in a random order.
//! −3: Destroy target artifact, enchantment, or creature with flying.
//! −8: You get an emblem with "Creatures you control get +2/+2 and have
//!     vigilance, trample, and indestructible." (GAP — emblem.)

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien Reid");
    let vivien = reg.interner_mut().intern("Vivien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top four cards of your library. You may reveal a creature or land card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target artifact, enchantment, or creature with flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        custom: Some(is_artifact_enchantment_or_flier),
                        ..Default::default()
                    }),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Creatures you control get +2/+2 and have vigilance, trample, and indestructible.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn is_artifact_enchantment_or_flier(
    obj: &arcana_core::objects::GameObject,
    state: &GameState,
) -> bool {
    let types = obj.characteristics.types;
    types.has(TypeLine::ARTIFACT)
        || types.has(TypeLine::ENCHANTMENT)
        || (types.has(TypeLine::CREATURE)
            && state.has_keyword(obj.id, &arcana_core::effects::KeywordAbility::Flying))
}

fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(ObjectFilter {
            types_any: Some(TypeLine(TypeLine::CREATURE | TypeLine::LAND)),
            ..Default::default()
        }),
        rest: arcana_core::effects::DigRest::BottomRandom,
    }]
}

fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a static anthem ("Creatures you control get +2/+2 and
    //      have vigilance, trample, and indestructible"). EmblemDefinition
    //      carries only triggered abilities; a continuous anthem static can't
    //      be expressed as an emblem ability here.
    Vec::new()
}
