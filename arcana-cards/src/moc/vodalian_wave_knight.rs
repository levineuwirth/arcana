//! Vodalian Wave-Knight — `{2}{W}{U}` 3/3 blue-white creature (Merfolk
//! Knight). "Whenever you draw a card, put a +1/+1 counter on each
//! other Merfolk and/or Knight you control."
//!
//! "each other Merfolk and/or Knight you control" — subtype-OR via
//! `ObjectFilter::with_subtypes_any`; "other" is honored by dropping the
//! source id from the resolved set.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vodalian Wave-Knight");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_draw(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "each other Merfolk and/or Knight you control": subtype-OR filter;
    // "other" = drop the source from the resolved set.
    let merfolk = reg
        .interner()
        .lookup("Merfolk")
        .expect("Merfolk interned during register()");
    let knight = reg
        .interner()
        .lookup("Knight")
        .expect("Knight interned during register()");
    let filter = ObjectFilter::permanent()
        .with_subtypes_any(vec![merfolk, knight])
        .controlled_by(ControllerConstraint::You);
    let mut ids = script::ids_matching(state, &filter, trig.controller);
    ids.retain(|&id| id != trig.source);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
