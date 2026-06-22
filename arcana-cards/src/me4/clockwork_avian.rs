//! Clockwork Avian — `{5}` 0/4 Artifact Creature — Bird with Flying.
//! "Enters with four +1/+0 counters. At end of combat, if it attacked or
//! blocked this combat, remove a +1/+0 counter from it. {X}, {T}: Put up to X
//! +1/+0 counters on it (max four total); activate only during your upkeep."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clockwork Avian");
    let bird = reg.interner_mut().intern("Bird");
    // "+1/+0" has no dedicated CounterKind variant; intern a named counter so
    // the add/remove bookkeeping is faithful (the P/T modifier such a counter
    // would carry is an engine fidelity gap for named counters).
    let _p1p0 = reg.interner_mut().intern("p1p0");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_four_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP (intervening-if): "if this creature attacked or blocked
                // this combat" has no conditions:: helper, so it's omitted and
                // the removal fires each end of combat — a documented over-fire.
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::EndCombat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: end_combat_remove_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (timing): "Activate only during your upkeep" has no
                // activation-window field and is omitted.
                text: "{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than four.".into(),
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
                effect: add_up_to_x_counters,
            }),
    )
}

fn etb_four_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("p1p0").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: trig.source, kind, count: 4 }]
}

fn end_combat_remove_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("p1p0").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::RemoveCounters { target: trig.source, kind, count: 1 }]
}

fn add_up_to_x_counters(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("p1p0").map(CounterKind::Named) else {
        return Vec::new();
    };
    let current = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(kind));
    let x = ctx.x_value.unwrap_or(0);
    // Cap the total at four +1/+0 counters.
    let allowed = 4u32.saturating_sub(current);
    let n = x.min(allowed);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters { target: ctx.source, kind, count: n }]
}
