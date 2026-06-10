//! Marsh Flats — nonbasic land (Zendikar, 2009). "{T}, Pay 1 life,
//! Sacrifice this land: Search your library for a Plains or Swamp
//! card, put it onto the battlefield, then shuffle."

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
    let name = reg.interner_mut().intern("Marsh Flats");
    // Pre-intern the fetched basic land subtypes for the resolver's
    // read-only lookup.
    let _plains = reg.interner_mut().intern("Plains");
    let _swamp = reg.interner_mut().intern("Swamp");
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
                       library for a Plains or Swamp card, put it onto the \
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
                effect: fetch_plains_or_swamp,
            },
        ),
    )
}

fn fetch_plains_or_swamp(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plains = reg.interner().lookup("Plains").unwrap_or_default();
    let swamp = reg.interner().lookup("Swamp").unwrap_or_default();
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(vec![plains, swamp]),
        tapped: false,
    }]
}
