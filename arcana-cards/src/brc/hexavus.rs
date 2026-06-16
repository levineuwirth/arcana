//! Hexavus — `{6}` 0/0 Artifact Creature — Construct with Flying.
//! "This creature enters with six +1/+1 counters on it."
//! "{1}, Remove a +1/+1 counter from this creature: Put a flying counter
//!  on another target creature."
//! "{1}, Remove a counter from another creature you control: Put a
//!  +1/+1 counter on this creature."
//!
//! Flying is a base keyword. "Enters with six +1/+1 counters" is modeled
//! as an ETB trigger adding six +1/+1 counters to itself. The first
//! activated ability removes a +1/+1 counter (cost) to put a flying
//! counter (`Named("flying")`) on a target creature. The second
//! activated ability is GAP'd: "Remove a counter from another creature
//! you control" has no remove-counter-from-other cost field.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hexavus");
    let construct = reg.interner_mut().intern("Construct");
    let _flying_counter = reg.interner_mut().intern("flying");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove a +1/+1 counter from this creature: Put a flying counter on another target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_flying_counter,
            }),
        // GAP: "{1}, Remove a counter from another creature you control:
        // Put a +1/+1 counter on this creature." — no cost field for
        // removing a counter from another (chosen) creature. Omitted.
    )
}

fn enter_with_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 6,
    }]
}

fn put_flying_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let flying = match reg.interner().lookup("flying") {
        Some(s) => s,
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(flying),
        count: 1,
    }]
}
