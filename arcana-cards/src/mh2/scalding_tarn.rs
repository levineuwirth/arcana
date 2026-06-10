//! Scalding Tarn — nonbasic land (fetchland).
//! "{T}, Pay 1 life, Sacrifice this land: Search your library for an
//! Island or Mountain card, put it onto the battlefield, then
//! shuffle." A single fetch activation: tap + 1 life + sacrifice-self
//! cost, `Effect::TutorToBattlefield` with a subtype-OR filter.

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
    let name = reg.interner_mut().intern("Scalding Tarn");
    let _island = reg.interner_mut().intern("Island");
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
                       library for an Island or Mountain card, put it onto \
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
                effect: fetch_island_or_mountain,
            },
        ),
    )
}

fn fetch_island_or_mountain(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subtypes = Vec::new();
    if let Some(s) = reg.interner().lookup("Island") {
        subtypes.push(s);
    }
    if let Some(s) = reg.interner().lookup("Mountain") {
        subtypes.push(s);
    }
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new().with_subtypes_any(subtypes),
        tapped: false,
    }]
}
