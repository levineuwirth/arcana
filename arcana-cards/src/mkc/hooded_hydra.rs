//! Hooded Hydra — `{X}{G}{G}` 0/0 green Snake Hydra.
//! "Enters with X +1/+1 counters. When this creature dies, create a 1/1 green
//! Snake creature token for each +1/+1 counter on it. Morph {3}{G}{G}. As this
//! creature is turned face up, put five +1/+1 counters on it."

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
    let name = reg.interner_mut().intern("Hooded Hydra");
    let snake = reg.interner_mut().intern("Snake");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(hydra);
    // GAP: "enters with X +1/+1 counters" — ETB-with-X-counters (X from the
    // casting cost) is a replacement effect; no primitive exposes X to a
    // trigger.
    // GAP: "Morph {3}{G}{G}" and "As this creature is turned face up, put five
    // +1/+1 counters on it" — Morph is not in the supported KeywordAbility set
    // and there is no turned-face-up trigger condition.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: snakes_on_death,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn snakes_on_death(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: arcana_core::effects::TokenDefinition {
                name: snake,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
