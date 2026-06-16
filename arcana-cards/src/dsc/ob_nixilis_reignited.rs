//! Ob Nixilis Reignited — `{3}{B}{B}` Legendary Planeswalker — Nixilis,
//! starting loyalty 5. Black.
//!
//! Oracle text:
//! * `+1`: You draw a card and you lose 1 life.
//! * `−3`: Destroy target creature.
//! * `−8`: Target opponent gets an emblem with "Whenever a player draws
//!   a card, you lose 2 life."
//!
//! # Scope
//!
//! * `+1` (draw a card + lose 1 life) and `−3` (destroy target
//!   creature) are fully expressible.
//! * `−8` grants an emblem to an opponent; emblem creation with a
//!   triggered draw-punish ability targeted at an opponent is bespoke —
//!   GAP'd, cost shell declared.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ob Nixilis Reignited");
    let nixilis = reg.interner_mut().intern("Nixilis");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nixilis);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
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
                text: "+1: You draw a card and you lose 1 life.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Target opponent gets an emblem with \"Whenever a \
                       player draws a card, you lose 2 life.\"".into(),
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
                effect: minus_eight,
            }),
    )
}

/// `+1`: draw a card and lose 1 life.
fn plus_one(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}

/// `−3`: destroy target creature.
fn minus_three(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−8`: target opponent gets a draw-punish emblem.
fn minus_eight(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem granted to an opponent carrying a triggered "whenever a
    // player draws, you lose 2 life" ability is bespoke / not expressible
    // from the demonstrated emblem surface.
    Vec::new()
}
