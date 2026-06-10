//! Paupers' Cage — `{3}` artifact (Mercadian Masques, 1999).
//! "At the beginning of each opponent's upkeep, if that player has two
//! or fewer cards in hand, this artifact deals 2 damage to that
//! player." Wired as an opponent-upkeep trigger with an
//! intervening-if hand-size gate. Fidelity note: "that player" is
//! resolved as the (single) opponent — exact in two-player games; in
//! multiplayer the first opponent is used.

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
    let name = reg.interner_mut().intern("Paupers' Cage");
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
                intervening_if: Some(if_opponent_hand_two_or_fewer),
                effect: ping_upkeep_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// Intervening-if: "if that player has two or fewer cards in hand" —
/// "that player" is the upkeep player; approximated as the first
/// opponent (exact in two-player games).
fn if_opponent_hand_two_or_fewer(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let opps = script::opponents(s, you);
    let Some(opp) = opps.first() else {
        return false;
    };
    // conditions:: has no hand_at_most helper; script::hand_size is the
    // sanctioned amount accessor.
    script::hand_size(s, *opp) <= 2
}

fn ping_upkeep_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, trig.controller);
    let Some(opp) = opps.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*opp),
        amount: 2,
        source: trig.source,
    }]
}
