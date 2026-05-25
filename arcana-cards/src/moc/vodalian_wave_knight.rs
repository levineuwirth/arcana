//! Vodalian Wave-Knight — `{2}{W}{U}` 3/3 blue-white creature (Merfolk
//! Knight). "Whenever you draw a card, put a +1/+1 counter on each
//! other Merfolk and/or Knight you control."
//!
//! GAP: "each other Merfolk and/or Knight" — ObjectFilter can filter by
//! a single subtype via script::subtype_filter; filtering by two
//! disjoint subtypes is not supported. Using ForEach with Merfolk
//! filter as closest approximation; Knight coverage is dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
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
    // GAP: "each other Merfolk and/or Knight" — filtering by two
    // disjoint subtypes not supported; using Merfolk only as approximation.
    let filter = script::subtype_filter(reg, "Merfolk")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
