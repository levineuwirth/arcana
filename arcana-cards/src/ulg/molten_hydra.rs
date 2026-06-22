//! Molten Hydra — `{1}{R}` 1/1 Hydra.
//! {1}{R}{R}: Put a +1/+1 counter on this creature.
//! {T}, Remove all +1/+1 counters from this creature: It deals damage to any
//! target equal to the number of +1/+1 counters removed this way.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Molten Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{R}: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove all +1/+1 counters from this creature: It deals damage to any target equal to the number of +1/+1 counters removed this way.".into(),
                // GAP (cost fidelity): "Remove all +1/+1 counters" as an
                // activation cost is not expressible (remove_self_counter is a
                // fixed count). Modeled as a {T} ability that, at resolution,
                // deals damage equal to the current +1/+1 counters and then
                // removes them.
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: remove_counters_deal_damage,
            }),
    )
}

fn add_counter(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn remove_counters_deal_damage(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let n = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    vec![Effect::Sequence(vec![
        Effect::RemoveCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: n,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: dt,
            amount: n,
        },
    ])]
}
