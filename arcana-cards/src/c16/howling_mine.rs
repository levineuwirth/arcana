//! Howling Mine — `{2}` artifact (Alpha, 1993).
//! "At the beginning of each player's draw step, if this artifact is
//! untapped, that player draws an additional card."
//! Modeled as TWO draw-step triggers (whose: You / whose: Opponent) so
//! "that player" can be resolved without an event-player accessor —
//! exact in two-player games; in multiplayer the first opponent stands
//! in for "that player" on opposing draw steps. The intervening-if
//! checks "this artifact is untapped" via an untapped-only name filter
//! (approximate if you control multiple Howling Mines).

use arcana_core::conditions;
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
    let name = reg.interner_mut().intern("Howling Mine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Draw,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_self_untapped),
                effect: controller_draws,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Draw,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: Some(if_self_untapped),
                effect: opponent_draws,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Intervening-if (CR 603.4): "if this artifact is untapped".
/// Approximated via an untapped-only filter on this card's name (the
/// sanctioned predicates have no per-source tapped accessor) — exact
/// unless you control multiple Howling Mines in mixed tap states.
fn if_self_untapped(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let filter = ObjectFilter {
        name: reg.interner().lookup("Howling Mine"),
        ..ObjectFilter::default()
    }
    .untapped_only();
    conditions::you_control_a(s, you, &filter)
}

fn controller_draws(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

fn opponent_draws(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, trig.controller);
    let Some(opp) = opps.first() else {
        return Vec::new();
    };
    vec![Effect::DrawCards { player: *opp, count: 1 }]
}
