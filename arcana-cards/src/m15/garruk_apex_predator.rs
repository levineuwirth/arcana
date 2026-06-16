//! Garruk, Apex Predator — `{5}{B}{G}` Legendary Planeswalker — Garruk, starting loyalty 5.
//!
//! +1: Destroy another target planeswalker. ("another" self-exclusion not
//!     expressible, but destroy target planeswalker is.)
//! +1: Create a 3/3 black Beast creature token with deathtouch.
//! −3: Destroy target creature. You gain life equal to its toughness. (The
//!     destroy is implemented; the "gain life equal to its toughness" rider
//!     GAPs — dynamic life gain from the destroyed creature's toughness is not
//!     expressible.)
//! −8: Target opponent gets an emblem with "Whenever a creature attacks you, it
//!     gets +5/+5 and gains trample until end of turn." (GAP — the
//!     attacks-you-from-the-emblem-owner's-perspective trigger + dynamic
//!     attacker pump is not expressible. Emblem shell created and assigned to
//!     the target opponent.)

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
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
    let _beast = reg.interner_mut().intern("Beast");
    let _emblem = reg.interner_mut().intern("Garruk, Apex Predator emblem");
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Destroy another target planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::PLANESWALKER.into()),
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/3 black Beast creature token with \
                       deathtouch."
                    .into(),
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
                effect: plus_one_beast,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature. You gain life equal to its \
                       toughness."
                    .into(),
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Target opponent gets an emblem with \"Whenever a \
                       creature attacks you, it gets +5/+5 and gains trample \
                       until end of turn.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(arcana_core::targets::ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+1: Destroy another target planeswalker.`
fn plus_one_destroy_pw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `+1: Create a 3/3 black Beast creature token with deathtouch.`
fn plus_one_beast(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned");
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

/// `−3: Destroy target creature. (gain-life=toughness rider GAP'd.)`
fn minus_three_destroy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "You gain life equal to its toughness" — dynamic life gain read from
    // the destroyed creature's toughness is not expressible.
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−8: target opponent gets a punishing emblem (trigger GAP'd; shell created).`
fn minus_eight_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Garruk, Apex Predator emblem")
        .expect("emblem name interned");
    // The emblem is granted to the TARGET OPPONENT, so its controller is the
    // chosen player, not ctx.controller.
    let owner = match ctx.targets.targets.first() {
        Some(TargetChoice::Player(p)) => *p,
        _ => return Vec::new(),
    };
    // GAP: "Whenever a creature attacks you, it gets +5/+5 and gains trample
    // until end of turn" — the attacks-you (emblem owner) trigger plus the
    // dynamic per-attacker pump is not expressible. Emblem shell assigned to
    // the target opponent.
    vec![Effect::CreateEmblem {
        controller: owner,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![],
        },
    }]
}
