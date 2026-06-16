//! Broodlord — `{X}{3}{G}` 3/3 Tyranid.
//!
//! * Ravenous (enters with X +1/+1 counters; if X is 5 or more, draw a card when
//!   it enters). (GAP — X-on-enter counters + the X≥5 conditional draw have no
//!   primitive keyed to the cast's X value.)
//! * Brood Telepathy — When this enters, distribute X +1/+1 counters among any
//!   number of other target creatures you control. (GAP — no counter-distribution
//!   primitive, and X comes from the cast cost.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broodlord");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: brood_telepathy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn brood_telepathy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "distribute X +1/+1 counters among any number of other target creatures
    // you control" — no counter-distribution primitive (AddCounters is fixed-count
    // to a single target) and X derives from the spell's cast cost.
    Vec::new()
}
