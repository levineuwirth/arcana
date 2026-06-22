//! Attuned Hunter — `{2}{G}` 3/3 Creature — Human Ranger.
//! Trample.
//! Whenever one or more cards leave your graveyard during your turn, put a
//! +1/+1 counter on this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Attuned Hunter");
    let human = reg.interner_mut().intern("Human");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Closest available condition: a card leaving the graveyard. The
            // "from: Graveyard" / "to: any zone" + "during your turn" gating
            // and "one or more cards" (batched) semantics are not fully
            // expressible — see GAP in the effect body.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: arcana_core::targets::ObjectFilter::new(),
                from: Some(Zone::Graveyard(0)),
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: add_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "during your turn" gating and "one or more cards leave your
    // graveyard" (to ANY zone, batched) cannot be expressed — ZoneChange needs
    // a single concrete `to` zone and there is no "leaves graveyard to any
    // zone, your turn only" trigger. The counter payload itself is faithful.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
