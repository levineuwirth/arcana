//! A-Dueling Coach — `{3}{W}` 2/2 Human Monk.
//! "When Dueling Coach enters, distribute two +1/+1 counters among one
//! or two target creatures you control."
//! "{W}, {T}: Put a +1/+1 counter on each creature you control with a
//! +1/+1 counter on it."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Dueling Coach");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // ETB: distribute two +1/+1 counters among one or two target creatures.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: distribute_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            // {W}, {T}: Put a +1/+1 counter on each creature you control with one.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}: Put a +1/+1 counter on each creature you control with a +1/+1 counter on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bolster_counter_creatures,
            }),
    )
}

/// ETB — distribute two +1/+1 counters among the one or two chosen
/// creatures. Fidelity note: the exact distribution choice isn't an
/// engine primitive, so when one target is chosen it receives two
/// counters; when two are chosen each receives one.
fn distribute_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids: Vec<_> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(*id),
            _ => None,
        })
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    let per = (2 / ids.len() as u32).max(1);
    let effects = ids
        .into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: per,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}

fn bolster_counter_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter {
        has_counter: Some(CounterKind::PlusOnePlusOne),
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    let effects: Vec<_> = script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect();
    if effects.is_empty() {
        return Vec::new();
    }
    vec![Effect::Sequence(effects)]
}
