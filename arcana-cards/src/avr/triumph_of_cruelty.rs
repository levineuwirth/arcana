//! Triumph of Cruelty — `{2}{B}` enchantment.
//! "At the beginning of your upkeep, target opponent discards a card
//! if you control the creature with the greatest power or tied for
//! the greatest power."
//!
//! The greatest-power check is a resolution-time condition evaluated
//! in the effect fn via `script::ids_matching` + `script::power_of`.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triumph of Cruelty");
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
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: opponent_discards_if_strongest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            },
        ),
    )
}

/// "…target opponent discards a card if you control the creature with
/// the greatest power or tied for the greatest power."
fn opponent_discards_if_strongest(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let all = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    let mine = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if mine.is_empty() {
        return Vec::new();
    }
    let max_all = all
        .iter()
        .map(|&id| script::power_of(state, id))
        .max()
        .unwrap_or(0);
    let max_mine = mine
        .iter()
        .map(|&id| script::power_of(state, id))
        .max()
        .unwrap_or(0);
    if max_mine < max_all {
        return Vec::new();
    }
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
