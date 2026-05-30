//! Artificer Class — `{1}{U}` blue Enchantment — Class.
//! Level 1: "The first artifact spell you cast each turn costs {1} less to cast."
//! Level 2 ({1}{U}): "When this Class becomes level 2, reveal cards from the top
//!   of your library until you reveal an artifact card. Put that card into your
//!   hand and the rest on the bottom of your library in a random order."
//! Level 3 ({5}{U}): "At the beginning of your end step, create a token that's a
//!   copy of target artifact you control."
//!
//! GAP: Level 1 "first artifact spell you cast each turn costs {1} less" is a
//!   cost-reduction continuous effect not expressible with install-on-level-up
//!   (no ContinuousEffect builder for cost reduction).
//! GAP: Level 3 "at the beginning of your end step, create a token that's a copy
//!   of target artifact you control" is a triggered ability installed on level-up
//!   that requires targeting from a triggered context; not yet expressible.

use arcana_core::effects::{Effect, RevealDest, DigRest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artificer Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 2: {1}{U} — reveal until artifact, put it in hand.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 3: {5}{U} — at beginning of your end step, copy target artifact.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{U}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{U}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

/// Level 2: add Level counter + reveal until artifact card, put it in hand.
fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::RevealUntil {
            player: ctx.controller,
            filter: artifact_filter,
            found_dest: RevealDest::Hand,
            rest: DigRest::BottomRandom,
            max_reveal: None,
        },
    ]
}

/// Level 3: add Level counter. "At the beginning of your end step, create a
/// token that's a copy of target artifact you control" is a triggered ability
/// installed on level-up; not expressible with the current install-on-level-up
/// builders (which only support anthem/keyword-anthem).
/// GAP: copy-artifact end-step trigger not modeled.
fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "At the beginning of your end step, create a token that's a copy of
    // target artifact you control" — install-on-level-up does not support
    // triggered-ability installation; deferred.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
