//! Vivien Reid — `{3}{G}{G}` Legendary Planeswalker — Vivien, starting loyalty
//! 5. Mono-green.
//!
//! +1: Look at the top four cards of your library. You may reveal a creature or
//!   land card and put it into your hand; rest on the bottom in a random order.
//!   Modeled with `Effect::DigTopN { count: 4, filter: creature-or-land, rest:
//!   BottomRandom }`.
//! −3: Destroy target artifact, enchantment, or creature with flying. The
//!   type-OR-keyword disjunction is expressed via the ObjectFilter `custom`
//!   predicate; `Effect::DestroyPermanent`.
//! −8: emblem ("Creatures you control get +2/+2 and have vigilance, trample, and
//!   indestructible."). Static emblem: +2/+2 anthem plus vigilance/trample/
//!   indestructible keyword anthems.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, DigRest};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien Reid");
    let vivien = reg.interner_mut().intern("Vivien");
    let _emblem = reg.interner_mut().intern("Vivien Reid emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top four cards of your library. You may \
                       reveal a creature or land card from among them and put \
                       it into your hand. Put the rest on the bottom of your \
                       library in a random order.".into(),
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
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target artifact, enchantment, or creature \
                       with flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        custom: Some(art_ench_or_flier),
                        ..Default::default()
                    }),
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
                text: "-8: You get an emblem with \"Creatures you control get \
                       +2/+2 and have vigilance, trample, and \
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

fn art_ench_or_flier(o: &GameObject, s: &GameState) -> bool {
    o.is_artifact()
        || o.is_enchantment()
        || (o.is_creature() && s.has_keyword(o.id, &KeywordAbility::Flying))
}

fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(ObjectFilter::new().with_types_any(
            (TypeLine::CREATURE | TypeLine::LAND).into(),
        )),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Vivien Reid emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![
                ContinuousEffect::anthem(NULL_OBJECT_ID, ctx.controller, 2, 2, Duration::Permanent),
                ContinuousEffect::keyword_anthem(NULL_OBJECT_ID, ctx.controller, KeywordAbility::Vigilance, Duration::Permanent),
                ContinuousEffect::keyword_anthem(NULL_OBJECT_ID, ctx.controller, KeywordAbility::Trample, Duration::Permanent),
                ContinuousEffect::keyword_anthem(NULL_OBJECT_ID, ctx.controller, KeywordAbility::Indestructible, Duration::Permanent),
            ],
            abilities: Vec::new(),
        },
    }]
}
