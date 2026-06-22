//! Baloth Packhunter — `{3}{G}` 3/3 green Creature — Beast.
//!
//! Trample
//! When this creature enters, put two +1/+1 counters on each other
//! creature you control named Baloth Packhunter.
//!
//! Decomposition: Trample → `keywords`; the ETB clause → one
//! `TriggeredAbilityDef` that fans an `AddCounters` over every OTHER
//! creature you control named "Baloth Packhunter" via `Effect::ForEach`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("Baloth Packhunter");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter_named,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_counter_named(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Baloth Packhunter");
    let filter = ObjectFilter {
        name: nm,
        ..ObjectFilter::creature()
    };
    let others = script::ids_matching(state, &filter, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect::<Vec<_>>();
    vec![Effect::ForEach {
        targets: others,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        }),
    }]
}
