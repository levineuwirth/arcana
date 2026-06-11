//! Karma — `{2}{W}{W}` enchantment (Limited Edition Alpha, 1993).
//! "At the beginning of each player's upkeep, this enchantment deals
//! damage to that player equal to the number of Swamps they control."
//!
//! "Each player's upkeep / that player" has no typed accessor, so the
//! trigger is decomposed into two `StepBegins` triggers — one for your
//! upkeep (damaging you by your Swamp count) and one for the
//! opponent's (damaging the opponent via the documented 2-player
//! `script::opponents` read).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karma");
    let _swamp = reg.interner_mut().intern("Swamp");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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

/// Your upkeep: damage you for your Swamp count.
fn burn_you(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Swamp")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(trig.controller),
        amount: n,
        source: trig.source,
    }]
}

/// An opponent's upkeep: damage that opponent for their Swamp count
/// (documented 2-player read for "that player").
fn burn_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(opp) = script::opponents(state, trig.controller).first().copied()
    else {
        return Vec::new();
    };
    let filter = script::subtype_filter(reg, "Swamp")
        .controlled_by(ControllerConstraint::Opponent);
    let n = script::count_matching(state, &filter, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(opp),
        amount: n,
        source: trig.source,
    }]
}
