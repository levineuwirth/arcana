//! Mask of Intolerance — `{2}` artifact.
//! "At the beginning of each player's upkeep, if there are four or more
//! basic land types among lands that player controls, this artifact deals
//! 3 damage to that player."
//!
//! NOTE: "each player's upkeep" is modeled as TWO triggers (whose: You /
//! whose: Opponent); the opponent's identity uses the documented 2-player
//! read via `script::opponents`.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mask of Intolerance");
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
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
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_four_types_you),
                effect: burn_you,
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
                intervening_if: Some(if_four_types_opponent),
                effect: burn_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "four or more basic land types among lands that player controls" —
/// counts which of the five basic land types `p` controls at least one of.
fn four_basic_types(s: &GameState, p: PlayerId, reg: &CardRegistry) -> bool {
    let mut n = 0;
    for t in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        if conditions::you_control_a(s, p, &script::subtype_filter(reg, t)) {
            n += 1;
        }
    }
    n >= 4
}

fn if_four_types_you(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    four_basic_types(s, you, reg)
}

fn if_four_types_opponent(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let Some(p) = script::opponents(s, you).first().copied() else {
        return false;
    };
    four_basic_types(s, p, reg)
}

/// "…this artifact deals 3 damage to that player." (your upkeep)
fn burn_you(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DealDamage {
        target: DamageTarget::Player(trig.controller),
        amount: 3,
        source: trig.source,
    }]
}

/// "…this artifact deals 3 damage to that player." (opponent's upkeep)
fn burn_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(p),
        amount: 3,
        source: trig.source,
    }]
}
