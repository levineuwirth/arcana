//! Jandor's Ring — `{6}` artifact.
//! "{2}, {T}, Discard the last card you drew this turn: Draw a card."
//! Modeled as mana + tap + discard-a-card cost, drawing a card.
//! GAP fidelity: 'the last card you drew this turn' is approximated by
//! a discard of any chosen card (the cost filter cannot reference the
//! draw history); the real ability is also unactivatable if you drew
//! no card this turn.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Jandor's Ring");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}, Discard the last card you drew this turn: Draw a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                // GAP: 'the last card you drew this turn' — the discard
                // cost filter cannot reference draw history; any card
                // may be chosen.
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: ring_draw,
        }),
    )
}

fn ring_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
