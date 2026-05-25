//! Crime Novelist — `{2}{R}` 1/3 red Goblin Bard.
//! "Whenever you sacrifice an artifact, put a +1/+1 counter on this
//! creature and add {R}."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crime Novelist");
    let goblin = reg.interner_mut().intern("Goblin");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(bard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                },
                intervening_if: None,
                effect: sacrifice_artifact_counter_and_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_artifact_counter_and_mana(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters { target: trig.source, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::AddMana {
            player: trig.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
        },
    ]
}
