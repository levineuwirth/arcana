//! Sigil Captain — `{1}{G}{W}{W}` 3/3 green-white Rhino Soldier. "Whenever a
//! creature you control enters, if that creature is 1/1, put two +1/+1 counters
//! on it." ZoneChange trigger on friendly creature ETB with intervening-if
//! (1/1 check); add 2 +1/+1 counters to the entering creature.
//! GAP: intervening-if condition (is it 1/1) not expressible; use None and
//! check at resolution via script::power_of / toughness_of.

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
    let name = reg.interner_mut().intern("Sigil Captain");
    let rhino = reg.interner_mut().intern("Rhino");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
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
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_min_power(1)
                        .with_max_power(1)
                        .with_max_toughness(1),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_one_one_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_one_one_etb(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entering = trig.entering_object().unwrap_or(trig.source);
    let p = script::power_of(state, entering);
    let t = script::toughness_of(state, entering);
    if p == 1 && t == 1 {
        vec![Effect::AddCounters {
            target: entering,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        }]
    } else {
        Vec::new()
    }
}
