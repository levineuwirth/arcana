//! Flood Plain — nonbasic land (Mirage slow fetchland).
//! "This land enters tapped." and "{T}, Sacrifice this land: Search
//! your library for a Plains or Island card, put it onto the
//! battlefield, then shuffle."
//!
//! Enters tapped plus a tap + sacrifice-self fetch activation over a
//! Plains-or-Island subtype-OR filter.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flood Plain");
    // Pre-intern the basic land subtypes so the resolver's read-only
    // lookup finds them.
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
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
                text: "{T}, Sacrifice this land: Search your library for a \
                       Plains or Island card, put it onto the battlefield, \
                       then shuffle."
                    .into(),
                cost: ActivationCost {
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
                effect: fetch_plains_or_island,
            }),
    )
}

fn fetch_plains_or_island(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plains = reg.interner().lookup("Plains").unwrap_or_default();
    let island = reg.interner().lookup("Island").unwrap_or_default();
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtypes_any(vec![plains, island]);
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
