//! Daretti, Ingenious Iconoclast — `{1}{B}{R}` planeswalker, starting
//! loyalty 3. Legendary Planeswalker — Daretti.
//!
//! Oracle text:
//! * `+1`: Create a 1/1 colorless Construct artifact creature token
//!   with defender.
//! * `−1`: You may sacrifice an artifact. If you do, destroy target
//!   artifact or creature.
//! * `−6`: Choose target artifact card in a graveyard or artifact on
//!   the battlefield. Create three tokens that are copies of it.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 704.5i — a planeswalker with 0 loyalty is sacrificed (SBA).
//!
//! # Scope
//!
//! All three abilities have their loyalty shells declared, but every
//! effect body is GAP'd:
//! * `+1` creates a token (TokenDefinition builder not demonstrated).
//! * `−1` is gated on an OPTIONAL SACRIFICE additional cost
//!   ("you may sacrifice an artifact. If you do, destroy target artifact or
//!   creature") — wired as an OptionalPayment (Sacrifice(Artifact) → destroy
//!   the chosen target; decline destroys nothing).
//! * `−6` targets a card in a graveyard (no any-graveyard sentinel) and
//!   creates token copies (token-copy not demonstrated).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daretti, Ingenious Iconoclast");
    let daretti = reg.interner_mut().intern("Daretti");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(daretti);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 colorless Construct artifact creature \
                       token with defender.".into(),
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
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: You may sacrifice an artifact. If you do, destroy \
                       target artifact or creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::CREATURE,
                        )),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Choose target artifact card in a graveyard or \
                       artifact on the battlefield. Create three tokens that \
                       are copies of it.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate_copy,
            }),
    )
}

/// `+1: Create a 1/1 colorless Construct artifact creature token with
/// defender.`
fn plus_one_token(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token creation requires a TokenDefinition builder not in the
    // demonstrated surface.
    Vec::new()
}

/// `−1: You may sacrifice an artifact. If you do, destroy target
/// artifact or creature.`
fn minus_one_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may sacrifice an artifact. If you do, destroy target artifact or
    // creature." Pay = sacrifice an artifact then destroy the chosen target;
    // decline destroys nothing.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: ctx.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Artifact),
        then: Box::new(Effect::DestroyPermanent { target: *id }),
        else_effect: None,
    }]
}

/// `−6: Choose target artifact card in a graveyard or artifact on the
/// battlefield. Create three tokens that are copies of it.`
fn ultimate_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: targets a card in a graveyard (no any-graveyard sentinel in
    // the demonstrated surface) and creates token copies (token-copy not
    // demonstrated).
    Vec::new()
}
