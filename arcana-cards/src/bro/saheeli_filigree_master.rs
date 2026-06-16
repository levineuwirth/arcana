//! Saheeli, Filigree Master — `{2}{U}{R}` Legendary Planeswalker — Saheeli.
//! Starting loyalty inferred 5.
//! +1: Scry 1. You may tap an untapped artifact you control. If you do, draw
//!   a card.
//! −2: Create two 1/1 colorless Thopter artifact creature tokens with flying.
//!   They gain haste until end of turn.
//! −4: You get an emblem with "Artifact creatures you control get +1/+1" and
//!   "Artifact spells you cast cost {1} less to cast."
//!
//! GAP: +1 "you may tap an untapped artifact you control; if you do, draw a
//!   card" — the optional tap-as-cost gating a draw is not expressible
//!   (OptionalPayment is Mana/Life only). The Scry 1 portion is modeled.
//! NOTE: −2 tokens are given Haste in their keyword list; the printed
//!   "until end of turn" duration is approximated as permanent on the token
//!   (no per-new-token id is available at resolution to grant a timed keyword).
//! GAP: −4 emblem statics (artifact-creature anthem + artifact-spell cost
//!   reduction) are not expressible (EmblemDefinition carries only triggered
//!   abilities). Declared with the correct −4 cost, effect GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Saheeli, Filigree Master");
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
                text: "+1: Scry 1. You may tap an untapped artifact you control. If you \
                       do, draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_scry,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create two 1/1 colorless Thopter artifact creature tokens \
                       with flying. They gain haste until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_thopters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: You get an emblem with \"Artifact creatures you control get \
                       +1/+1\" and \"Artifact spells you cast cost {1} less to cast.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_emblem,
            }),
    )
}

fn plus_one_scry(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional tap-an-artifact gating a draw not expressible.
    vec![Effect::Scry { player: ctx.controller, count: 1 }]
}

fn minus_two_thopters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").expect("Thopter interned");
    let make = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(thopter);
        let token = TokenDefinition {
            name: thopter,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
            abilities: vec![],
        };
        Effect::CreateToken { controller: ctx.controller, token }
    };
    vec![make(), make()]
}

fn minus_four_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem statics (anthem + cost reduction) not expressible.
    Vec::new()
}
