//! Nissa of Shadowed Boughs — `{2}{B}{G}` Legendary Planeswalker — Nissa.
//! Printed starting loyalty 3 (CR 113.3c). Colors B/G.
//!
//! Landfall trigger (CR 603): "Whenever a land you control enters, put a
//! loyalty counter on Nissa." — a triggered ability (not a loyalty
//! ability); expressible via a land-enters `ZoneChange` trigger that adds
//! one Loyalty counter to the source.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Untap target land you control. You may have it become a 3/3
//!   Elemental creature with haste and menace until end of turn (still a
//!   land). — the untap is emitted; the "may become a 3/3 creature"
//!   rider is GAP'd (optional become-a-creature-with-keywords overlay is
//!   beyond the demonstrated surface for this resolver).
//! * `−5`: You may put a creature card with mana value ≤ the number of
//!   lands you control onto the battlefield from your hand or graveyard
//!   with two +1/+1 counters on it. — GAP (cross-zone reanimation with a
//!   dynamic-MV filter and enter-with-counters has no single primitive;
//!   PutFromHandOntoBattlefield / Reanimate don't combine the two zones
//!   plus the counters).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa of Shadowed Boughs");
    let nissa = reg.interner_mut().intern("Nissa");
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target land you control. You may have it become a \
                       3/3 Elemental creature with haste and menace until end of \
                       turn. It's still a land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: You may put a creature card with mana value less than or \
                       equal to the number of lands you control onto the \
                       battlefield from your hand or graveyard with two +1/+1 \
                       counters on it.".into(),
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
                effect: minus_five_reanimate,
            }),
    )
}

/// Landfall: put a loyalty counter on Nissa.
fn landfall_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

/// `+1`: untap target land; the may-become-a-creature rider is a GAP.
fn plus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "you may have it become a 3/3 Elemental with haste and menace,
    // still a land" — optional become-a-creature overlay isn't built from
    // this resolver. The untap is emitted.
    vec![Effect::Untap { target: *id }]
}

/// `−5`: cross-zone reanimation with dynamic-MV filter + counters — GAP.
fn minus_five_reanimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "from your hand OR graveyard, MV ≤ lands you control, enters
    // with two +1/+1 counters" — no single primitive spans both zones with
    // a dynamic filter and enter-with-counters.
    Vec::new()
}
