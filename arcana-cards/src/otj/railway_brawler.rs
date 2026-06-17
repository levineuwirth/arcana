//! Railway Brawler — `{3}{G}{G}` 5/5 Rhino Warrior with Reach and
//! Trample. "Whenever another creature you control enters, put X
//! +1/+1 counters on it, where X is its power." Plot {3}{G}.
//!
//! Reach + Trample are base keywords. The ETB-of-another-creature
//! trigger reads the entering object's power and stamps that many
//! +1/+1 counters onto it. Plot (cast-from-exile cost) is not an
//! expressible keyword primitive.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Railway Brawler");
    let rhino = reg.interner_mut().intern("Rhino");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Plot {3}{G} (alternate cast-from-exile cost) is not an
    // expressible keyword/ability primitive.
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
                effect: counters_equal_power,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counters_equal_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = match trig.entering_object() {
        Some(id) => id,
        None => return Vec::new(),
    };
    // "another creature" — exclude this creature itself.
    if id == trig.source {
        return Vec::new();
    }
    let n = script::power_of(state, id).max(0) as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
