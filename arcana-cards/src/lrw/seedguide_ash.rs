//! Seedguide Ash — `{4}{G}` 4/4 green Creature — Treefolk Druid.
//! "When this creature dies, you may search your library for up to three Forest cards,
//! put them onto the battlefield tapped, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seedguide Ash");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_tutor_forests,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_tutor_forests(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Tutor up to three Forest cards to battlefield tapped.
    // TutorToBattlefield puts one card; repeat three times for "up to three".
    // GAP: "tapped" parameter on TutorToBattlefield — using tapped: true.
    // GAP: "up to three" — emitting three separate tutors; controller can find fewer.
    let filter = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToBattlefield { player: trig.controller, filter: filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: trig.controller, filter: filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: trig.controller, filter, tapped: true },
    ]
}
