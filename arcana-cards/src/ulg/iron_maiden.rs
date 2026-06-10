//! Iron Maiden — `{3}` artifact (Urza's Legacy).
//! "At the beginning of each opponent's upkeep, this artifact deals X
//! damage to that player, where X is the number of cards in their
//! hand minus 4." The opponent-upkeep trigger and the dynamic hand
//! count are wired; the per-event upkeep player is approximated by
//! the first opponent (faithful in two-player games).

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
    let name = reg.interner_mut().intern("Iron Maiden");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: damage_for_hand_size,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn damage_for_hand_size(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player" — the per-event upkeep player is not readable from
    // PendingTrigger; the first opponent is used (exact in two-player games).
    let opponents = script::opponents(state, trig.controller);
    let Some(player) = opponents.first() else {
        return Vec::new();
    };
    let x = script::hand_size(state, *player).saturating_sub(4);
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*player),
        amount: x,
        source: trig.source,
    }]
}
