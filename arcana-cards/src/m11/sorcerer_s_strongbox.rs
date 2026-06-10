//! Sorcerer's Strongbox — `{4}` artifact (Core Set 2020, 2019).
//! "{2}, {T}: Flip a coin. If you win the flip, sacrifice this
//! artifact and draw three cards."
//! One activated ability whose effect is a coin flip; the win branch
//! sacrifices this artifact (name-filtered) and draws three.

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
    let name = reg.interner_mut().intern("Sorcerer's Strongbox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Flip a coin. If you win the flip, sacrifice this artifact and draw three cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: flip_for_cards,
            },
        ),
    )
}

fn flip_for_cards(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Sorcerer's Strongbox");
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::Sequence(vec![
            Effect::Sacrifice {
                player: ctx.controller,
                filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
                count: 1,
            },
            Effect::DrawCards { player: ctx.controller, count: 3 },
        ])),
        lose: None,
    }]
}
