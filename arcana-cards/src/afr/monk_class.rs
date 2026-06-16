//! Monk Class — `{W}{U}` blue/white Enchantment — Class.
//!
//! Level 1 (base): The second spell you cast each turn costs {1} less to cast.
//! Level 2 ({W}{U}): When this Class becomes level 2, return up to one target nonland
//!   permanent to its owner's hand.
//! Level 3 ({1}{W}{U}): At the beginning of your upkeep, exile the top card of your library.
//!   For as long as it remains exiled, it has "You may cast this card from exile as long as
//!   you've cast another spell this turn."
//!
//! # GAPs
//! - Level 1 "second spell each turn costs {1} less": cost-reduction continuous effect not
//!   expressible via anthem/keyword_anthem builders; GAP (no matching ContinuousEffect builder).
//! - Level 3 upkeep trigger with "exile top card, may cast from exile if you've cast another
//!   spell": per-source exile tracking + conditional cast-from-exile not in engine; GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monk Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1: "second spell each turn costs {1} less" — cost reduction is a GAP.
            // No triggered ability for level 1 (continuous effect engine debt).
            // Level 2 activation: {W}{U}, requires level 1.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}").unwrap(),
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
            // Level 3 activation: {1}{W}{U}, requires level 2.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}{U}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}{U}").unwrap(),
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
            })
            // "When this Class becomes level 2, return up to one target nonland permanent
            //  to its owner's hand."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Level),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: on_become_level_2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
    // The "becomes level 2" trigger fires separately and handles the bounce.
    // GAP: "second spell each turn costs {1} less" — cost reduction not modeled
    // (continuous-effect engine subsystem).
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 3 "at beginning of your upkeep, exile top card; may cast from exile
    // if you've cast another spell" — per-source exile tracking + conditional cast
    // from exile not expressible in engine.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn on_become_level_2(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}
