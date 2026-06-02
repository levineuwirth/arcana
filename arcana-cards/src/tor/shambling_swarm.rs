//! Shambling Swarm — `{1}{B}{B}{B}` 3/3 black Horror. "When this
//! creature dies, distribute three -1/-1 counters among one, two, or
//! three target creatures. For each -1/-1 counter you put on a
//! creature this way, remove a -1/-1 counter from that creature at the
//! beginning of the next end step."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shambling Swarm");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: distribute_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
            }),
    )
}

fn distribute_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "distribute three -1/-1 counters among one, two, or three
    // target creatures" — the engine cannot split a fixed pool across a
    // player-chosen division, and the "remove a -1/-1 counter at the next
    // end step for each one placed" rider is not expressible. Best effort:
    // place one -1/-1 counter on each chosen target.
    let _ = ObjectFilter::creature();
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::MinusOneMinusOne,
                count: 1,
            }),
            _ => None,
        })
        .collect()
}
