//! Stillness in Motion — `{1}{U}` enchantment (March of the Machine:
//! The Aftermath-adjacent pool). "At the beginning of your upkeep,
//! mill three cards. Then if your library has no cards in it, exile
//! this enchantment and put five cards from your graveyard on top of
//! your library in any order."
//!
//! The upkeep mill is wired; the empty-library reset (a player-ordered
//! five-card move from graveyard to library top) has no catalog
//! Effect — that clause is an honest GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stillness in Motion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mill_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…mill three cards." (The empty-library reset clause is GAP'd —
/// see below.)
fn mill_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Then if your library has no cards in it, exile this
    // enchantment and put five cards from your graveyard on top of your
    // library in any order" — no Effect moves a chosen, player-ordered set
    // of graveyard cards onto the library top, and the post-mill
    // empty-library check cannot be sequenced inside one resolution.
    vec![Effect::Mill {
        player: trig.controller,
        count: 3,
    }]
}
