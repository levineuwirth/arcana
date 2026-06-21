//! Arwen, Mortal Queen — `{1}{G}{W}` 2/2 Legendary Elf Noble.
//!
//! * Arwen enters with an indestructible counter on it.
//! * `{1}, Remove an indestructible counter from Arwen`: Another target
//!   creature gains indestructible until end of turn. Put a +1/+1 counter
//!   and a lifelink counter on that creature and a +1/+1 counter and a
//!   lifelink counter on Arwen.
//!
//! The "indestructible counter" and "lifelink counter" are keyword
//! counters modeled as named counters; their static ability grants are a
//! known engine gap (the counters themselves are tracked / consumable).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arwen, Mortal Queen");
    let elf = reg.interner_mut().intern("Elf");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(noble);
    // Pre-intern the keyword-counter names so the resolver lookups succeed.
    let _ = reg.interner_mut().intern("indestructible");
    let _ = reg.interner_mut().intern("lifelink");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let indestructible_counter =
        CounterKind::Named(reg.interner_mut().intern("indestructible"));

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_indestructible_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove an indestructible counter from Arwen: Another target creature gains indestructible until end of turn. Put a +1/+1 counter and a lifelink counter on that creature and a +1/+1 counter and a lifelink counter on Arwen."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    remove_self_counter: Some((indestructible_counter, 1)),
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
                effect: bestow_counters,
            }),
    )
}

/// ETB: place one indestructible (named) counter on Arwen.
fn etb_indestructible_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: trig.source, kind, count: 1 }]
}

/// Activated: grant indestructible EOT to the target and distribute +1/+1
/// and lifelink counters to both the target and Arwen.
fn bestow_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let target = *id;
    let lifelink = match reg.interner().lookup("lifelink").map(CounterKind::Named) {
        Some(k) => k,
        None => return Vec::new(),
    };
    vec![Effect::Sequence(vec![
        Effect::GrantKeyword {
            target,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::AddCounters { target, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::AddCounters { target, kind: lifelink, count: 1 },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::AddCounters { target: ctx.source, kind: lifelink, count: 1 },
    ])]
}
