//! Juniper Order Ranger — `{3}{G}{W}` 2/4 green-white Human Knight Ranger.
//! "Whenever another creature you control enters, put a +1/+1 counter on
//! that creature and a +1/+1 counter on this creature." ZoneChange trigger;
//! add +1/+1 to entering creature and to self.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Juniper Order Ranger");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    subtypes.0.insert(ranger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_creature_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entering = trig.entering_object().unwrap_or(trig.source);
    vec![
        Effect::AddCounters {
            target: entering,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
