//! Chandra, Dressed to Kill — `{1}{R}{R}` Legendary Planeswalker —
//! Chandra, starting loyalty 3. Red.
//!
//! Oracle text:
//! * `+1`: Add {R}. Chandra deals 1 damage to up to one target player
//!   or planeswalker.
//! * `+1`: Exile the top card of your library. If it's red, you may
//!   cast it this turn.
//! * `−7`: Exile the top five cards of your library. You may cast red
//!   spells from among them this turn. You get an emblem with "Whenever
//!   you cast a red spell, this emblem deals X damage to any target,
//!   where X is the amount of mana spent to cast that spell."
//!
//! # Scope
//!
//! * First `+1`: add {R}, then 1 damage to an "up to one" target.
//!   There's no player-or-planeswalker-only filter on the surface;
//!   `AnyTarget` is used (it also admits creatures — a minor
//!   over-permission noted here) and the damage routes to either a
//!   Player or an Object.
//! * Second `+1`: "exile top card, if red you may cast it this turn" —
//!   the color-conditional cast-from-exile rider isn't expressible
//!   (ImpulseExile has no color filter and the cast permission is a
//!   per-card rider) — GAP'd.
//! * `−7`: impulse-exile-five-castable-if-red plus a red-spell-cast
//!   emblem — bespoke — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement, ObjectOrPlayer};
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Dressed to Kill");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}. Chandra deals 1 damage to up to one \
                       target player or planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_burn,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile the top card of your library. If it's red, \
                       you may cast it this turn.".into(),
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
                effect: plus_one_impulse,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Exile the top five cards of your library. You may \
                       cast red spells from among them this turn. You get an \
                       emblem with \"Whenever you cast a red spell, this \
                       emblem deals X damage to any target, where X is the \
                       amount of mana spent to cast that spell.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+1`: add {R}, then 1 damage to up to one target.
fn plus_one_burn(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }];
    if let Some(t) = ctx.targets.targets.first() {
        let dt = match t {
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => Some(DamageTarget::Player(*p)),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => Some(DamageTarget::Object(*id)),
        };
        if let Some(dt) = dt {
            effects.push(Effect::DealDamage { source: ctx.source, target: dt, amount: 1 });
        }
    }
    effects
}

/// `+1`: exile top card, if red may cast this turn.
fn plus_one_impulse(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: color-conditional impulse exile ("if it's red, you may cast it
    // this turn") — ImpulseExile has no color filter and the conditional
    // cast permission is not expressible.
    Vec::new()
}

/// `−7`: impulse five (red-castable) + red-spell-cast emblem.
fn minus_seven(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: filtered impulse-of-five plus a red-cast emblem dealing
    // mana-value damage — bespoke and not expressible.
    Vec::new()
}
