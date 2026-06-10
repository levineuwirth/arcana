//! Inscribed Tablet — `{1}` artifact (Ixalan, 2017).
//! "{1}, {T}, Sacrifice this artifact: Reveal the top five cards of your
//! library. Put a land card from among them into your hand and the rest on
//! the bottom of your library in a random order. If you didn't put a card
//! into your hand this way, draw a card."

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
    let name = reg.interner_mut().intern("Inscribed Tablet");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Reveal the top \
                       five cards of your library. Put a land card from \
                       among them into your hand and the rest on the \
                       bottom of your library in a random order. If you \
                       didn't put a card into your hand this way, draw a \
                       card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: dig_for_land,
            },
        ),
    )
}

fn dig_for_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you didn't put a card into your hand this way, draw a
    // card" — the whiff-fallback draw is not expressible alongside
    // DigTopN.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 5,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::LAND.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
