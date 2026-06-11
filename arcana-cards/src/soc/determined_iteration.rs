//! Determined Iteration — `{1}{R}` enchantment.
//! "At the beginning of combat on your turn, populate. The token created
//! this way gains haste. Sacrifice it at the beginning of the next end
//! step."
//!
//! Populate is modeled as a token-copy of a creature token you control.
//! // GAP: fidelity — populate's COPY CHOICE is deterministic here (the
//! // first matching token id, not player-chosen), and the haste +
//! // sacrifice riders cannot be attached to the new token (its id is
//! // not known at effect-construction time).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Determined Iteration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: populate_hasty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…populate. The token created this way gains haste. Sacrifice it at
/// the beginning of the next end step."
fn populate_hasty(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let tokens = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        trig.controller,
    );
    let Some(&id) = tokens.first() else {
        return Vec::new();
    };
    // GAP: copy choice is deterministic (first matching token); the haste
    // grant and next-end-step sacrifice riders cannot reference the
    // newly minted token's id.
    vec![Effect::CopyPermanent { target: id }]
}
