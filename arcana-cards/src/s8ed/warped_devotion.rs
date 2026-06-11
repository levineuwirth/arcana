//! Warped Devotion — `{2}{B}` enchantment.
//! "Whenever a permanent is returned to a player's hand, that player
//! discards a card."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warped Devotion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent(),
                    from: Some(Zone::Battlefield),
                    // Any player's hand (0 as the wildcard owner, mirroring
                    // the Zone::Graveyard(0) convention for "a graveyard").
                    to: Zone::Hand(0),
                },
                intervening_if: None,
                effect: bounced_player_discards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player discards a card." — the player whose hand received the
/// permanent (its owner).
fn bounced_player_discards(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no documented accessor pairs with a hand-bound ZoneChange;
    // entering_object() is documented for battlefield-bound moves and may
    // return None here, in which case the trigger no-ops.
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    let player = script::target_controller(state, id, trig.controller);
    vec![Effect::Discard {
        player,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
