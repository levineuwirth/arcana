//! Clockwork Beast — `{6}` 0/4 Artifact Beast. Enters with seven +1/+0
//! counters; at end of combat (if it attacked/blocked) removes one; `{X}, {T}:
//! put up to X +1/+0 counters on it (max seven total), only during upkeep`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clockwork Beast");
    let beast = reg.interner_mut().intern("Beast");
    let _plus_one_zero = reg.interner_mut().intern("+1/+0");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "enters with seven +1/+0 counters" is a static enters-with clause,
    // not a triggered/activated ability — not expressible in this shape (and
    // there is no +1/+0 CounterKind variant; only Named).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::EndCombat,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if this creature attacked or blocked this
                // combat" — no expressible condition predicate for combat
                // participation; fires unconditionally instead.
                intervening_if: None,
                effect: remove_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Put up to X +1/+0 counters on this creature. Activate only during your upkeep.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_x_counters,
            }),
    )
    // GAP: "can't exceed seven total" cap and "only during your upkeep" timing
    // restriction are not expressible.
}

fn remove_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("+1/+0").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn add_x_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = ctx.x_value.unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let Some(kind) = reg.interner().lookup("+1/+0").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind,
        count: n,
    }]
}
