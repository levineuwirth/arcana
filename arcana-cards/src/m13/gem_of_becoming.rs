//! Gem of Becoming — `{3}` artifact (Magic 2013).
//! "{3}, {T}, Sacrifice this artifact: Search your library for an
//! Island card, a Swamp card, and a Mountain card. Reveal those cards,
//! put them into your hand, then shuffle."
//!
//! Modeled as three `TutorToHand` searches in sequence (one per named
//! basic-land subtype); the shuffle is automatic.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gem of Becoming");
    // Pre-intern the searched subtypes for the resolver's filters.
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{3}, {T}, Sacrifice this artifact: Search your library \
                       for an Island card, a Swamp card, and a Mountain card. \
                       Reveal those cards, put them into your hand, then \
                       shuffle."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
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
                effect: fetch_three_lands,
            },
        ),
    )
}

fn fetch_three_lands(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::TutorToHand {
            player: ctx.controller,
            filter: script::subtype_filter(reg, "Island"),
            reveal: true,
        },
        Effect::TutorToHand {
            player: ctx.controller,
            filter: script::subtype_filter(reg, "Swamp"),
            reveal: true,
        },
        Effect::TutorToHand {
            player: ctx.controller,
            filter: script::subtype_filter(reg, "Mountain"),
            reveal: true,
        },
    ]
}
