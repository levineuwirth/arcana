//! Teferi, Temporal Pilgrim — `{3}{U}{U}` Legendary Planeswalker — Teferi,
//! starting loyalty 5. Mono-blue.
//!
//! Oracle:
//! Whenever you draw a card, put a loyalty counter on Teferi.
//! 0: Draw a card.
//! −2: Create a 2/2 blue Spirit creature token with vigilance and "Whenever
//!     you draw a card, put a +1/+1 counter on this token."
//! −12: Target opponent chooses a permanent they control and returns it to its
//!      owner's hand. Then they shuffle each nonland permanent they control
//!      into its owner's library.
//!
//! # Scope
//! * Static draw trigger — modeled: CardDrawn (you) → AddCounters Loyalty on
//!   Teferi.
//! * 0 — modeled: draw a card.
//! * −2 — modeled: create the Spirit token, including its own CardDrawn →
//!   +1/+1 internal triggered ability.
//! * −12 — GAP: opponent-chooses-then-shuffle bespoke one-shot not expressible.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Temporal Pilgrim");
    let sub = reg.interner_mut().intern("Teferi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    // Pre-intern the token's name/subtype so the effect fn can look them up.
    let _ = reg.interner_mut().intern("Spirit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_draw_add_loyalty,
                trigger_zones: vec![],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Draw a card.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 2/2 blue Spirit creature token with \
                       vigilance and \"Whenever you draw a card, put a +1/+1 \
                       counter on this token.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−12: Target opponent chooses a permanent they control \
                       and returns it to its owner's hand. Then they shuffle \
                       each nonland permanent they control into its owner's \
                       library.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 12)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_twelve_gap,
            }),
    )
}

/// Static: "Whenever you draw a card, put a loyalty counter on Teferi."
fn on_draw_add_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

/// `0: Draw a card.`
fn zero_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

/// `−2:` create the Spirit token.
fn minus_two_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit_name = reg.interner().lookup("Spirit").unwrap_or_default();
    let spirit_sub = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut spirit_subtypes = SubtypeSet::default();
    spirit_subtypes.0.insert(spirit_sub);
    let token = TokenDefinition {
        name: spirit_name,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: spirit_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: token_draw_counter,
            trigger_zones: vec![],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![],
        }],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}

/// The Spirit token's "Whenever you draw a card, put a +1/+1 counter on this
/// token."
fn token_draw_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

/// `−12` — GAP: opponent-chooses-then-shuffle bespoke one-shot.
fn minus_twelve_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: target opponent chooses + shuffle-each-nonland is not expressible.
    Vec::new()
}
