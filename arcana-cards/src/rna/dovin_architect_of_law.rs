//! Dovin, Architect of Law — `{4}{W}{U}` Legendary Planeswalker — Dovin,
//! starting loyalty 5 — colors U, W.
//!
//! Oracle text:
//! * `+1`: You gain 2 life and draw a card. — `GainLife` + `DrawCards`.
//! * `−1`: Tap target creature. It doesn't untap during its controller's next
//!   untap step. — `Tap` is faithful; the "doesn't untap next untap step"
//!   rider is GAP'd.
//! * `−9`: Tap all permanents target opponent controls. That player skips
//!   their next untap step. — tap-all via `ForEach` over that player's
//!   permanents; the skip-untap rider is GAP'd.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dovin, Architect of Law");
    let dovin = reg.interner_mut().intern("Dovin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dovin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 2 life and draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_gain_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Tap target creature. It doesn't untap during its \
                       controller's next untap step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_tap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: Tap all permanents target opponent controls. That \
                       player skips their next untap step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_tap_all,
            }),
    )
}

/// `+1`: gain 2 life and draw a card.
fn plus_one_gain_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::GainLife { player: ctx.controller, amount: 2 },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ])]
}

/// `−1`: tap target creature (skip-untap rider GAP'd).
fn minus_one_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it doesn't untap during its controller's next untap step" is not
    // expressible; the tap is faithful.
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::Tap { target }]
}

/// `−9`: tap all permanents the target opponent controls (skip-untap GAP'd).
fn minus_nine_tap_all(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player skips their next untap step" is not expressible; the
    // tap-all is faithful.
    let player = match ctx.targets.targets.first() {
        Some(TargetChoice::Player(p)) => *p,
        _ => return Vec::new(),
    };
    let filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::Player(player));
    let ids = script::ids_matching(state, &filter, player);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
    }]
}
