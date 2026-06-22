//! Korlash, Heir to Blackblade — `{2}{B}{B}` Legendary */* Zombie Warrior.
//! "Korlash's power and toughness are each equal to the number of
//!  Swamps you control." (characteristic-defining static — GAP; base */* = 0/0)
//! "{1}{B}: Regenerate Korlash."
//! "Grandeur — Discard another card named Korlash, Heir to Blackblade:
//!  Search your library for up to two Swamp cards, put them onto the
//!  battlefield tapped, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Korlash, Heir to Blackblade");
    let zombie = reg.interner_mut().intern("Zombie");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: P/T is a characteristic-defining static (= number of Swamps you
        // control); printed */* recorded as base 0/0.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // Grandeur cost: discard another card named "Korlash, Heir to Blackblade".
    let discard_filter = ObjectFilter {
        name: Some(name),
        ..ObjectFilter::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Regenerate Korlash.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Grandeur — Discard another card named Korlash, Heir to Blackblade: Search your library for up to two Swamp cards, put them onto the battlefield tapped, then shuffle.".into(),
                cost: ActivationCost {
                    discard_other: Some(discard_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grandeur_fetch_swamps,
            }),
    )
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}

fn grandeur_fetch_swamps(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = script::subtype_filter(reg, "Swamp");
    // "up to two Swamp cards … onto the battlefield tapped" — two single
    // tutor-to-battlefield puts (each shuffles); "up to" is a deck-dependent
    // best-effort (no count parameter on TutorToBattlefield).
    vec![
        Effect::TutorToBattlefield {
            player: ctx.controller,
            filter: swamp.clone(),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: ctx.controller,
            filter: swamp,
            tapped: true,
        },
    ]
}
