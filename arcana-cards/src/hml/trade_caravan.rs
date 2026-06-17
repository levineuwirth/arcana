//! Trade Caravan — `{W}` 1/1 Human Nomad.
//! "At the beginning of your upkeep, put a currency counter on this
//! creature." and "Remove two currency counters from this creature:
//! Untap target basic land. Activate only during an opponent's upkeep."
//!
//! Both abilities are wired. The "currency" counter is a named counter.
//! The activation's "only during an opponent's upkeep" timing window has
//! no expressible activation-condition predicate, so that restriction is
//! GAP'd (the untap and the two-counter cost are wired).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trade Caravan");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let currency = reg.interner_mut().intern("currency");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_currency,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove two currency counters from this creature: Untap target basic land. Activate only during an opponent's upkeep.".into(),
                // GAP: "Activate only during an opponent's upkeep." — no
                //      activation-timing-window condition; the timing restriction
                //      is omitted (the cost and untap are wired).
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(currency), 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        types: Some(TypeLine::LAND.into()),
                        supertypes: Some(SupertypeSet(SupertypeSet::BASIC)),
                        ..ObjectFilter::default()
                    }),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_land,
            }),
    )
}

fn add_currency(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let currency = reg.interner().lookup("currency").map(CounterKind::Named);
    let Some(kind) = currency else { return Vec::new(); };
    vec![Effect::AddCounters { target: trig.source, kind, count: 1 }]
}

fn untap_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Untap { target: *id }]
}
