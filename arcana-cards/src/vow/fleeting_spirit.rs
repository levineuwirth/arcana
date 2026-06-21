//! Fleeting Spirit — `{1}{W}` 3/1 Spirit.
//! "{W}, Exile three cards from your graveyard: This creature gains first strike
//!  until end of turn."
//! "Discard a card: Exile this creature. Return it to the battlefield under its
//!  owner's control at the beginning of the next end step."
//!
//! The first activated ability is a GAP — its "Exile three cards from your
//! graveyard" cost is not an expressible ActivationCost (no graveyard-exile cost
//! field; only hand-discard/self-sacrifice/counter costs exist). The second
//! ability is fully wired: discard-a-card cost + self-exile + next-end-step return.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fleeting Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: activated ability — "{W}, Exile three cards from your graveyard: gains
    // first strike until end of turn." The "exile three cards from your graveyard"
    // cost is not an expressible ActivationCost (no graveyard-exile cost field), so
    // the whole ability is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Discard a card: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.".into(),
            cost: ActivationCost {
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: blink_self,
        }),
    )
}

fn blink_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::ExilePermanent { target: ctx.source },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
