//! Liliana of the Dark Realms — `{2}{B}{B}` Legendary Planeswalker — Liliana, loyalty 3.
//!
//! +1: Search your library for a Swamp card, reveal it, put it into your hand,
//!   then shuffle.
//! −3: Target creature gets +X/+X or -X/-X until end of turn, where X is the
//!   number of Swamps you control.
//! −6: You get an emblem with "Swamps you control have '{T}: Add {B}{B}{B}{B}.'"
//!
//! # Scope
//! GAP: the −3 player choice between +X/+X and -X/-X isn't expressible; the
//!   +X/+X branch is modeled (X = Swamps you control). The pump-down mode is
//!   the unmodeled half.
//! GAP: the −6 emblem ("Swamps you control have '{T}: Add {B}{B}{B}{B}'") grants
//!   a mana ability to a permanent class via an emblem — not expressible.
//!   Ability shell declared, effect empty.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana of the Dark Realms");
    let liliana = reg.interner_mut().intern("Liliana");
    let _swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Search your library for a Swamp card, reveal it, put it into your hand, then shuffle.".into(),
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
                effect: plus_one_tutor,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature gets +X/+X or -X/-X until end of turn, where X is the number of Swamps you control.".into(),
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
                effect: minus_three_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Swamps you control have '{T}: Add {B}{B}{B}{B}.'\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_gap,
            }),
    )
}

fn plus_one_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: script::subtype_filter(reg, "Swamp"),
        reveal: true,
    }]
}

fn minus_three_pump(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: player's choice of +X/+X vs -X/-X; the +X/+X branch is modeled.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Swamp").controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) as i32;
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn minus_six_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem granting a mana ability to all Swamps you control.
    Vec::new()
}
