//! Pious Kitsune — `{2}{W}` 1/2 Fox Cleric.
//!
//! Oracle:
//! * At the beginning of your upkeep, put a devotion counter on this creature.
//!   Then if a creature named Eight-and-a-Half-Tails is on the battlefield, you
//!   gain 1 life for each devotion counter on this creature.
//! * {T}, Remove a devotion counter from this creature: You gain 1 life.
//!
//! "Devotion counter" is a named counter. The upkeep trigger adds one devotion
//! counter, then (resolution-time) checks for a battlefield creature named
//! Eight-and-a-Half-Tails and, if present, gains life equal to the devotion
//! counters this creature will have (current + the one being added). The
//! activated ability removes a devotion counter as a cost to gain 1 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pious Kitsune");
    let fox = reg.interner_mut().intern("Fox");
    let cleric = reg.interner_mut().intern("Cleric");
    let _devotion = reg.interner_mut().intern("devotion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let devotion = reg.interner().lookup("devotion").unwrap_or_default();

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_devotion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a devotion counter from this creature: You gain 1 life.".into(),
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(devotion), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_one,
            }),
    )
}

fn upkeep_devotion(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let devotion = match reg.interner().lookup("devotion") {
        Some(d) => d,
        None => return Vec::new(),
    };
    let mut effects = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(devotion),
        count: 1,
    }];

    // "Then if a creature named Eight-and-a-Half-Tails is on the battlefield,
    // you gain 1 life for each devotion counter on this creature." The count
    // is taken after the new counter is added, so use current + 1.
    if let Some(nm) = reg.interner().lookup("Eight-and-a-Half-Tails") {
        let filter = ObjectFilter { name: Some(nm), ..ObjectFilter::default() };
        let on_bf = script::count_matching(state, &filter, trig.controller);
        if on_bf > 0 {
            let current = state
                .objects
                .get(trig.source)
                .map_or(0, |o| o.count_counters(CounterKind::Named(devotion)));
            effects.push(Effect::GainLife {
                player: trig.controller,
                amount: current + 1,
            });
        }
    }
    effects
}

fn gain_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 1 }]
}
