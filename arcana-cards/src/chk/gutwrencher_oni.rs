//! Gutwrencher Oni — `{3}{B}{B}` 5/4 Demon Spirit (B) with Trample.
//!
//! Oracle:
//! * Trample.
//! * At the beginning of your upkeep, discard a card if you don't
//!   control an Ogre.
//!
//! Trample is a base keyword. The upkeep trigger carries an
//! intervening-if ("if you don't control an Ogre") gating the discard;
//! the gate is implemented as a CR 603.4 condition that returns true
//! only when the controller controls no Ogre.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gutwrencher Oni");
    let demon = reg.interner_mut().intern("Demon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
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
                intervening_if: Some(if_you_dont_control_an_ogre),
                effect: upkeep_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_you_dont_control_an_ogre(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    !conditions::you_control_a(s, you, &script::subtype_filter(reg, "Ogre"))
}

fn upkeep_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
