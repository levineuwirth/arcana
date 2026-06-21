//! Mausoleum Harpy — `{4}{B}` 3/3 Harpy.
//! Flying.
//! Ascend — GAP: city's-blessing is not in the usable keyword/predicate surface.
//! Whenever another creature you control dies, if you have the city's
//! blessing, put a +1/+1 counter on this creature. (intervening-if GAP'd —
//! no city's-blessing predicate.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mausoleum Harpy");
    let harpy = reg.interner_mut().intern("Harpy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(harpy);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            // GAP: intervening-if "if you have the city's blessing" — no
            // city's-blessing predicate in conditions::; fires unconditionally.
            intervening_if: None,
            effect: counter_on_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_on_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
