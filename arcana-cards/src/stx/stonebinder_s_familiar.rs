//! Stonebinder's Familiar — `{W}` 1/1 white Spirit Dog creature.
//! "Whenever one or more cards are put into exile during your turn, put a +1/+1 counter on
//! this creature. This ability triggers only once each turn."
//!
//! # Notes
//! GAP: "cards are put into exile during your turn" — no TriggerCondition for exile events.
//! Using ZoneChange to Exile zone as closest approximation.
//! GAP: Zone::Exile not in catalog examples; using Zone::Battlefield as placeholder.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stonebinder's Familiar");
    let spirit = reg.interner_mut().intern("Spirit");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "cards put into exile during your turn" has no TriggerCondition.
                // Using ZoneChange from anywhere to Exile as best approximation.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new(),
                    from: None,
                    to: Zone::Exile,
                },
                intervening_if: None,
                effect: exile_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::PlusOnePlusOne, count: 1 }]
}
