//! Land Tax — `{W}` enchantment.
//! "At the beginning of your upkeep, if an opponent controls more lands
//! than you, you may search your library for up to three basic land
//! cards, reveal them, put them into your hand, then shuffle."
//!
//! // GAP: fidelity — "you may … up to three" is modeled as three
//! // mandatory single-card tutors (each finds nothing once the library
//! // has no more basic lands); the optionality is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Land Tax");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_opponent_more_lands),
                effect: fetch_basics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// Intervening-if: "if an opponent controls more lands than you".
fn if_opponent_more_lands(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let yours = script::count_matching(
        s,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        you,
    );
    let theirs = script::count_matching(
        s,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::Opponent),
        you,
    );
    theirs > yours
}

/// "…search your library for up to three basic land cards, reveal them,
/// put them into your hand, then shuffle."
fn fetch_basics(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let basic_land = || {
        ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
    };
    vec![
        Effect::TutorToHand {
            player: trig.controller,
            filter: basic_land(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: basic_land(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: basic_land(),
            reveal: true,
        },
    ]
}
