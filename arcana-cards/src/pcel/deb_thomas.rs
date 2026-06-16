//! Deb Thomas — `{3}{R}` Legendary Planeswalker — Deb, starting loyalty 4.
//!
//! Static/triggered:
//! * "Whenever an Employee enters under your control, put a loyalty counter on
//!   Deb Thomas." — modeled as a `ZoneChange` trigger (any zone → battlefield)
//!   filtered to Employee subtype controlled by you, putting a Loyalty counter
//!   on the source.
//!
//! Loyalty abilities:
//! * `+1`: Create a 1/1 red Employee creature token.
//! * `−X`: Employees and Dogs you control get +X/+0 until end of turn. GAP —
//!   dynamic-X loyalty cost is not expressible (`remove_self_counter` is a
//!   fixed u32). Ability omitted.

use arcana_core::effects::Effect;
use arcana_core::effects::TokenDefinition;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deb Thomas");
    let deb = reg.interner_mut().intern("Deb");
    let _employee = reg.interner_mut().intern("Employee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(deb);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let employee_sym = reg.interner().lookup("Employee")
        .expect("Employee interned during register()");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_subtype_sym(employee_sym)
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: add_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 red Employee creature token.".into(),
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
                effect: plus_one_token,
            }),
    )
}

/// "Whenever an Employee enters under your control, put a loyalty counter on
/// Deb Thomas."
fn add_loyalty(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

/// `+1: Create a 1/1 red Employee creature token.`
fn plus_one_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let employee = reg.interner().lookup("Employee")
        .expect("Employee interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(employee);
    let token = TokenDefinition {
        name: employee,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
