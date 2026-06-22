//! Bonehoard Dracosaur — `{3}{R}{R}` 5/5 Dinosaur Dragon with Flying
//! and First strike.
//!
//! Oracle:
//! * Flying, first strike
//! * At the beginning of your upkeep, exile the top two cards of your
//!   library. You may play them this turn. If you exiled a land card
//!   this way, create a 3/1 red Dinosaur creature token. If you exiled
//!   a nonland card this way, create a Treasure token.
//!
//! The upkeep impulse-exile ("exile the top two, you may play them this
//! turn") is wired via `Effect::ImpulseExile`.
//!
//! GAP: the conditional follow-ups (create a 3/1 red Dinosaur if a land
//! was exiled; create a Treasure if a nonland was exiled) depend on
//! inspecting which cards were exiled this way; no accessor exposes the
//! impulse-exiled cards' types, so the conditional token creation cannot
//! be expressed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bonehoard Dracosaur");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_impulse(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ImpulseExile { player: trig.controller, count: 2 }]
}
