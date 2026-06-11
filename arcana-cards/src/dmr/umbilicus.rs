//! Umbilicus — `{4}` artifact.
//! "At the beginning of each player's upkeep, that player may pay 2 life.
//! If they don't, they return a permanent they control to its owner's
//! hand."
//!
//! NOTE: "each player's upkeep" is modeled as TWO triggers (whose: You /
//! whose: Opponent); the opponent's identity uses the documented 2-player
//! read via `script::opponents`.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Umbilicus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
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
                effect: tax_you,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: tax_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tax_you(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    tax(trig.controller)
}

fn tax_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    tax(p)
}

/// "…that player may pay 2 life. If they don't, they return a permanent
/// they control to its owner's hand."
fn tax(p: PlayerId) -> Vec<Effect> {
    // GAP: fidelity — "return A permanent they control" is exactly one;
    // ChooseAnyNumberFromZone is the closest selection primitive but allows
    // choosing zero (or more than one) permanents.
    vec![Effect::OptionalPayment {
        chooser: p,
        cost: OptionalPaymentKind::Life(2),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::ChooseAnyNumberFromZone {
            chooser: p,
            zone: Zone::Battlefield,
            filter: ObjectFilter::permanent()
                .controlled_by(ControllerConstraint::You),
            action: PickAction::ReturnToHand,
        })),
    }]
}
