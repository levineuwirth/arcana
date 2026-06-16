//! Nahiri, the Unforgiving — `{1}{R}{R/W/P}{W}` legendary planeswalker,
//! starting loyalty 5 (Compleated; the engine handles the life-paid
//! loyalty reduction). Colors: R, W.
//!
//! Keywords: Compleated — not in the usable keyword surface; `keywords:
//! vec![]`.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Until your next turn, up to one target creature attacks a player
//!   each combat if able. — GAP'd (a "must attack" duration effect is not
//!   in the demonstrated surface).
//! * `+1`: Discard a card, then draw a card. — EXPRESSED.
//! * `0`: Exile target creature or Equipment card with mana value less than
//!   Nahiri's loyalty from your graveyard. Create a token that's a copy of
//!   it; that token gains haste; exile it at the next end step. — GAP'd
//!   (graveyard-targeting + token-copy-of-a-card are not in the
//!   demonstrated surface).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nahiri, the Unforgiving");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R/W/P}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target creature \
                       attacks a player each combat if able.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_lure,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Discard a card, then draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Exile target creature or Equipment card with mana \
                       value less than Nahiri's loyalty from your graveyard. \
                       Create a token that's a copy of it. That token gains \
                       haste. Exile it at the beginning of the next end \
                       step.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_reanimate_copy,
            }),
    )
}

/// `+1` (first)
fn plus_one_lure(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "until your next turn, attacks each combat if able" is a
    // forced-attack duration effect, not in the demonstrated surface.
    Vec::new()
}

/// `+1: Discard a card, then draw a card.`
fn plus_one_loot(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
    ]
}

/// `0`
fn zero_reanimate_copy(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: graveyard-targeting (no any-graveyard sentinel) + token-copy of a
    // graveyard card with a delayed-exile rider — none in the demonstrated
    // surface.
    Vec::new()
}
