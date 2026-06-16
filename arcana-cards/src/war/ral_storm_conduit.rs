//! Ral, Storm Conduit — `{2}{U}{R}` Legendary Planeswalker — Ral, starting
//! loyalty 4.
//!
//! Static/triggered:
//! * "Whenever you cast or copy an instant or sorcery spell, Ral deals 1 damage
//!   to target opponent or planeswalker." — modeled via a `SpellCast` trigger
//!   (filter: instant OR sorcery, caster = you) dealing 1 damage. The "or copy"
//!   half is not expressible (no copy-event trigger condition in the
//!   demonstrated surface) — partial fidelity, cast half wired.
//!
//! Loyalty abilities:
//! * `+2`: Scry 1.
//! * `−2`: When you next cast an instant or sorcery spell this turn, copy that
//!   spell (may choose new targets). GAP — the delayed "next cast → copy"
//!   rider is not expressible. Ability shell declared.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Storm Conduit");
    let ral = reg.interner_mut().intern("Ral");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ral);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: ping,
                trigger_zones: vec![arcana_core::zones::Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Scry 1.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_scry,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: When you next cast an instant or sorcery spell this \
                       turn, copy that spell. You may choose new targets for \
                       the copy.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            }),
    )
}

/// "Whenever you cast … an instant or sorcery spell, Ral deals 1 damage to
/// target opponent or planeswalker."
fn ping(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: trig.source, target: dt, amount: 1 }]
}

/// `+2: Scry 1.`
fn plus_two_scry(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry { player: ctx.controller, count: 1 }]
}

/// `−2`: delayed next-cast copy.
fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "when you next cast an instant or sorcery this turn, copy it" — the
    // delayed next-cast copy rider is not expressible here.
    Vec::new()
}
