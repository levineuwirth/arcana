//! Veiled Crocodile — `{2}{U}` enchantment.
//! "When a player has no cards in hand, if this permanent is an
//! enchantment, it becomes a 4/4 Crocodile creature."
//!
//! GAP: "when a player has no cards in hand" is a state trigger with
//! no condition variant; the closest stand-in is `CardDiscarded(Any)`
//! (the usual way a hand empties) gated by an intervening-if that
//! checks for an empty hand. GAP: "if this permanent is an
//! enchantment" (not yet animated) has no source-type predicate.
//! GAP: the Crocodile subtype cannot be added (no add-subtype
//! effect).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veiled Crocodile");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "when a player has no cards in hand" is a
                // state trigger; CardDiscarded(Any) + an empty-hand
                // intervening-if is the closest approximation (misses
                // hands emptied by casting/playing the last card).
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Any,
                },
                intervening_if: Some(if_a_player_has_empty_hand),
                effect: awaken_crocodile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…a player has no cards in hand…"
fn if_a_player_has_empty_hand(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::all_players(s)
        .into_iter()
        .any(|p| script::hand_size(s, p) == 0)
}

/// "…it becomes a 4/4 Crocodile creature."
/// GAP: the Crocodile subtype is not addable.
fn awaken_crocodile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 4,
            toughness: 4,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
