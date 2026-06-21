//! Spike Cannibal — `{1}{B}{B}` 0/0 Spike.
//!
//! This creature enters with a +1/+1 counter on it.
//! When this creature enters, move all +1/+1 counters from all creatures onto
//! it.
//!
//! The "enters with a +1/+1 counter" replacement has no expressible
//! enters-with primitive on this card class and is GAP'd. The ETB
//! counter-move is implemented by enumerating every other creature and
//! moving each one's +1/+1 counters onto this creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spike Cannibal");
    let spike = reg.interner_mut().intern("Spike");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spike);

    // GAP: "enters with a +1/+1 counter on it" — no enters-with-counter
    //      replacement primitive on this card class.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_move_all_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_move_all_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    ids.into_iter()
        .filter(|&id| id != trig.source)
        .filter_map(|id| {
            let n = state
                .objects
                .get(id)
                .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
            if n == 0 {
                None
            } else {
                Some(Effect::MoveCounter {
                    from: id,
                    to: trig.source,
                    kind: CounterKind::PlusOnePlusOne,
                    count: n,
                })
            }
        })
        .collect()
}
