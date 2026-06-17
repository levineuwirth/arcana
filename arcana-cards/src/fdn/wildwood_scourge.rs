//! Wildwood Scourge — `{X}{G}` 0/0 Hydra. "This creature enters with X
//! +1/+1 counters on it." "Whenever one or more +1/+1 counters are put on
//! another non-Hydra creature you control, put a +1/+1 counter on this
//! creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wildwood Scourge");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // The trigger watches +1/+1 counters placed on ANOTHER creature you
    // control (TriggerSelf::AnotherMatching excludes the source). Self-
    // exclusion is mandatory: this ability's resolution adds a +1/+1 counter
    // to itself, so AnyMatching (which includes the source) re-fires on its
    // own counter forever (random-game harness seed 295). GAP DETAIL: the
    // "non-Hydra" exclusion still isn't expressible (minor over-fire on other
    // Hydras you control).
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromX {
                kind: CounterKind::PlusOnePlusOne,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::AnotherMatching(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: add_counter_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_counter_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
