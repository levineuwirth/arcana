//! Scarab of the Unseen — `{2}` artifact (Antiquities, 1994).
//! "{T}, Sacrifice this artifact: Return all Auras attached to target
//! permanent you own to their owners' hands. Draw a card at the
//! beginning of the next turn's upkeep."
//!
//! A non-creature artifact whose single activated ability sacrifices
//! itself ({T} + Sacrifice) targeting a permanent you own, bounces the
//! Auras attached to it, and schedules a delayed upkeep draw.
//!
//! The delayed draw IS expressible (Effect::DelayedAction with
//! DelayedWhen::NextUpkeep + DelayedAction::ControllerDrawsCard, the
//! Lodestone Bauble pattern). The Aura-bounce is GAP'd: ObjectFilter
//! has no "attached to <id>" predicate, so "return all Auras attached
//! to the chosen permanent" can't be enumerated with the demonstrated
//! API. The cost and target are kept faithful.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scarab of the Unseen");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this artifact: Return all Auras attached to \
                   target permanent you own to their owners' hands. Draw a card \
                   at the beginning of the next turn's upkeep."
                .into(),
            cost: ActivationCost {
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::new()),
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::You),
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: scarab_effect,
        }),
    )
}

fn scarab_effect(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Return all Auras attached to target permanent you own to
    // their owners' hands" — ObjectFilter has no attached-to predicate,
    // so the set of Auras on the chosen permanent can't be enumerated.
    // The delayed upkeep draw IS expressible and is kept.
    vec![Effect::DelayedAction {
        source: ctx.source,
        controller: ctx.controller,
        when: DelayedWhen::NextUpkeep,
        action: DelayedAction::ControllerDrawsCard,
    }]
}
