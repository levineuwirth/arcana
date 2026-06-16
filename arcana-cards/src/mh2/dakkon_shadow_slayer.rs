//! Dakkon, Shadow Slayer — `{W}{U}{B}` Legendary Planeswalker — Dakkon.
//! Colors B, U, W.
//!
//! Dakkon enters with a number of loyalty counters on him equal to the number
//! of lands you control.
//!   GAP: dynamic starting loyalty (= lands you control) is not expressible
//!   via the static printed-loyalty field; printed loyalty is set to 0 (its
//!   printed value is a star/variable) and the ETB-counts-lands rider is not
//!   modeled.
//! +1: Surveil 2.
//! −3: Exile target creature.
//! −6: You may put an artifact card from your hand or graveyard onto the
//!     battlefield.
//!     GAP: a "from hand OR graveyard" optional put (two source zones with a
//!     player choice) is not expressible from the demonstrated surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dakkon, Shadow Slayer");
    let dakkon = reg.interner_mut().intern("Dakkon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dakkon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: printed loyalty is variable (= lands you control); modeled as 0.
        loyalty: Some(0),
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
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_surveil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Exile target creature.".into(),
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
                effect: minus_three_exile,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You may put an artifact card from your hand or \
                       graveyard onto the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_gap,
            }),
    )
}

fn plus_one_surveil(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: ctx.controller,
        count: 2,
    }]
}

fn minus_three_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

fn minus_six_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put an artifact from your hand OR graveyard onto the battlefield"
    //      — a two-zone optional put is not expressible.
    Vec::new()
}
