//! Mountain Valley — nonbasic land.
//! "This land enters tapped." and "{T}, Sacrifice this land: Search your
//! library for a Mountain or Forest card, put it onto the battlefield, then
//! shuffle."
//! Enters-tapped via `EntersWithSpec::Tapped`; the fetch is a
//! `TutorToBattlefield` over land cards with the Mountain or Forest subtype.

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
    let name = reg.interner_mut().intern("Mountain Valley");
    // Pre-intern the fetch subtypes so the resolver's read-only lookup hits.
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
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
                       Mountain or Forest card, put it onto the battlefield, \
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
                effect: fetch_mountain_or_forest,
            }),
    )
}

fn fetch_mountain_or_forest(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mountain = reg.interner().lookup("Mountain").unwrap_or_default();
    let forest = reg.interner().lookup("Forest").unwrap_or_default();
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(vec![mountain, forest]),
        tapped: false,
    }]
}
