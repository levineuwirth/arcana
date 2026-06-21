//! Cactuar — `{G}` 3/3 Plant with Trample.
//!
//! "At the beginning of your end step, if this creature didn't enter the
//! battlefield this turn, return it to its owner's hand."
//!
//! The end-step bounce is gated by an intervening-if checking that the source
//! did NOT enter the battlefield this turn.

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cactuar");
    let plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
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
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_didnt_enter_this_turn),
            effect: return_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_didnt_enter_this_turn(
    s: &GameState,
    source: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    !script::entered_battlefield_this_turn(s, source)
}

fn return_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnToHand { target: trig.source }]
}
