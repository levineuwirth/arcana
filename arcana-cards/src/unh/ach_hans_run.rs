//! "Ach! Hans, Run!" — `{2}{R}{R}{G}{G}` enchantment (Unhinged).
//! "At the beginning of your upkeep, you may say "Ach! Hans, run! It's
//! the . . ." and the name of a creature card. If you do, search your
//! library for a card with that name, put it onto the battlefield,
//! then shuffle. That creature gains haste. Exile it at the beginning
//! of the next end step."
//!
//! Best-effort: the tutor-to-battlefield over creature cards is wired.
//! GAPs: the resolution-time card-name choice (the search should be
//! restricted to the named card), the haste grant, and the delayed
//! exile both need the id of the card put onto the battlefield, which
//! `TutorToBattlefield` does not expose.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("\"Ach! Hans, Run!\"");
    let chars = Characteristics {
        name,
        mana_cost: Some(
            ManaCost::parse("{2}{R}{R}{G}{G}").expect("valid cost"),
        ),
        colors: ColorSet::red() | ColorSet::green(),
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
                intervening_if: None,
                effect: hans_run,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…search your library for a card with that name, put it onto the
/// battlefield, then shuffle. That creature gains haste. Exile it at
/// the beginning of the next end step."
fn hans_run(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "say a creature card's name" choice is not expressible —
    // the search is widened to any creature card ("you may" is resolved
    // as mandatory). The haste grant and the exile at the next end step
    // need the tutored card's battlefield id, which TutorToBattlefield
    // does not expose, so both riders are omitted.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
        tapped: false,
    }]
}
