//! Garruk, Apex Predator — `{5}{B}{G}` Legendary Planeswalker — Garruk,
//! starting loyalty 5.
//! +1: Destroy another target planeswalker.
//! +1: Create a 3/3 black Beast creature token with deathtouch.
//! −3: Destroy target creature. You gain life equal to its toughness.
//! −8: Target opponent gets an emblem with "Whenever a creature attacks you,
//!   it gets +5/+5 and gains trample until end of turn."
//!
//! GAP: the +1 "destroy ANOTHER target planeswalker" self-exclusion is not
//!   expressible; the target is filtered to any planeswalker.
//! GAP: the −3 "you gain life equal to its toughness" rider is a dynamic
//!   resolution-time amount; only the destroy is emitted.
//! GAP: the −8 emblem is not in the demonstrated Effect catalog.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Apex Predator");
    let garruk = reg.interner_mut().intern("Garruk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +1: Destroy another target planeswalker.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Destroy another target planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_destroy_pw,
            })
            // +1: Create a 3/3 black Beast creature token with deathtouch.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/3 black Beast creature token with deathtouch.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_make_beast,
            })
            // −3: Destroy target creature. You gain life equal to its toughness.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target creature. You gain life equal to its toughness.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            // −8: emblem — GAP
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Target opponent gets an emblem with \"Whenever a creature attacks you, it gets +5/+5 and gains trample until end of turn.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_destroy_pw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "another" self-exclusion not expressible; filtered to any planeswalker.
    vec![Effect::DestroyPermanent { target: *id }]
}

fn plus_one_make_beast(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: beast,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Deathtouch],
            abilities: vec![],
        },
    }]
}

fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "you gain life equal to its toughness" is a dynamic amount; only
    // the destroy is emitted.
    vec![Effect::DestroyPermanent { target: *id }]
}

fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated Effect catalog.
    Vec::new()
}
