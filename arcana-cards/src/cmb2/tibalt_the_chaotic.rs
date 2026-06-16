//! Tibalt the Chaotic — `{1}{R}{R}` Legendary Planeswalker — Tibalt,
//! starting loyalty 5.
//!
//! +1: Cast a copy of one of the following cards chosen at random — Ignorant
//!     Bliss, Crack the Earth, Blazing Volley.
//! −3: Cast a copy of one of the following cards chosen at random — Seething
//!     Song, Dance with Devils, Flamebreak.
//! −6: Cast a copy of one of the following cards chosen at random — Hellion
//!     Eruption, Insurrection, Warp World.
//!
//! GAP (all three): "cast a copy of a card chosen at random from a fixed
//! named list" has no demonstrated primitive. The ability shells carry the
//! correct loyalty costs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tibalt the Chaotic");
    let tibalt = reg.interner_mut().intern("Tibalt");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tibalt);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Cast a copy of one of the following cards chosen at \
                       random — Ignorant Bliss, Crack the Earth, Blazing Volley.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_cast_random,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Cast a copy of one of the following cards chosen at \
                       random — Seething Song, Dance with Devils, Flamebreak.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_cast_random,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Cast a copy of one of the following cards chosen at \
                       random — Hellion Eruption, Insurrection, Warp World.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_cast_random,
            }),
    )
}

fn gap_cast_random(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cast a copy of a random card from a fixed named list.
    Vec::new()
}
