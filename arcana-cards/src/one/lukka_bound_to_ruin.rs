//! Lukka, Bound to Ruin — `{2}{R}{R/G/P}{G}` Legendary Planeswalker —
//! Lukka, starting loyalty 5.
//!
//! Oracle text:
//! * Compleated ({R/G/P} can be paid with {R}, {G}, or 2 life. If life
//!   was paid, this planeswalker enters with two fewer loyalty
//!   counters.) — the `Compleated` keyword is not part of the usable
//!   keyword surface, so `keywords` is left empty and the
//!   life-payment / reduced-loyalty rider is GAP'd.
//! * `+1`: Add {R}{G}. Spend this mana only to cast creature spells or
//!   activate abilities of creatures.
//! * `−1`: Create a 3/3 green Phyrexian Beast creature token with
//!   toxic 1.
//! * `−4`: Lukka deals X damage divided as you choose among any number
//!   of target creatures and/or planeswalkers, where X is the greatest
//!   power among creatures you control as you activate this ability.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope
//!
//! All three loyalty-ability shells are emitted with their correct
//! loyalty costs, but each effect is GAP'd:
//! * `+1` — adding restricted mana ("spend only to cast creature spells
//!   / activate abilities of creatures") is not expressible with the
//!   demonstrated Effect surface.
//! * `−1` — token creation requires a `TokenDefinition` whose
//!   construction is not demonstrated in the prompt.
//! * `−4` — dynamic `X` (greatest power among your creatures) divided
//!   damage among any number of targets is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lukka, Bound to Ruin");
    let lukka = reg.interner_mut().intern("Lukka");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lukka);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R/G/P}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // Compleated is not in the usable keyword surface; no keywords.
        keywords: vec![],
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}{G}. Spend this mana only to cast creature \
                       spells or activate abilities of creatures.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Create a 3/3 green Phyrexian Beast creature token with \
                       toxic 1.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: Lukka deals X damage divided as you choose among any \
                       number of target creatures and/or planeswalkers, where X \
                       is the greatest power among creatures you control as you \
                       activate this ability.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_damage,
            }),
    )
}

/// `+1: Add {R}{G}. Spend this mana only to cast creature spells ...`
fn plus_one_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: adding restricted-use mana is not expressible with the
    // demonstrated Effect surface.
    Vec::new()
}

/// `−1: Create a 3/3 green Phyrexian Beast creature token with toxic 1.`
fn minus_one_token(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token creation requires a TokenDefinition whose construction
    // is not demonstrated in the prompt.
    Vec::new()
}

/// `−4: Lukka deals X damage divided ... where X is the greatest power ...`
fn minus_four_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic X (greatest power among your creatures) divided damage
    // among any number of targets is not expressible.
    Vec::new()
}
