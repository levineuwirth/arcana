//! Akal Pakal, First Among Equals — `{2}{U}` 1/5 blue Legendary Creature — Human Advisor.
//! "At the beginning of each player's end step, if an artifact entered
//! the battlefield under your control this turn, look at the top two
//! cards of your library. Put one of them into your hand and the other
//! into your graveyard."
//!
//! GAP: "look at top two cards, put one in hand and other in graveyard"
//! — Surveil(2) mills one and keeps one, but the choice is not
//! precisely modeled. Using Surveil(2) as the closest available
//! approximation; the intervening-if artifact-entered condition is
//! also not expressible so intervening_if: None.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akal Pakal, First Among Equals");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening_if — "if an artifact entered the
                // battlefield under your control this turn"; not expressible.
                intervening_if: None,
                effect: on_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_end_step(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top two, put one in hand one in graveyard" — using
    // Surveil(2) as structural approximation; player choice of which
    // goes to hand vs graveyard is not captured exactly.
    vec![Effect::Surveil { player: trig.controller, count: 2 }]
}
