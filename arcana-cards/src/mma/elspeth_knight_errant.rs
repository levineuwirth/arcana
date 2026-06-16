//! Elspeth, Knight-Errant — `{2}{W}{W}` Legendary Planeswalker — Elspeth,
//! starting loyalty 4.
//!
//! +1: Create a 1/1 white Soldier creature token.
//! +1: Target creature gets +3/+3 and gains flying until end of turn.
//! −8: You get an emblem with "Artifacts, creatures, enchantments, and lands you
//!   control have indestructible." Modeled as a STATIC emblem granting
//!   Indestructible to permanents you control via `filtered_keyword`.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth, Knight-Errant");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let _soldier = reg.interner_mut().intern("Soldier");
    let _emblem = reg.interner_mut().intern("Elspeth, Knight-Errant emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
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
                text: "+1: Create a 1/1 white Soldier creature token.".into(),
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
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target creature gets +3/+3 and gains flying until end \
                       of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Artifacts, creatures, \
                       enchantments, and lands you control have \
                       indestructible.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
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
    }]
}

fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Elspeth, Knight-Errant emblem").expect("name interned");
    let filter = ObjectFilter::permanent().controlled_by(ControllerConstraint::You);
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![ContinuousEffect::filtered_keyword(
                NULL_OBJECT_ID,
                filter,
                KeywordAbility::Indestructible,
                Duration::Permanent,
            )],
            abilities: Vec::new(),
        },
    }]
}
