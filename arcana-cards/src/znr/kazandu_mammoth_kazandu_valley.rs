//! Kazandu Mammoth // Kazandu Valley
//!
//! Front: Creature — Elephant {1}{G}{G} 3/3 (green)
//!   Landfall — Whenever a land you control enters, this creature gets +2/+2 until end of turn.
//! Back: Land (enters tapped; {T}: Add {G})
//! GAP: MDFC back face not modeled (mechanic deferred)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::layers::Duration;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kazandu Mammoth");
    let back_name = reg.interner_mut().intern("Kazandu Valley");
    let elephant = reg.interner_mut().intern("Elephant");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(elephant);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // Landfall trigger
    let landfall_trigger = TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::ZoneChange {
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
            from: None,
            to: Zone::Battlefield,
        },
        intervening_if: None,
        effect: landfall_pump,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![],
    };

    let back_chars = Characteristics {
        types: TypeLine::LAND.into(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(landfall_trigger)
            .with_mdfc_back(CardFace {
                name: back_name,
                characteristics: back_chars,
                spell_ability: None,
            }),
    )
}

fn landfall_pump(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
