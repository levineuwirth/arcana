//! Clockwork Swarm — `{4}` 0/3 colorless Artifact Creature — Insect.
//!
//! This creature enters with four +1/+0 counters on it.
//! This creature can't be blocked by Walls.
//! At end of combat, if this creature attacked or blocked this combat, remove a
//! +1/+0 counter from it.
//! {X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't
//! cause the total number of +1/+0 counters on this creature to be greater than
//! four. Activate only during your upkeep.
//!
//! NOTE: "+1/+0" counters have no dedicated `CounterKind` variant and the
//! engine's Named counters carry no P/T semantics, so throughout this card the
//! +1/+0 counters are tracked as `CounterKind::Named("+1/+0")` for their COUNT
//! only — they do NOT grant +1/+0 to power (a documented fidelity GAP).
//! - "enters with four +1/+0 counters" → ETB AddCounters ×4 (Named).
//! - "can't be blocked by Walls" → static evasion restriction; GAP'd.
//! - "at end of combat … remove a counter" → EndCombat trigger; the "if
//!   attacked or blocked" intervening-if has no helper, so it is GAP'd.
//! - "{X}, {T}: put up to X counters; cap at four; upkeep only" → X-cost tap
//!   activation adding X Named counters; the four-cap and upkeep-only timing
//!   restriction are GAP'd.

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
    let name = reg.interner_mut().intern("Clockwork Swarm");
    let insect = reg.interner_mut().intern("Insect");
    let _p1p0 = reg.interner_mut().intern("+1/+0");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: static "can't be blocked by Walls" — continuous evasion
        // restriction; not expressible as a triggered/activated ability.
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
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::EndCombat,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if this creature attacked or blocked this
                // combat" — no helper for a per-combat attacked/blocked flag.
                intervening_if: None,
                effect: end_combat_remove_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than four. Activate only during your upkeep.".into(),
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
}

fn etb_four_counters(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let kind = reg
        .interner()
        .lookup("+1/+0")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    // GAP (fidelity): Named "+1/+0" counters track count only, no P/T effect.
    vec![Effect::AddCounters { target: trig.source, kind, count: 4 }]
}

fn end_combat_remove_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = reg
        .interner()
        .lookup("+1/+0")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::RemoveCounters { target: trig.source, kind, count: 1 }]
}

fn add_x_counters(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    if x == 0 {
        return Vec::new();
    }
    let kind = reg
        .interner()
        .lookup("+1/+0")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    // GAP: the "no more than four total" cap and "activate only during your
    // upkeep" timing restriction are not expressible; X counters are added raw.
    vec![Effect::AddCounters { target: ctx.source, kind, count: x }]
}
