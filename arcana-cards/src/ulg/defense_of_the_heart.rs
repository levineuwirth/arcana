//! Defense of the Heart — `{3}{G}` enchantment.
//! "At the beginning of your upkeep, if an opponent controls three or
//! more creatures, sacrifice this enchantment, search your library for
//! up to two creature cards, put those cards onto the battlefield, then
//! shuffle."
//!
//! GAP (fidelity): the self-sacrifice is modeled as DestroyPermanent on
//! the source (no sacrifice-self one-shot), and "up to two" tutors are
//! emitted as two mandatory TutorToBattlefield effects.

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
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Defense of the Heart");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_opponent_has_three_creatures),
            effect: sacrifice_and_tutor,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…if an opponent controls three or more creatures…"
fn if_opponent_has_three_creatures(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::count_matching(
        s,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        you,
    ) >= 3
}

/// "…sacrifice this enchantment, search your library for up to two
/// creature cards, put those cards onto the battlefield, then shuffle."
fn sacrifice_and_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        // GAP: sacrifice-self one-shot modeled as DestroyPermanent.
        Effect::DestroyPermanent {
            target: trig.source,
        },
        // GAP: "up to two" — two mandatory tutors (shuffle automatic).
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            tapped: false,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            tapped: false,
        },
    ]
}
