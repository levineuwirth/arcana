//! Ral Zarek, Guest Lecturer — `{1}{B}{B}` Legendary Planeswalker — Ral,
//! starting loyalty 5.
//!
//! Oracle text:
//! * `+1`: Surveil 2.
//! * `−1`: Any number of target players each discard a card.
//! * `−2`: Return target creature card with mana value 3 or less from
//!   your graveyard to the battlefield.
//! * `−7`: Flip five coins. Target opponent skips their next X turns,
//!   where X is the number of coins that came up heads.
//!
//! # Rules references
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//!
//! # Scope
//! * `+1` Surveil 2 — implemented.
//! * `−1` any-number-of-target-players each discard — implemented.
//! * `−2` reanimate from your graveyard — GAP: targeting a card in a
//!   graveyard needs a concrete `Zone::Graveyard(player)` and there is
//!   no any-graveyard sentinel; the register fn has no controller
//!   PlayerId, so the own-graveyard zone can't be built. Shell only.
//! * `−7` flip-five-coins / skip-turns — GAP: skipping turns is not
//!   expressible. Shell only.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral Zarek, Guest Lecturer");
    let ral = reg.interner_mut().intern("Ral");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ral);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
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
                text: "+1: Surveil 2.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_surveil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Any number of target players each discard a \
                       card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Any,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Return target creature card with mana value 3 \
                       or less from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Flip five coins. Target opponent skips their \
                       next X turns, where X is the number of coins that \
                       came up heads.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+1: Surveil 2.`
fn plus_one_surveil(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil { player: ctx.controller, count: 2 }]
}

/// `−1: Any number of target players each discard a card.`
fn minus_one_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Player(p) = target {
            effects.push(Effect::Discard {
                player: *p,
                count: 1,
                choice: DiscardChoice::ControllerChooses,
            });
        }
    }
    effects
}

fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: targeting a creature card in your graveyard needs a concrete
    // Zone::Graveyard(player); no any-graveyard sentinel and no
    // controller PlayerId at register time.
    Vec::new()
}

fn minus_seven(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: flip five coins + make a player skip their next X turns is
    // not expressible (turn-skipping has no Effect surface).
    Vec::new()
}
