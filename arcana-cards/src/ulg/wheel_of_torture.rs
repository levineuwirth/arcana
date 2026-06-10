//! Wheel of Torture — `{3}` artifact (Tempest).
//! "At the beginning of each opponent's upkeep, this artifact deals X
//! damage to that player, where X is 3 minus the number of cards in their
//! hand."
//!
//! Fidelity note: "that player" (the upkeep player) has no dedicated
//! accessor on the trigger — modeled as the first opponent, which is exact
//! in two-player games.

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
    let name = reg.interner_mut().intern("Wheel of Torture");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: damage_upkeep_player,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn damage_upkeep_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player" — no upkeep-player accessor on the trigger;
    // approximated as the first opponent (exact in two-player games).
    let opponents = script::opponents(state, trig.controller);
    let Some(&opp) = opponents.first() else {
        return Vec::new();
    };
    let x = (3i32 - script::hand_size(state, opp) as i32).max(0) as u32;
    vec![Effect::DealDamage {
        target: DamageTarget::Player(opp),
        amount: x,
        source: trig.source,
    }]
}
