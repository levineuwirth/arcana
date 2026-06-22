//! Thornvault Forager — `{1}{G}` 2/2 Squirrel Ranger.
//!
//! Oracle:
//! * {T}: Add {G}.
//! * {T}, Forage: Add two mana in any combination of colors. (The Forage cost —
//!   exile three cards from your graveyard or sacrifice a Food — is not an
//!   expressible activation-cost field; GAP'd. The "any combination of colors"
//!   choice is also not expressible, so two green mana is produced as a
//!   fidelity approximation.)
//! * {3}{G}, {T}: Search your library for a Squirrel card, reveal it, put it
//!   into your hand, then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thornvault Forager");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Forage: Add two mana in any combination of colors.".into(),
                // GAP: the Forage cost (exile three cards from your graveyard or
                // sacrifice a Food) has no activation-cost field; only the tap
                // portion is expressed.
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_any,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}, {T}: Search your library for a Squirrel card, reveal it, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_squirrel,
            }),
    )
}

fn add_green(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn add_two_any(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "any combination of colors" player choice — two green mana produced
    // as a fidelity approximation.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); 2],
    }]
}

fn tutor_squirrel(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: script::subtype_filter(reg, "Squirrel"),
        reveal: true,
    }]
}
