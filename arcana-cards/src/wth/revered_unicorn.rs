//! Revered Unicorn — `{1}{W}` 2/3 Unicorn. Cumulative upkeep {1} (not an
//! expressible keyword → GAP). When it leaves the battlefield, you gain life
//! equal to the number of age counters on it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revered Unicorn");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let _age = reg.interner_mut().intern("age");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Cumulative upkeep {1} is not an expressible keyword/ability.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: gain_per_age,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_per_age(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(age) = reg.interner().lookup("age").map(CounterKind::Named) else {
        return Vec::new();
    };
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(age));
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
