//! Manta Ray — `{1}{U}{U}` 3/3 Fish.
//! "This creature can't attack unless defending player controls an Island.
//!  This creature can't be blocked except by blue creatures.
//!  When you control no Islands, sacrifice this creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::{conditions, script};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Manta Ray");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: static "can't attack unless defending player controls an Island" —
            // no conditional attack-restriction Effect for an opponent-controls predicate.
            // GAP: static "can't be blocked except by blue creatures" —
            // CantBeBlocked has no by-color exception parameter.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "When you control no Islands, sacrifice this creature." Modeled as a
                // state-trigger approximation on your upkeep; gated by intervening-if.
                trigger_condition: TriggerCondition::StepBegins {
                    step: arcana_core::turn::Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_no_islands),
                effect: sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_no_islands(s: &GameState, _src: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_at_most(s, you, &script::subtype_filter(reg, "Island"), 0)
}

fn sacrifice_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
