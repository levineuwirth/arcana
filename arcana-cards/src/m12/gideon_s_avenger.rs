//! Gideon's Avenger — `{1}{W}{W}` 2/2 white Human Soldier.
//! "Whenever a creature an opponent controls becomes tapped, put
//! a +1/+1 counter on this creature."
//!
//! GAP: the trigger catalog only models `SelfBecomesTapped` — a
//! filtered "another creature becomes tapped" condition is not
//! expressible. Closest available variant used so the +1/+1 effect
//! body is recorded; a follow-up engine pass needs a generic
//! `BecomesTapped { filter }` trigger.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon's Avenger");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                // GAP: trigger — oracle is "whenever a creature an
                // opponent controls becomes tapped"; the catalog has
                // only `SelfBecomesTapped` (no filtered/opponent
                // variant). Using closest available so the effect
                // body is wired; semantics will under-fire until a
                // generic `BecomesTapped { filter }` exists.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: add_plus_one_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Put a +1/+1 counter on Gideon's Avenger itself.
fn add_plus_one_counter(
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
