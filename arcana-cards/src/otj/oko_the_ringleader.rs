//! Oko, the Ringleader — `{2}{G}{U}` Legendary Planeswalker — Oko. Colors G, U.
//! Starting loyalty 4.
//!
//! At the beginning of combat on your turn, Oko becomes a copy of up to one
//!   target creature you control until end of turn, except he has hexproof.
//! +1: Draw two cards. If you've committed a crime this turn, discard a card.
//!   Otherwise, discard two cards.
//! −1: Create a 3/3 green Elk creature token.
//! −5: For each other nonland permanent you control, create a token that's a
//!   copy of that permanent.
//!
//! GAP: the begin-combat trigger ("becomes a copy of a creature you control,
//!   except it has hexproof") is a bespoke copy-self effect not expressible
//!   from the demonstrated surface; the trigger is declared with an empty
//!   effect.
//! GAP: the +1 conditional discard depends on "committed a crime this turn"
//!   crime tracking; the draw is emitted, the conditional discard is omitted.
//! GAP: the −1 "3/3 green Elk token" cannot be constructed from the
//!   demonstrated token APIs; declared with its correct cost, effect empty.
//! GAP: the −5 "copy each nonland permanent you control" token-copy effect is
//!   not expressible; declared with its correct cost, effect empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oko, the Ringleader");
    let oko = reg.interner_mut().intern("Oko");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(oko);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of combat on your turn, Oko becomes a copy ... (GAP'd).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_become_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // +1: Draw two cards (conditional discard GAP'd).
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw two cards. If you've committed a crime this turn, discard a card. Otherwise, discard two cards.".into(),
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
                effect: plus_one_draw,
            })
            // −1: Create a 3/3 green Elk creature token (GAP'd).
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Create a 3/3 green Elk creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_elk,
            })
            // −5: Copy each other nonland permanent you control (GAP'd).
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: For each other nonland permanent you control, create a token that's a copy of that permanent.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_copy,
            }),
    )
}

/// Begin-combat: Oko becomes a copy of a creature you control (GAP'd).
fn combat_become_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a copy of up to one target creature you control, except it
    // has hexproof" is a bespoke copy-self effect not expressible here.
    Vec::new()
}

/// `+1: Draw two cards. (conditional discard GAP'd)`
fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the conditional discard depends on "committed a crime this turn"
    // crime tracking, not expressible; emitting the unconditional draw two.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 2,
    }]
}

/// `-1: Create a 3/3 green Elk creature token.`
fn minus_one_elk(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot construct a 3/3 green Elk creature token from the
    // demonstrated token APIs.
    Vec::new()
}

/// `-5: For each other nonland permanent you control, copy it.`
fn minus_five_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token-copy of each nonland permanent is not expressible.
    Vec::new()
}
