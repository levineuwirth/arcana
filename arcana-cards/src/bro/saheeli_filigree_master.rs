//! Saheeli, Filigree Master — `{2}{U}{R}` Legendary Planeswalker — Saheeli, starting loyalty 5.
//!
//! +1: Scry 1. You may tap an untapped artifact you control. If you do, draw a card.
//!     Scry 1 modeled; the optional "tap an artifact you control, if you do draw"
//!     is a resolution-time cost-then-effect not expressible from the demonstrated
//!     surface — GAP (the tap-and-draw rider is omitted; scry is honest).
//! −2: Create two 1/1 colorless Thopter artifact creature tokens with flying.
//!     They gain haste until end of turn. Tokens carry Flying + Haste keywords.
//! −4: You get an emblem with "Artifact creatures you control get +1/+1" and
//!     "Artifact spells you cast cost {1} less to cast." The +1/+1 static is
//!     modeled via a filtered anthem on the emblem; the cost-reduction static is
//!     a rule-altering effect the builders can't express — GAP (omitted).

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saheeli, Filigree Master");
    let saheeli = reg.interner_mut().intern("Saheeli");
    let _thopter = reg.interner_mut().intern("Thopter");
    let _emblem = reg.interner_mut().intern("Saheeli, Filigree Master emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saheeli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Scry 1. You may tap an untapped artifact you control. \
                       If you do, draw a card.".into(),
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
                text: "-2: Create two 1/1 colorless Thopter artifact creature \
                       tokens with flying. They gain haste until end of turn.".into(),
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
                text: "-4: You get an emblem with \"Artifact creatures you control \
                       get +1/+1\" and \"Artifact spells you cast cost {1} less to \
                       cast.\"".into(),
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
    // GAP: "You may tap an untapped artifact you control. If you do, draw a card."
    //      — the optional tap-an-artifact-cost-then-draw rider is not expressible
    //      from the demonstrated surface. Scry 1 modeled.
    vec![Effect::Scry { player: ctx.controller, count: 1 }]
}

fn minus_two_thopters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").expect("Thopter interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(thopter);
    let make = || Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: thopter,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subtypes.clone(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
            abilities: vec![],
        },
    };
    vec![make(), make()]
}

fn minus_four_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Saheeli, Filigree Master emblem")
        .expect("emblem name interned");
    let artifact_creature = ObjectFilter::new()
        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE));
    // GAP: "Artifact spells you cast cost {1} less to cast." — a cost-reduction
    //      static the demonstrated builders can't express; omitted. The
    //      "Artifact creatures you control get +1/+1" static is modeled.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![ContinuousEffect::filtered_pump(
                NULL_OBJECT_ID,
                artifact_creature,
                1,
                1,
                Duration::Permanent,
            )],
            abilities: vec![],
        },
    }]
}
