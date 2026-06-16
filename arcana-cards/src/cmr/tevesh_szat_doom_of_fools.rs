//! Tevesh Szat, Doom of Fools — `{4}{B}` Legendary Planeswalker — Szat.
//! Printed starting loyalty 5 (CR 113.3c). Color black.
//!
//! Keyword: Partner — not in the usable keyword surface; `keywords:
//! vec![]`, GAP. "Tevesh Szat can be your commander" is a command-zone
//! eligibility static, not a loyalty ability — GAP.
//!
//! Loyalty abilities (CR 606):
//! * `+2`: Create two 0/1 black Thrull creature tokens.
//! * `+1`: You may sacrifice another creature or planeswalker. If you do,
//!   draw two cards, then draw another if the sacrificed permanent was a
//!   commander. — GAP (OptionalPayment supports only Mana/Life costs, not
//!   sacrifice; the commander-conditional extra draw is also unbuilt).
//! * `−10`: Gain control of all commanders; put all commanders from the
//!   command zone onto the battlefield under your control. — GAP
//!   (command-zone reanimation + mass control has no demonstrated
//!   surface).
//!
//! Only the `+2` is expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tevesh Szat, Doom of Fools");
    let szat = reg.interner_mut().intern("Szat");
    let _thrull = reg.interner_mut().intern("Thrull");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(szat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        // GAP: Partner is not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Create two 0/1 black Thrull creature tokens.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_thrulls,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You may sacrifice another creature or planeswalker. If \
                       you do, draw two cards, then draw another card if the \
                       sacrificed permanent was a commander.".into(),
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
                effect: plus_one_sac,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Gain control of all commanders. Put all commanders from \
                       the command zone onto the battlefield under your control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_commanders,
            }),
    )
}

/// `+2`: two 0/1 black Thrull tokens.
fn plus_two_thrulls(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thrull = reg
        .interner()
        .lookup("Thrull")
        .expect("Thrull interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);
    let token = TokenDefinition {
        name: thrull,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token,
        },
    ]
}

/// `+1`: optional sacrifice-then-draw — GAP.
fn plus_one_sac(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: OptionalPayment supports only Mana/Life costs, not "sacrifice a
    // creature or planeswalker"; the commander-conditional extra draw is
    // also unbuilt.
    Vec::new()
}

/// `−10`: command-zone mass reanimation + control — GAP.
fn minus_ten_commanders(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: putting all commanders from the command zone onto the
    // battlefield under your control has no demonstrated surface.
    Vec::new()
}
