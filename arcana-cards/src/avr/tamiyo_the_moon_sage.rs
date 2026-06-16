//! Tamiyo, the Moon Sage — `{3}{U}{U}` legendary planeswalker, starting
//! loyalty 4.
//!
//! Oracle:
//! * `+1`: Tap target permanent. It doesn't untap during its
//!   controller's next untap step.
//! * `−2`: Draw a card for each tapped creature target player controls.
//! * `−8`: You get an emblem with "You have no maximum hand size" and
//!   "Whenever a card is put into your graveyard from anywhere, you may
//!   return it to your hand."
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, controller
//!   only, once per turn per planeswalker.
//! * CR 704.5i — 0 loyalty → graveyard SBA.
//!
//! # Scope
//!
//! * `+1`: the tap is modeled via `Effect::Tap`. The "doesn't untap
//!   during its controller's next untap step" rider has no demonstrated
//!   `Effect` variant and is omitted (the tap itself is faithful).
//! * `−2`: fully modeled — `script::count_matching` counts tapped
//!   creatures the chosen player controls and that many cards are drawn.
//! * `−8`: emblem creation is not expressible from the demonstrated
//!   surface — GAP'd shell (correct loyalty cost, empty effect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Tamiyo, the Moon Sage");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Tap target permanent. It doesn't untap during \
                       its controller's next untap step."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Permanent(
                        ObjectFilter::permanent(),
                    ),
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_tap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Draw a card for each tapped creature target \
                       player controls."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"You have no maximum \
                       hand size\" and \"Whenever a card is put into your \
                       graveyard from anywhere, you may return it to your \
                       hand.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+1: Tap target permanent.` (untap-restriction rider not modeled.)
fn plus_one_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Tap { target: *id }]
}

/// `−2: Draw a card for each tapped creature target player controls.`
fn minus_two_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::Player(*p))
        .tapped_only();
    let count = script::count_matching(state, &filter, *p);
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: ctx.controller,
        count,
    }]
}

/// `−8` ultimate — emblem creation.
fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a static hand-size grant + a graveyard-return
    // triggered ability is not expressible from the demonstrated Effect
    // surface.
    Vec::new()
}
