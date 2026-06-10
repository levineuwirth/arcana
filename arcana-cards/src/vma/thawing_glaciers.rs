//! Thawing Glaciers — nonbasic land.
//! "This land enters tapped." and "{1}, {T}: Search your library for a
//! basic land card, put that card onto the battlefield tapped, then
//! shuffle. Return this land to its owner's hand at the beginning of
//! the next cleanup step."
//! Enters-tapped plus a fetch activation; the self-return is a delayed
//! ReturnToHand.
//! GAP fidelity: the delayed return fires at the next END step
//! (DelayedWhen has no cleanup-step variant) instead of the cleanup
//! step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thawing Glaciers");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle. Return this land to its owner's hand at the beginning of the next cleanup step.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fetch_and_bounce,
            }),
    )
}

fn fetch_and_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: returns at the next end step, not the cleanup step
    // (DelayedWhen has no cleanup variant).
    vec![
        Effect::TutorToBattlefield {
            player: ctx.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
            tapped: true,
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
    ]
}
