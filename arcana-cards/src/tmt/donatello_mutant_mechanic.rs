//! Donatello, Mutant Mechanic — `{3}{U}` 3/5 Legendary Mutant Ninja Turtle.
//!
//! Oracle:
//! * {T}: Put three +1/+1 counters on target artifact you control. If it isn't
//!   a creature, it becomes a 0/0 Robot creature in addition to its other types.
//!   Activate only as a sorcery.
//! * Whenever an artifact you control is put into a graveyard from the
//!   battlefield, if it had counters on it, put those counters on up to one
//!   target artifact or creature you control.
//!
//! GAP: the "if it isn't a creature, it becomes a 0/0 Robot creature" clause is
//! conditional on the target not already being a creature — `Condition` has no
//! "is/isn't a creature" variant — so the animate half is dropped; the three
//! +1/+1 counters (the always-applied half) are wired.
//! GAP: the death trigger moves "those counters it had" onto a new target; the
//! dying object's counter count is not an available accessor and there is no
//! move-counters-from-a-gone-object primitive, so its effect is GAP'd whole.

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
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Donatello, Mutant Mechanic");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put three +1/+1 counters on target artifact you control. If it isn't a creature, it becomes a 0/0 Robot creature in addition to its other types. Activate only as a sorcery.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_three_counters,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: move_counters_on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn add_three_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "if it isn't a creature, it becomes a 0/0 Robot creature" — no
    // is-not-a-creature condition; only the unconditional counter half is wired.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}

fn move_counters_on_death(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it had counters on it, put those counters on up to one target" —
    // the dying artifact's counter inventory is not an accessible accessor and
    // there is no move-counters-from-a-gone-object primitive.
    Vec::new()
}
