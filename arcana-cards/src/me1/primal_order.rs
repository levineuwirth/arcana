//! Primal Order — `{2}{G}{G}` enchantment.
//! "At the beginning of each player's upkeep, this enchantment deals
//! damage to that player equal to the number of nonbasic lands they
//! control."
//!
//! NOTE: "each player's upkeep" is modeled as TWO triggers (whose: You /
//! whose: Opponent); the opponent's identity uses the documented 2-player
//! read via `script::opponents`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
use arcana_core::types::{CardId, ColorSet, PlayerId, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Primal Order");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                intervening_if: None,
                effect: burn_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…deals damage to that player equal to the number of nonbasic lands
/// they control."
fn burn_for_nonbasics(
    state: &GameState,
    trig: &PendingTrigger,
    p: PlayerId,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, p);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(p),
        amount: n,
        source: trig.source,
    }]
}

fn burn_you(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    burn_for_nonbasics(state, trig, trig.controller)
}

fn burn_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    burn_for_nonbasics(state, trig, p)
}
