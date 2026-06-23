//! Harsh Deceiver — `{3}{W}` 1/4 white Spirit.
//! "{1}: Look at the top card of your library."
//! "{2}: Reveal the top card of your library. If it's a land card, untap
//! this creature and it gets +1/+1 until end of turn. Activate only once
//! each turn."
//!
//! Ability 1's payload ("look at the top card") has no faithful primitive
//! (Scry would permit bottoming, which "look" does not), so its effect is
//! GAP'd. Ability 2 is wired as an untap + +1/+1 pump (the expressible
//! payload), gated `once_per_turn`; the "reveal the top card / if it's a
//! land" condition has no top-of-library predicate, so the gate is GAP'd
//! and the untap+pump fire unconditionally on activation.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harsh Deceiver");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{1}: Look at the top card of your library."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Look at the top card of your library.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_at_top,
            })
            // "{2}: Reveal the top card of your library. If it's a land
            // card, untap this creature and it gets +1/+1 until end of turn.
            // Activate only once each turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Reveal the top card of your library. If it's a land card, untap this creature and it gets +1/+1 until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_and_pump,
            }),
    )
}

fn look_at_top(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top card of your library" — no non-mutating
    // look-at-top primitive (Scry would permit bottoming the card).
    Vec::new()
}

fn untap_and_pump(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Reveal the top card ... If it's a land card" — no top-of-library
    // type predicate; the untap + +1/+1 payload fires unconditionally.
    vec![
        Effect::Untap { target: ctx.source },
        Effect::Pump {
            target: ctx.source,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
