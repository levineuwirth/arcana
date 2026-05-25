//! Blech, Loafing Pest — `{1}{B}{G}` 3/4 legendary black-green Pest.
//! "Whenever you gain life, put a +1/+1 counter on each Pest, Bat, Insect,
//! Snake, and Spider you control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blech, Loafing Pest");
    let pest = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);
    let _bat = reg.interner_mut().intern("Bat");
    let _insect = reg.interner_mut().intern("Insect");
    let _snake = reg.interner_mut().intern("Snake");
    let _spider = reg.interner_mut().intern("Spider");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_life_gained,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_life_gained(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Collect all Pest, Bat, Insect, Snake, Spider you control and give each
    // a +1/+1 counter. We union by iterating each subtype filter.
    let subtypes = ["Pest", "Bat", "Insect", "Snake", "Spider"];
    let mut all_ids = std::collections::HashSet::new();
    for s in &subtypes {
        let filter = script::subtype_filter(reg, s)
            .controlled_by(ControllerConstraint::You);
        for id in script::ids_matching(state, &filter, trig.controller) {
            all_ids.insert(id);
        }
    }
    all_ids.into_iter().map(|id| Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }).collect()
}
