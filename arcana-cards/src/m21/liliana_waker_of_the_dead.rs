//! Liliana, Waker of the Dead — `{2}{B}{B}` Legendary Planeswalker — Liliana, starting loyalty 5.
//!
//! +1: Each player discards a card. Each opponent who can't loses 3 life.
//! −3: Target creature gets -X/-X until end of turn, where X is the number of
//!     cards in your graveyard.
//! −7: You get an emblem with "At the beginning of combat on your turn, put
//!     target creature card from a graveyard onto the battlefield under your
//!     control. It gains haste."
//!
//! # Scope
//! - `+1`: "each player discards a card; each opponent who can't loses 3 life"
//!   combines an each-player discard with a per-opponent conditional life-loss
//!   that the demonstrated single-`player` Discard / LoseLife surface can't
//!   express — ability shell with correct `+1` cost, GAP'd body.
//! - `−3`: "-X/-X where X is cards in your graveyard" is a dynamic-X pump — not
//!   expressible (Pump power/toughness are fixed) — ability shell with correct
//!   `−3` cost + `target creature` requirement, GAP'd body.
//! - `−7`: EMBLEM with a triggered ability ("At the beginning of combat on your
//!   turn, put target creature card from a graveyard onto the battlefield ...
//!   gains haste") — the emblem and its StepBegins(BeginCombat, You) trigger
//!   SHELL are created; the graveyard-targeting reanimation + haste body is
//!   GAP'd (no any-graveyard target sentinel in the demonstrated surface).

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, Waker of the Dead");
    let liliana = reg.interner_mut().intern("Liliana");
    let _emblem = reg.interner_mut().intern("Liliana, Waker of the Dead emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each player discards a card. Each opponent who can't \
                       loses 3 life."
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
                effect: plus_one_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature gets -X/-X until end of turn, where X \
                       is the number of cards in your graveyard."
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
                effect: minus_three_minus_x,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"At the beginning of combat on \
                       your turn, put target creature card from a graveyard onto \
                       the battlefield under your control. It gains haste.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1: Each player discards a card. Each opponent who can't loses 3 life.`
fn plus_one_discard(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each player discards a card" (each-player iteration) combined with
    // "each opponent who can't loses 3 life" (per-opponent conditional life
    // loss) is not expressible with the single-player Discard / LoseLife surface.
    Vec::new()
}

/// `−3: Target creature gets -X/-X until end of turn (X = cards in your gy).`
fn minus_three_minus_x(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic-X pump (-X/-X where X = cards in your graveyard); Pump's
    // power/toughness are fixed and cannot read the graveyard size.
    Vec::new()
}

/// `−7: You get an emblem with a beginning-of-combat reanimation trigger.`
fn minus_seven_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Liliana, Waker of the Dead emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_reanimate,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

/// Emblem: "put target creature card from a graveyard onto the battlefield under
/// your control. It gains haste."
fn emblem_reanimate(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: target a creature card in ANY graveyard and reanimate it under your
    // control with haste — there is no any-graveyard target sentinel in the
    // demonstrated surface, and the per-controller reanimate-with-haste rider
    // isn't expressible here. The StepBegins trigger shell still fires.
    Vec::new()
}
