//! Baylen, the Haymaker — `{R}{G}{W}` 4/3 Legendary Rabbit Warrior.
//!
//! Tap two untapped tokens you control: Add one mana of any color.
//! Tap three untapped tokens you control: Draw a card.
//! Tap four untapped tokens you control: Put three +1/+1 counters on Baylen.
//! It gains trample until end of turn.
//!
//! The tap-N-tokens costs are modeled with `tap_other` + `tap_other_count`.
//! The first ability's "add one mana of any color" payload is modeled as five
//! mana abilities, one per WUBRG color, each costing tap-two-tokens; the player
//! picks the color by choosing which ability to activate (command_tower idiom).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baylen, the Haymaker");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let tokens = || ObjectFilter::permanent().controlled_by(ControllerConstraint::You).tokens_only();

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(tap_two_mana("Tap two untapped tokens you control: Add {W}.", tokens(), add_white))
            .with_activated_ability(tap_two_mana("Tap two untapped tokens you control: Add {U}.", tokens(), add_blue))
            .with_activated_ability(tap_two_mana("Tap two untapped tokens you control: Add {B}.", tokens(), add_black))
            .with_activated_ability(tap_two_mana("Tap two untapped tokens you control: Add {R}.", tokens(), add_red))
            .with_activated_ability(tap_two_mana("Tap two untapped tokens you control: Add {G}.", tokens(), add_green))
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap three untapped tokens you control: Draw a card.".into(),
                cost: ActivationCost {
                    tap_other: Some(tokens()),
                    tap_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap four untapped tokens you control: Put three +1/+1 counters on Baylen. It gains trample until end of turn.".into(),
                cost: ActivationCost {
                    tap_other: Some(tokens()),
                    tap_other_count: 4,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counters_and_trample,
            }),
    )
}

/// One "Tap two untapped tokens you control: Add {C}." mana ability — the
/// shared tap-two-tokens cost means only one of the five fires.
fn tap_two_mana(
    text: &str,
    tokens: ObjectFilter,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            tap_other: Some(tokens),
            tap_other_count: 2,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn counters_and_trample(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 3,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}
