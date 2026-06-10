//! Arid Mesa — nonbasic land (Zendikar, 2009).
//! "{T}, Pay 1 life, Sacrifice this land: Search your library for a
//! Mountain or Plains card, put it onto the battlefield, then shuffle."
//! The classic fetchland: tap + 1 life + sacrifice-self cost, then a
//! tutor-to-battlefield over a Mountain-or-Plains subtype filter.
//! The subtype symbols are interned at registration and looked up
//! read-only in the resolver.

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
    let name = reg.interner_mut().intern("Arid Mesa");
    // Pre-intern the searched subtypes so the resolver's read-only lookup
    // finds them.
    let _mountain = reg.interner_mut().intern("Mountain");
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
                       library for a Mountain or Plains card, put it onto \
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
                effect: fetch_mountain_or_plains,
            },
        ),
    )
}

fn fetch_mountain_or_plains(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subs = Vec::new();
    if let Some(s) = reg.interner().lookup("Mountain") {
        subs.push(s);
    }
    if let Some(s) = reg.interner().lookup("Plains") {
        subs.push(s);
    }
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtypes_any(subs);
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
