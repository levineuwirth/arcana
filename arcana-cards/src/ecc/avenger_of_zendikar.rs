//! Avenger of Zendikar — `{5}{G}{G}` 5/5 Elemental.
//!
//! * "When this creature enters, create a 0/1 green Plant creature token
//!   for each land you control." — an ETB trigger that counts your lands
//!   (`script::count_matching`) and emits that many `CreateToken`s.
//! * "Landfall — Whenever a land you control enters, you may put a +1/+1
//!   counter on each Plant creature you control." — a `ZoneChange`
//!   trigger watching lands entering under your control; resolution adds
//!   one +1/+1 counter to each Plant you control via `ForEach`.
//!   FIDELITY GAP: the "you may" is treated as unconditional (no engine
//!   may-prompt primitive in scope).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avenger of Zendikar");
    let elemental = reg.interner_mut().intern("Elemental");
    let _plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_plants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_grow_plants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_plants(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let lands = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let plant = reg.interner().lookup("Plant").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    let token = TokenDefinition {
        name: plant,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..lands)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}

fn landfall_grow_plants(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plant_filter = script::subtype_filter(reg, "Plant").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &plant_filter, trig.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
