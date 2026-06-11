//! Shrieking Affliction — `{B}` enchantment.
//! "At the beginning of each opponent's upkeep, if that player has one
//! or fewer cards in hand, they lose 3 life."
//!
//! Wired on `StepBegins { Upkeep, Opponent }`. The intervening-if reads
//! the opponent's hand size via the documented 2-player read
//! (`script::opponents(..).first()`), as does the effect's "they".

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Shrieking Affliction");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: Some(if_opponent_hand_one_or_fewer),
            effect: lose_three_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "if that player has one or fewer cards in hand" — the upkeep player is
/// the opponent (2-player read off `script::opponents`).
fn if_opponent_hand_one_or_fewer(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let Some(p) = script::opponents(s, you).first().copied() else {
        return false;
    };
    script::hand_size(s, p) <= 1
}

/// "…they lose 3 life."
fn lose_three_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    vec![Effect::LoseLife { player: p, amount: 3 }]
}
