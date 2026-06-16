//! A-Saheeli, Filigree Master — `{2}{U}{R}` Legendary Planeswalker — Saheeli,
//! starting loyalty 5.
//!
//! Loyalty abilities:
//! * `+1`: Scry 2. If you control an artifact, draw a card. (Scry +
//!   `Conditional` draw gated on controlling an artifact.)
//! * `−2`: Create two 1/1 colorless Thopter artifact creature tokens with
//!   flying. They gain haste until end of turn. The tokens are minted with
//!   Flying AND Haste (the haste-until-EOT grant on freshly-made tokens is
//!   faithful within the turn; baking it onto the token avoids the
//!   not-expressible post-creation per-id grant).
//! * `−4`: emblem. GAP — anthem + cost-reduction emblem not expressible.
//!   Ability shell declared.

use arcana_core::effects::{Condition, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Saheeli, Filigree Master");
    let saheeli = reg.interner_mut().intern("Saheeli");
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saheeli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Scry 2. If you control an artifact, draw a card.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create two 1/1 colorless Thopter artifact creature \
                       tokens with flying. They gain haste until end of turn.".into(),
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
                effect: minus_two_thopters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: You get an emblem with \"Artifact creatures you \
                       control get +1/+1\" and \"Artifact spells you cast cost \
                       {1} less to cast.\"".into(),
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
                effect: ultimate,
            }),
    )
}

/// `+1: Scry 2. If you control an artifact, draw a card.`
fn plus_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Scry { player: ctx.controller, count: 2 },
        Effect::Conditional {
            condition: Condition::ControlPermanentMatching(
                ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
            ),
            then: Box::new(Effect::DrawCards { player: ctx.controller, count: 1 }),
            otherwise: None,
        },
    ]
}

/// `−2: Create two 1/1 colorless Thopter artifact creature tokens with flying
/// and haste.`
fn minus_two_thopters(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").expect("Thopter interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    let token = TokenDefinition {
        name: thopter,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

/// `−4`: emblem.
fn ultimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem with an artifact-creature anthem + artifact cost reduction is
    // not expressible.
    Vec::new()
}
