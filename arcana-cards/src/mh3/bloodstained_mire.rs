//! Bloodstained Mire — nonbasic land (Onslaught fetchland).
//! "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp
//! or Mountain card, put it onto the battlefield, then shuffle." The full
//! cost (tap + 1 life + sacrifice-self) sits on one `ActivationCost`; the
//! Swamp-or-Mountain search uses a subtype-OR filter built at resolution
//! from the read-only interner lookup (shuffle is automatic).

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
    let name = reg.interner_mut().intern("Bloodstained Mire");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
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
                       library for a Swamp or Mountain card, put it onto \
                       the battlefield, then shuffle."
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
                effect: fetch_swamp_or_mountain,
            },
        ),
    )
}

fn fetch_swamp_or_mountain(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").unwrap_or_default();
    let mountain = reg.interner().lookup("Mountain").unwrap_or_default();
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(vec![swamp, mountain]),
        tapped: false,
    }]
}
