//! Guide of Souls — `{W}` 1/2 Human Cleric.
//! "Whenever another creature you control enters, you gain 1 life and
//!  get {E} (an energy counter).
//!  Whenever you attack, you may pay {E}{E}{E}. When you do, put two
//!  +1/+1 counters and a flying counter on target attacking creature.
//!  It becomes an Angel in addition to its other types."
//!
//! The {E}{E}{E} payment is modeled with Effect::OptionalPayment {
//! cost: Energy(3) } gating the +1/+1 + flying-counter buff (the target
//! attacking creature is chosen as the trigger's target up front; the buff
//! only lands if you pay). Residual GAP: "becomes an Angel" (subtype add).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guide of Souls");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let _flying = reg.interner_mut().intern("flying");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP (fidelity): "another" self-exclusion not expressible in the filter.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_life_and_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: buff_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    // "target attacking creature" — restrict to attackers.
                    filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn gain_life_and_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
        Effect::GainEnergy {
            player: trig.controller,
            amount: 1,
        },
    ]
}

fn buff_attacker(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let id = *id;
    let flying = reg.interner().lookup("flying").unwrap_or_default();
    // "you may pay {E}{E}{E}. When you do, put two +1/+1 counters and a flying
    // counter on [it]." Gate the buff on paying 3 energy.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Energy(3),
        then: Box::new(Effect::Sequence(vec![
            Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 2 },
            // GAP: "becomes an Angel" is a subtype addition; AddType only adds
            // card TYPES, not creature subtypes.
            Effect::AddCounters { target: id, kind: CounterKind::Named(flying), count: 1 },
        ])),
        else_effect: None,
    }]
}
