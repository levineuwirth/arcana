//! Emiel the Blessed — `{2}{W}{W}` 4/4 Legendary Unicorn.
//!
//! Oracle:
//! * "{3}: Exile another target creature you control, then return it to the
//!   battlefield under its owner's control."
//! * "Whenever another creature you control enters, you may pay {G/W}. If you
//!   do, put a +1/+1 counter on it. If it's a Unicorn, put two +1/+1 counters
//!   on it instead."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emiel the Blessed");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}: Exile another target creature you control, then return it to the battlefield under its owner's control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blink_creature,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: maybe_counter_entering,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blink_creature(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile … then return it to the battlefield" is an atomic flicker; the
    // engine re-ids the object on the exile zone-move, so a Sequence of
    // ExilePermanent + ReturnFromExileToBattlefield cannot reference the same
    // (now-stale) id, and there is no single-effect flicker primitive. Exiling
    // without the return would permanently remove your own creature (worse than
    // a no-op), so the whole effect is GAP'd.
    Vec::new()
}

fn maybe_counter_entering(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    // "If it's a Unicorn, put two +1/+1 counters instead."
    let unicorn_filter = script::subtype_filter(reg, "Unicorn");
    let is_unicorn = script::ids_matching(state, &unicorn_filter, trig.controller).contains(&id);
    let count = if is_unicorn { 2 } else { 1 };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{G/W}").expect("valid cost")),
        then: Box::new(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count,
        }),
        else_effect: None,
    }]
}
