//! Builder's Talent — `{1}{W}` white Enchantment — Class.
//!
//! Level 1: When this Class enters, create a 0/4 white Wall creature token with defender.
//! {W}: Level 2 — Whenever one or more noncreature, nonland permanents you control enter,
//!   put a +1/+1 counter on target creature you control.
//! {4}{W}: Level 3 — When this Class becomes level 3, return target noncreature, nonland
//!   permanent card from your graveyard to the battlefield.
//!
//! GAP: Level-2 trigger fires for any noncreature nonland permanent entering under your
//!   control (no level-gating on triggers — fires regardless of level). Modeled as ZoneChange
//!   trigger.
//! GAP: Level-3 effect fires from level-up activation (not a separate "when becomes level 3"
//!   event). Target is a noncreature nonland permanent in graveyard.
//! GAP: per-level static abilities beyond counter placement are deferred
//!   (continuous-effect engine subsystem).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Builder's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    // Pre-intern token subtype for resolution-time lookup
    let _wall = reg.interner_mut().intern("Wall");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
            // Level 1: ETB — create a 0/4 white Wall creature token with defender
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_wall,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: activated ability (requires level >= 1)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 2 trigger: whenever noncreature, nonland permanents you control enter,
            // put a +1/+1 counter on target creature you control.
            // GAP: no level-gating on triggers — fires at any level.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .without_types(TypeLine::CREATURE.into())
                        .without_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: level2_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Level 3: activated ability (requires level >= 2); returns target noncreature nonland
            // permanent from graveyard.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .without_types(TypeLine::CREATURE.into())
                            .without_types(TypeLine::LAND.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            })
    )
}

fn etb_create_wall(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wall = reg.interner().lookup("Wall").expect("Wall interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(wall);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: wall,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Defender],
            abilities: vec![],
        },
    }]
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
}

fn level2_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }];
    if let Some(target) = ctx.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    effects
}
