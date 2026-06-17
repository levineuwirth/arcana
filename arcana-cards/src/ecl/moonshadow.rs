//! Moonshadow — `{B}` 7/7 Elemental with Menace.
//! Enters with six -1/-1 counters on it.
//! Whenever one or more permanent cards are put into your graveyard from
//! anywhere while this creature has a -1/-1 counter on it, remove a
//! -1/-1 counter from this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::conditions;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moonshadow");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    // GAP: "enters with six -1/-1 counters on it" — enters-with-counters is not in the
    // demonstrated trigger/activated surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You)
                        .with_types_any(TypeLine(
                            TypeLine::CREATURE
                                | TypeLine::ARTIFACT
                                | TypeLine::ENCHANTMENT
                                | TypeLine::LAND
                                | TypeLine::PLANESWALKER,
                        )),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: Some(if_has_minus_counter),
                effect: remove_minus_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_has_minus_counter(
    s: &GameState,
    src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::source_has_counter(s, src, CounterKind::MinusOneMinusOne)
}

fn remove_minus_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
