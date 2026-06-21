//! Myojin of Blooming Dawn — `{5}{W}{W}{W}` 4/6 Legendary Spirit.
//!
//! Myojin enters with an indestructible counter on it if you cast it from your
//! hand. (Modeled as an ETB trigger that adds an indestructible counter; the
//! "if you cast it from your hand" gate is a GAP — no cast-from-hand predicate
//! exists. We add the counter on ETB unconditionally as best-effort.)
//! Remove an indestructible counter from Myojin: Create a 1/1 colorless Spirit
//! creature token for each permanent you control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Blooming Dawn");
    let spirit = reg.interner_mut().intern("Spirit");
    let indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if you cast it from your hand" — no cast-from-hand
                // predicate is available for the intervening-if gate.
                intervening_if: None,
                effect: etb_indestructible_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove an indestructible counter from Myojin of Blooming Dawn: Create a 1/1 colorless Spirit creature token for each permanent you control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(indestructible), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_spirits,
            }),
    )
}

/// ETB: add an indestructible counter to Myojin.
fn etb_indestructible_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(indestructible) = reg.interner().lookup("indestructible") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(indestructible),
        count: 1,
    }]
}

/// Remove the counter: create a 1/1 colorless Spirit token for each permanent
/// you control.
fn make_spirits(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let Some(spirit) = reg.interner().lookup("Spirit") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let mut out = Vec::new();
    for _ in 0..n {
        out.push(Effect::CreateToken {
            controller: ctx.controller,
            token: arcana_core::effects::TokenDefinition {
                name: spirit,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    out
}
