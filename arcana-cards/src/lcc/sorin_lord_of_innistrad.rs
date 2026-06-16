//! Sorin, Lord of Innistrad — `{2}{W}{B}` Legendary Planeswalker — Sorin, starting loyalty 4.
//!
//! +1: Create a 1/1 black Vampire creature token with lifelink. (CreateToken.)
//! −2: You get an emblem with "Creatures you control get +1/+0." Modeled via
//!     `CreateEmblem` with a STATIC anthem (+1/+0, Permanent).
//! −6: Destroy up to three target creatures and/or other planeswalkers. Return each
//!     card put into a graveyard this way to the battlefield under your control. The
//!     destroy half is expressed (up to three target permanents → DestroyPermanent);
//!     the "return each to the battlefield under your control" rider is a bespoke
//!     one-shot not expressible here. GAP that rider.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Lord of Innistrad");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 black Vampire creature token with lifelink."
                    .into(),
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
                effect: plus_one_vampire,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: You get an emblem with \"Creatures you control get \
                       +1/+0.\""
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
                effect: minus_two_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Destroy up to three target creatures and/or other \
                       planeswalkers. Return each card put into a graveyard this \
                       way to the battlefield under your control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_destroy,
            }),
    )
}

fn plus_one_vampire(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").expect("Vampire interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(vampire);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: vampire,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Lifelink],
            abilities: vec![],
        },
    }]
}

fn minus_two_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Sorin, Lord of Innistrad").expect("name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![ContinuousEffect::anthem(
                arcana_core::objects::NULL_OBJECT_ID,
                ctx.controller,
                1,
                0,
                Duration::Permanent,
            )],
            abilities: vec![],
        },
    }]
}

fn minus_six_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Return each card put into a graveyard this way to the battlefield under
    //      your control" — a bespoke reanimation rider keyed on what this destroy put
    //      into graveyards is not expressible here. The destroy half is modeled.
    let mut effects = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    effects
}
