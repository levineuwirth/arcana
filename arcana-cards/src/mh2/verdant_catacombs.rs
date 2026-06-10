//! Verdant Catacombs — nonbasic land (fetch land).
//! "{T}, Pay 1 life, Sacrifice this land: Search your library for a
//! Swamp or Forest card, put it onto the battlefield, then shuffle."
//! The fetch is a `TutorToBattlefield` over a Swamp-or-Forest
//! subtype-OR filter; the cost combines tap + 1 life + sacrifice.

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
    let name = reg.interner_mut().intern("Verdant Catacombs");
    // Pre-intern the land subtypes so the resolver's read-only lookup
    // succeeds at resolution time.
    let _swamp = reg.interner_mut().intern("Swamp");
    let _forest = reg.interner_mut().intern("Forest");
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
                       library for a Swamp or Forest card, put it onto the \
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
                effect: fetch_land,
            },
        ),
    )
}

fn fetch_land(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Swamp") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Forest") {
        syms.push(s);
    }
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(syms),
        tapped: false,
    }]
}
