//! Windswept Heath — nonbasic land (Onslaught fetchland).
//! "{T}, Pay 1 life, Sacrifice this land: Search your library for a
//! Forest or Plains card, put it onto the battlefield, then shuffle."
//!
//! The cost combines tap + 1 life + sacrifice-self; the search is a
//! `TutorToBattlefield` over a Forest-or-Plains subtype-OR filter
//! (subtypes interned at registration, looked up read-only at
//! resolution).

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windswept Heath");
    // Pre-intern the basic land subtypes so the resolver's read-only
    // lookup finds them.
    let _forest = reg.interner_mut().intern("Forest");
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Pay 1 life, Sacrifice this land: Search your \
                       library for a Forest or Plains card, put it onto the \
                       battlefield, then shuffle."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    life: 1,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fetch_forest_or_plains,
            },
        ),
    )
}

fn fetch_forest_or_plains(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest").unwrap_or_default();
    let plains = reg.interner().lookup("Plains").unwrap_or_default();
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtypes_any(vec![forest, plains]);
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
