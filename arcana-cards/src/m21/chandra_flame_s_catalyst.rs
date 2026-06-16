//! Chandra, Flame's Catalyst — `{4}{R}{R}` Legendary Planeswalker — Chandra,
//! starting loyalty 5. Mono-red.
//!
//! Oracle:
//! +1: Chandra deals 3 damage to each opponent.
//! −2: You may cast target red instant or sorcery card from your graveyard. If
//!     that spell would be put into your graveyard, exile it instead.
//! −8: Discard your hand, then draw seven cards. Until end of turn, you may
//!     cast spells from your hand without paying their mana costs.
//!
//! # Scope
//! * +1 — modeled: 3 damage to each opponent.
//! * −2 — GAP: graveyard-targeting cast (no any-graveyard target sentinel).
//! * −8 — partial: discard your hand, then draw seven cards. GAP: the
//!   "until end of turn, cast spells from hand without paying" blanket
//!   free-cast window is not expressible (CastFromHandFree is per-target).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Flame's Catalyst");
    let sub = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
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
                text: "+1: Chandra deals 3 damage to each opponent.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_each_opp,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You may cast target red instant or sorcery card from \
                       your graveyard. If that spell would be put into your \
                       graveyard, exile it instead.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Discard your hand, then draw seven cards. Until end \
                       of turn, you may cast spells from your hand without \
                       paying their mana costs.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_wheel,
            }),
    )
}

/// `+1:` Chandra deals 3 damage to each opponent.
fn plus_one_each_opp(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    state
        .opponents_of(ctx.controller)
        .map(|p| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 3,
        })
        .collect()
}

/// `−2` — GAP: cast a card from your graveyard (graveyard-targeting).
fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: target red instant/sorcery in your graveyard (no sentinel).
    Vec::new()
}

/// `−8:` discard your hand, then draw seven (free-cast rider GAP'd).
fn minus_eight_wheel(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let hand = state
        .objects_in_zone(Zone::Hand(ctx.controller))
        .count() as u32;
    let mut effects = Vec::new();
    if hand > 0 {
        effects.push(Effect::Discard {
            player: ctx.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    // GAP: "until end of turn, cast spells from hand without paying" blanket
    // free-cast window.
    effects.push(Effect::DrawCards {
        player: ctx.controller,
        count: 7,
    });
    effects
}
