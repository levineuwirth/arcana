//! Garruk, Cursed Huntsman — `{4}{B}{G}` Legendary Planeswalker — Garruk,
//! starting loyalty 5.
//!
//! 0: Create two 2/2 black and green Wolf creature tokens with "When this token
//!   dies, put a loyalty counter on each Garruk you control." Modeled via two
//!   CreateToken effects; each token carries a SelfDies triggered ability that
//!   sweeps the controller's Garruk-subtype planeswalkers.
//! −3: Destroy target creature. Draw a card.
//! −6: You get an emblem with "Creatures you control get +3/+3 and have
//!   trample." Modeled as a STATIC emblem: an anthem (+3/+3) plus a Trample
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
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Cursed Huntsman");
    let garruk = reg.interner_mut().intern("Garruk");
    let _wolf = reg.interner_mut().intern("Wolf");
    let _emblem = reg.interner_mut().intern("Garruk, Cursed Huntsman emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create two 2/2 black and green Wolf creature tokens \
                       with \"When this token dies, put a loyalty counter on each \
                       Garruk you control.\"".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_wolves,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target creature. Draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
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
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Creatures you control get \
                       +3/+3 and have trample.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn zero_wolves(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf").expect("Wolf interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(wolf);
    let token = TokenDefinition {
        name: wolf,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: wolf_dies_loyalty,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

fn wolf_dies_loyalty(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let garruk = reg.interner().lookup("Garruk").expect("Garruk interned at register");
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::PLANESWALKER.into())
        .with_subtype_sym(garruk)
        .controlled_by(ControllerConstraint::You);
    script::ids_matching(state, &filter, trig.controller)
        .into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::Loyalty,
            count: 1,
        })
        .collect()
}

fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Garruk, Cursed Huntsman emblem").expect("name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![
                ContinuousEffect::anthem(
                    NULL_OBJECT_ID,
                    ctx.controller,
                    3,
                    3,
                    Duration::Permanent,
                ),
                ContinuousEffect::keyword_anthem(
                    NULL_OBJECT_ID,
                    ctx.controller,
                    KeywordAbility::Trample,
                    Duration::Permanent,
                ),
            ],
            abilities: Vec::new(),
        },
    }]
}
