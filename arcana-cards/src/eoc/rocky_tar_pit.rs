//! Rocky Tar Pit — nonbasic land.
//! "This land enters tapped." and "{T}, Sacrifice this land: Search
//! your library for a Swamp or Mountain card, put it onto the
//! battlefield, then shuffle." Enters-tapped plus a sacrifice-to-fetch
//! activation (`Effect::TutorToBattlefield` with a subtype-OR filter).

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
    let name = reg.interner_mut().intern("Rocky Tar Pit");
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
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice this land: Search your library for a \
                       Swamp or Mountain card, put it onto the battlefield, \
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
                effect: fetch_swamp_or_mountain,
            }),
    )
}

fn fetch_swamp_or_mountain(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subtypes = Vec::new();
    if let Some(s) = reg.interner().lookup("Swamp") {
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
