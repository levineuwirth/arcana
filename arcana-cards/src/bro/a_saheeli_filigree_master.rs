//! A-Saheeli, Filigree Master — `{2}{U}{R}` Legendary Planeswalker — Saheeli, starting loyalty 3.
//! +1: Scry 2, then if you control an artifact, draw a card (resolution-time conditional draw).
//! −2: Create two 1/1 colorless Thopter artifact creature tokens with flying. (haste-until-eot rider GAP'd)
//! −4: Emblem — STATIC "Artifact creatures you control get +1/+1" via filtered_pump. The
//!     "artifact spells cost {1} less" half is a cost reduction not expressible from the
//!     anthem/keyword/filtered builders — GAP'd (emblem still emitted with the buildable static).

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Saheeli, Filigree Master");
    let sub = reg.interner_mut().intern("Saheeli");
    let _thopter = reg.interner_mut().intern("Thopter");
    let _emblem = reg.interner_mut().intern("A-Saheeli, Filigree Master emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
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
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create two 1/1 colorless Thopter artifact creature tokens with \
                       flying. They gain haste until end of turn."
                    .into(),
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
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: You get an emblem with \"Artifact creatures you control get \
                       +1/+1\" and \"Artifact spells you cast cost {1} less to cast.\""
                    .into(),
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
                effect: minus_four,
            }),
    )
}

/// `+1: Scry 2. If you control an artifact, draw a card.`
fn plus_one(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::Scry {
        player: ctx.controller,
        count: 2,
    }];
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    if script::count_matching(state, &artifact_filter, ctx.controller) > 0 {
        effects.push(Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        });
    }
    effects
}

/// `−2: Create two 1/1 colorless Thopter artifact creature tokens with flying.`
fn minus_two(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let thopter = reg
        .interner()
        .lookup("Thopter")
        .expect("Thopter interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(thopter);
    let token = TokenDefinition {
        name: thopter,
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: "They gain haste until end of turn." — cannot re-target the freshly-minted
    // token ids from this same effect list.
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

/// `−4: emblem — artifact creatures get +1/+1 (static); artifact-spell cost reduction GAP'd.`
fn minus_four(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("A-Saheeli, Filigree Master emblem")
        .expect("emblem name interned");
    let artifact_creature = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .with_types(TypeLine::CREATURE.into());
    // GAP: "Artifact spells you cast cost {1} less to cast." — cost reduction is not
    // expressible from anthem/keyword/filtered builders. Buildable +1/+1 static emitted.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![ContinuousEffect::filtered_pump(
                arcana_core::objects::NULL_OBJECT_ID,
                artifact_creature,
                1,
                1,
                Duration::Permanent,
            )],
            abilities: vec![],
        },
    }]
}
