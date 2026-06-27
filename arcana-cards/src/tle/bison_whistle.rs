//! Bison Whistle — `{1}{G}` Artifact (green).
//! "{1}, {T}: Look at the top card of your library. If it's a Bison
//! card, you may put it onto the battlefield. If it's a creature card,
//! you may reveal it and put it into your hand. Otherwise, you may put
//! it into your graveyard."
//!
//! Modeled with `DigTopN` over the top 1 card: a creature card may go to
//! hand, otherwise it goes to the graveyard.
//! GAP: the "if it's a Bison card, put it onto the battlefield" branch
//! is not expressible (DigTopN's single take always goes to hand), so a
//! Bison creature is taken to hand rather than entering the battlefield.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bison Whistle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}: Look at the top card of your library. If it's a Bison card, you may put it onto the battlefield. If it's a creature card, you may reveal it and put it into your hand. Otherwise, you may put it into your graveyard.".into(),
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
            effect: look_top,
        }),
    )
}

fn look_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..Default::default()
        }),
        rest: DigRest::Graveyard,
    }]
}
