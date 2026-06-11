//! Destructive Flow — `{B}{R}{G}` enchantment.
//! "At the beginning of each player's upkeep, that player sacrifices a
//! nonbasic land of their choice."
//!
//! "Each player's upkeep" is split into two StepBegins triggers (You /
//! Opponent) since no accessor identifies whose upkeep fired; the
//! opponent half uses the documented 2-player read.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Destructive Flow");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
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
                effect: you_sacrifice_nonbasic,
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
                effect: opponent_sacrifices_nonbasic,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn nonbasic_land() -> ObjectFilter {
    ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
}

/// Your upkeep: you sacrifice a nonbasic land of your choice.
fn you_sacrifice_nonbasic(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: nonbasic_land(),
        count: 1,
    }]
}

/// An opponent's upkeep: that player sacrifices a nonbasic land.
fn opponent_sacrifices_nonbasic(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(opp) = script::opponents(state, trig.controller).first().copied()
    else {
        return Vec::new();
    };
    vec![Effect::Sacrifice {
        player: opp,
        filter: nonbasic_land(),
        count: 1,
    }]
}
