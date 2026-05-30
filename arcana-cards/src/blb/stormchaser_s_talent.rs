//! Stormchaser's Talent — `{U}` blue Enchantment — Class.
//!
//! Level 1: When this Class enters, create a 1/1 blue and red Otter creature token with prowess.
//!   GAP: Prowess is not in the engine keyword surface; token is created without prowess.
//!
//! Level 2 ({3}{U}): When this Class becomes level 2, return target instant or sorcery card
//!   from your graveyard to your hand.
//!
//! Level 3 ({5}{U}): Whenever you cast an instant or sorcery spell, create a 1/1 blue and red
//!   Otter creature token with prowess.
//!   GAP: Prowess not in keyword surface; token created without prowess.
//!   GAP: Level 3 triggered ability fires from level 1 (no level-gate on TriggeredAbilityDef).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormchaser's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    // Pre-intern Otter subtype for token creation at resolve time.
    let _ = reg.interner_mut().intern("Otter");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
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
            // Level 1: when this Class enters, create a 1/1 blue and red Otter token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_otter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2 activation: {3}{U}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
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
            // Level 3 activation: {5}{U}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{U}: Level 3.".into(),
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
            })
            // Level 2 "when this Class becomes level 2, return target instant or sorcery card
            // from your graveyard to your hand."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
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
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Level 3 "whenever you cast an instant or sorcery spell, create a 1/1 blue and
            // red Otter creature token."
            // GAP: fires from level 1 (no level-gate on TriggeredAbilityDef).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_instant_sorcery_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_otter_token(controller: PlayerId, reg: &CardRegistry) -> Effect {
    let otter_sub = reg.interner().lookup("Otter").expect("Otter interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(otter_sub);
    Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: otter_sub,
            colors: ColorSet::blue() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            // GAP: Prowess not in engine keyword surface; omitted.
            keywords: vec![],
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            abilities: vec![],
        },
    }
}

/// Level 1 ETB: create a 1/1 blue and red Otter creature token.
fn etb_create_otter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![make_otter_token(trig.controller, reg)]
}

/// Level 2 activation: bump Level counter.
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

/// Level 3 activation: bump Level counter.
fn level_up_to_3(
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

/// "When this Class becomes level 2, return target instant or sorcery card from your graveyard to your hand."
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
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

/// Level 3: "Whenever you cast an instant or sorcery spell, create a 1/1 blue and red Otter token."
fn on_instant_sorcery_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![make_otter_token(trig.controller, reg)]
}
