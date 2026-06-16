//! Davriel, Soul Broker — `{2}{B}{B}` Legendary Planeswalker — Davriel,
//! starting loyalty 5. Mono-black.
//!
//! Oracle text:
//! * `+1`: Until your next turn, whenever an opponent attacks you and/or
//!   planeswalkers you control, they discard a card. If they can't, they
//!   sacrifice an attacking creature.
//! * `−2`: Accept one of Davriel's offers, then accept one of Davriel's
//!   conditions.
//! * `−3`: Target creature an opponent controls perpetually gets -3/-3.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 606.3 — controller-only, sorcery speed, stack empty, once per
//!   turn per planeswalker.
//! * CR 704.5i — 0-loyalty SBA sacrifice.
//!
//! # Scope
//!
//! * `+1` installs an until-your-next-turn delayed conditional combat
//!   trigger ("whenever an opponent attacks…") — not in the demonstrated
//!   one-shot Effect surface — GAP.
//! * `−2` "accept offers/conditions" is an Alchemy-style bespoke
//!   sub-menu mechanic — GAP.
//! * `−3` "perpetually gets -3/-3" is a perpetual modification (Alchemy);
//!   not expressible with end-of-turn/while-on-battlefield Durations —
//!   GAP.
//! All three loyalty ability shells are declared with their correct
//! loyalty costs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Davriel, Soul Broker");
    let davriel = reg.interner_mut().intern("Davriel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(davriel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, whenever an opponent \
                       attacks you and/or planeswalkers you control, they \
                       discard a card. If they can't, they sacrifice an \
                       attacking creature.".into(),
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
                effect: plus_one_attack_punish,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Accept one of Davriel's offers, then accept one \
                       of Davriel's conditions.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_offers,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature an opponent controls perpetually \
                       gets -3/-3.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_perpetual,
            }),
    )
}

/// `+1: Until your next turn, whenever an opponent attacks…`
fn plus_one_attack_punish(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: installs an until-your-next-turn delayed combat trigger;
    // not in the demonstrated one-shot Effect surface.
    Vec::new()
}

/// `-2: Accept one of Davriel's offers, then accept one of his conditions.`
fn minus_two_offers(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Alchemy-style bespoke offers/conditions sub-menu mechanic.
    Vec::new()
}

/// `-3: Target creature an opponent controls perpetually gets -3/-3.`
fn minus_three_perpetual(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: perpetual modification (Alchemy) is not expressible with the
    // demonstrated end-of-turn / while-on-battlefield Durations.
    Vec::new()
}
