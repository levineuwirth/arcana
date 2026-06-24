//! Anthroplasm — `{2}{U}{U}` 0/0 Shapeshifter.
//! "This creature enters with two +1/+1 counters on it."
//! "{X}, {T}: Remove all +1/+1 counters from this creature and put X +1/+1
//!  counters on it." — wired as an `{X}, {T}` activated ability. X is read
//!  from `ctx.x_value`; "remove all" reads the source's current +1/+1 count
//!  from state and removes exactly that many, then adds X.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anthroplasm");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "enters with two +1/+1 counters" — modeled as a self-ETB trigger
            // that adds the counters.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_with_two_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Remove all +1/+1 counters from this creature \
                       and put X +1/+1 counters on it."
                    .into(),
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
                effect: reset_counters_to_x,
            }),
    )
}

fn enters_with_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

/// "Remove all +1/+1 counters from this creature, then put X +1/+1 counters
/// on it." Reads the source's current +1/+1 count from state for the
/// remove-all step; X comes from `ctx.x_value`.
fn reset_counters_to_x(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let existing = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let mut effects = Vec::new();
    if existing > 0 {
        effects.push(Effect::RemoveCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: existing,
        });
    }
    if x > 0 {
        effects.push(Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: x,
        });
    }
    effects
}
