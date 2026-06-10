//! Flooded Strand — nonbasic land.
//! "{T}, Pay 1 life, Sacrifice this land: Search your library for a
//! Plains or Island card, put it onto the battlefield, then shuffle."
//! The classic fetchland: tap + 1 life + sacrifice on one
//! `ActivationCost`, fetching any Plains- or Island-typed land
//! (not restricted to basics) untapped.

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
    let name = reg.interner_mut().intern("Flooded Strand");
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
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Island card, put it onto the battlefield, then shuffle.".into(),
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
            effect: fetch_plains_or_island,
        }),
    )
}

fn fetch_plains_or_island(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subs = Vec::new();
    for n in ["Plains", "Island"] {
        if let Some(s) = reg.interner().lookup(n) {
            subs.push(s);
        }
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
