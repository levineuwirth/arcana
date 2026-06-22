//! Arvad, Weatherlight Smuggler — `{W}{B}` 1/1 Legendary Vampire Knight (W/B).
//! Deathtouch, lifelink.
//! At the beginning of your end step, if a creature died this turn, Arvad
//!   perpetually gets +X/+X where X is the number of creatures that died
//!   this turn. This ability also triggers if Arvad is in your graveyard.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arvad, Weatherlight Smuggler");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of your end step, if a creature died this
            // turn, Arvad gets +X/+X. Also triggers from the graveyard.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_a_creature_died),
                effect: pump_by_deaths,
                // Also triggers while Arvad is in the graveyard.
                trigger_zones: vec![Zone::Battlefield, Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_a_creature_died(state: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    script::creatures_died_this_turn(state) >= 1
}

fn pump_by_deaths(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::creatures_died_this_turn(state) as i32;
    // "Perpetually gets +X/+X" approximated with a permanent pump.
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: n,
        duration: Duration::Permanent,
        keywords: vec![],
    }]
}
