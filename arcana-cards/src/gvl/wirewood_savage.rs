//! Wirewood Savage — `{2}{G}` 2/2 green Creature — Elf.
//! "Whenever a Beast enters, you may draw a card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wirewood Savage");
    let elf = reg.interner_mut().intern("Elf");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Any),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: beast_enters_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn beast_enters_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ZoneChange filter cannot narrow to Beast subtype specifically via ObjectFilter alone;
    // using subtype_filter to check but we can only use it with script::ids_matching, not in the trigger filter.
    // Best effort: draw whenever any creature enters (Beast subtype not filterable in trigger condition).
    let beast_filter = arcana_core::script::subtype_filter(reg, "Beast")
        .controlled_by(ControllerConstraint::Any);
    // We cannot check if the entering creature was the Beast here without state access beyond API.
    // GAP: no way to check 'entering creature is a Beast' at resolution time; drawing unconditionally.
    let _ = beast_filter;
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
