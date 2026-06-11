//! Smuggler's Share — `{2}{W}` enchantment (Commander Legends: Battle
//! for Baldur's Gate, 2022). "At the beginning of each end step, draw
//! a card for each opponent who drew two or more cards this turn, then
//! create a Treasure token for each opponent who had two or more lands
//! enter the battlefield under their control this turn."
//!
//! The draw half is computable per opponent via
//! `script::cards_drawn_this_turn`; the Treasure half needs a
//! per-player lands-entered-this-turn count that no script accessor
//! provides — that half is an honest GAP.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Smuggler's Share");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: share,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…draw a card for each opponent who drew two or more cards this
/// turn…" (the Treasure half is GAP'd — see below).
fn share(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create a Treasure token for each opponent who had two or more
    // lands enter the battlefield under their control this turn" — no
    // script accessor counts per-player lands entered this turn.
    let n = script::opponents(state, trig.controller)
        .into_iter()
        .filter(|&p| script::cards_drawn_this_turn(state, p) >= 2)
        .count() as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
