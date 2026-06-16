//! Phelddagrif — `{1}{G}{W}{U}` 4/4 Legendary Phelddagrif.
//!
//! Oracle (three activated abilities):
//! * {G}: Phelddagrif gains trample until end of turn. Target opponent creates
//!   a 1/1 green Hippo creature token.
//! * {W}: Phelddagrif gains flying until end of turn. Target opponent gains 2 life.
//! * {U}: Return Phelddagrif to its owner's hand. Target opponent may draw a card.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phelddagrif");
    let phelddagrif = reg.interner_mut().intern("Phelddagrif");
    // Pre-intern the Hippo token subtype.
    let _hippo = reg.interner_mut().intern("Hippo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phelddagrif);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: Phelddagrif gains trample until end of turn. Target opponent creates a 1/1 green Hippo creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: green_trample_hippo,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: Phelddagrif gains flying until end of turn. Target opponent gains 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: white_flying_gain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: Return Phelddagrif to its owner's hand. Target opponent may draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blue_bounce_draw,
            }),
    )
}

fn green_trample_hippo(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let hippo = reg.interner().lookup("Hippo").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hippo);
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::CreateToken {
            controller: *p,
            token: TokenDefinition {
                name: hippo,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}

fn white_flying_gain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        Effect::GainLife { player: *p, amount: 2 },
    ]
}

fn blue_bounce_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::ReturnToHand { target: ctx.source },
        Effect::DrawCards { player: *p, count: 1 },
    ]
}
