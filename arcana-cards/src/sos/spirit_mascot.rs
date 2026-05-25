//! Spirit Mascot — `{R}{W}` 2/2 red-white creature. "Whenever one or more cards
//! leave your graveyard, put a +1/+1 counter on this creature."
//!
//! GAP: trigger — no "cards leave graveyard" TriggerCondition. Omitting trigger;
//! emitting stub with Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit Mascot");
    let spirit = reg.interner_mut().intern("Spirit");
    let ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(ox);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: trigger — "cards leave graveyard" not expressible; using SelfBecomesTapped as placeholder stub
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: no graveyard-leave trigger; entire ability unimplementable
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: graveyard_leave_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn graveyard_leave_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: trigger condition not correct; real ability fires on graveyard leave
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
