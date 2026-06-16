//! A-Nahiri, Heir of the Ancients — `{2}{R}{W}` Legendary Planeswalker —
//! Nahiri, starting loyalty 3 — colors R, W.
//!
//! Oracle text:
//! * `+1`: Create a 1/1 white Kor Warrior creature token. You may attach an
//!   Equipment you control to it. — `CreateToken` is faithful; the "may attach
//!   an Equipment" rider is GAP'd (still emit the token).
//! * `−2`: Look at the top six cards of your library. You may reveal a Warrior
//!   card and/or an Equipment card from among them and put them into your hand.
//!   Put the rest on the bottom of your library in a random order. — a "Warrior
//!   and/or Equipment" dual take with an OR'd type/subtype filter is not
//!   cleanly expressible via a single `DigTopN`. GAP.
//! * `−3`: Nahiri deals damage to target creature or planeswalker equal to
//!   twice the number of Warriors and Equipment you control. — dynamic
//!   `DealDamage`; amount = 2 × (Warriors + Equipment you control).
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Nahiri, Heir of the Ancients");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let _kor = reg.interner_mut().intern("Kor");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _equipment = reg.interner_mut().intern("Equipment");
    let _token_name = reg.interner_mut().intern("Kor Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 white Kor Warrior creature token. You \
                       may attach an Equipment you control to it.".into(),
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
                effect: plus_one_kor_warrior,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Look at the top six cards of your library. You may \
                       reveal a Warrior card and/or an Equipment card from \
                       among them and put them into your hand. Put the rest on \
                       the bottom of your library in a random order.".into(),
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
                effect: minus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Nahiri deals damage to target creature or \
                       planeswalker equal to twice the number of Warriors and \
                       Equipment you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(
                            (TypeLine::CREATURE | TypeLine::PLANESWALKER).into(),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            }),
    )
}

/// `+1`: create a 1/1 white Kor Warrior token (attach-Equipment rider GAP'd).
fn plus_one_kor_warrior(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "You may attach an Equipment you control to it" rider is not
    // expressible; the token creation is faithful.
    let token_name = match reg.interner().lookup("Kor Warrior") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Kor") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Warrior") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−2`: dig 6 with a "Warrior and/or Equipment" dual take.
fn minus_two_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a single DigTopN can take at most one card and cannot express a
    // "Warrior card and/or Equipment card" OR-filtered dual selection.
    Vec::new()
}

/// `−3`: deal 2 × (Warriors + Equipment you control) to a creature/PW.
fn minus_three_damage(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    let mut count = 0u32;
    if let Some(warrior) = reg.interner().lookup("Warrior") {
        let f = ObjectFilter::creature()
            .with_subtype_sym(warrior)
            .controlled_by(ControllerConstraint::You);
        count += script::count_matching(state, &f, ctx.controller);
    }
    let equipment_sym = reg.interner().lookup("Equipment");
    if let Some(equip) = equipment_sym {
        let f = ObjectFilter::permanent()
            .with_subtype_sym(equip)
            .controlled_by(ControllerConstraint::You);
        count += script::count_matching(state, &f, ctx.controller);
    }
    let amount = count * 2;
    vec![Effect::DealDamage {
        source: ctx.source,
        target,
        amount,
    }]
}
