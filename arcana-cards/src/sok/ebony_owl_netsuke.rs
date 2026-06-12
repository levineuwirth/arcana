//! Ebony Owl Netsuke — `{2}` artifact (Saviors of Kamigawa, 2005).
//! "At the beginning of each opponent's upkeep, if that player has seven
//! or more cards in hand, this artifact deals 4 damage to that player."
//! StepBegins(Upkeep, Opponent) trigger; "that player" is the upkeep
//! owner — the active player while the trigger fires/resolves
//! (`state.active_player()`). The intervening-if gates on that player's
//! hand size.

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
    let name = reg.interner_mut().intern("Ebony Owl Netsuke");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
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
                // "…if that player has seven or more cards in hand…" —
                // "that player" is the upkeep owner (the active player).
                intervening_if: Some(if_upkeep_player_full_hand),
                effect: ping_full_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if that player has seven or more cards in hand…"
fn if_upkeep_player_full_hand(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::hand_size(s, s.active_player()) >= 7
}

fn ping_full_hand(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "That player" = whose upkeep it is = the active player.
    vec![Effect::DealDamage {
        target: DamageTarget::Player(state.active_player()),
        amount: 4,
        source: trig.source,
    }]
}
