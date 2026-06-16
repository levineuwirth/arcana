//! Elspeth, Sun's Champion — `{4}{W}{W}` Legendary Planeswalker — Elspeth, starting loyalty 4.
//!
//! +1: Create three 1/1 white Soldier creature tokens.
//! −3: Destroy all creatures with power 4 or greater. Modeled with a
//!   `ForEach` over the matching creatures + `DestroyPermanent`.
//! −7: You get an emblem with "Creatures you control get +2/+2 and have
//!   flying." Implemented as a static emblem with an anthem and a
//!   keyword anthem.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth, Sun's Champion");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let _soldier = reg.interner_mut().intern("Soldier");
    let _emblem = reg.interner_mut().intern("Elspeth, Sun's Champion emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create three 1/1 white Soldier creature tokens.".into(),
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
                effect: plus_one_soldiers,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy all creatures with power 4 or greater.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_wrath,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Creatures you control get \
                       +2/+2 and have flying.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_soldiers(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned at register");
    let make = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(soldier);
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: soldier,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    vec![make(), make(), make()]
}

fn minus_three_wrath(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ForEach {
        targets: script::ids_matching(
            state,
            &ObjectFilter::creature().with_min_power(4),
            ctx.controller,
        ),
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Elspeth, Sun's Champion emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![
                ContinuousEffect::anthem(NULL_OBJECT_ID, ctx.controller, 2, 2, Duration::Permanent),
                ContinuousEffect::keyword_anthem(
                    NULL_OBJECT_ID,
                    ctx.controller,
                    KeywordAbility::Flying,
                    Duration::Permanent,
                ),
            ],
            abilities: Vec::new(),
        },
    }]
}
