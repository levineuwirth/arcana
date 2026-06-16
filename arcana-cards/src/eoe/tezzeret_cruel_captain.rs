//! Tezzeret, Cruel Captain — `{3}` Legendary Planeswalker — Tezzeret,
//! starting loyalty (printed) 3.
//!
//! Whenever an artifact you control enters, put a loyalty counter on Tezzeret.
//! 0: Untap target artifact or creature. If it's an artifact creature, put a
//!    +1/+1 counter on it.
//! −3: Search your library for an artifact card with mana value 1 or less,
//!     reveal it, put it into your hand, then shuffle.
//! −7: You get an emblem with "At the beginning of combat on your turn, put
//!     three +1/+1 counters on target artifact you control. If it's not a
//!     creature, it becomes a 0/0 Robot artifact creature."
//!
//! # Scope
//! - The static-trigger ("an artifact you control enters → loyalty counter")
//!   is a ZoneChange(to Battlefield) of an artifact you control → AddCounters
//!   Loyalty on the source.
//! - The `0` untaps a target artifact OR creature; at resolution it checks the
//!   target's types and, if it's an artifact creature, adds a +1/+1 counter.
//! - The `−3` tutors an artifact card with mana value ≤ 1 to hand (reveal).
//! - The `−7` TRIGGERED EMBLEM (now supported): each combat on your turn, put
//!   three +1/+1 counters on a target artifact you control and, if it isn't a
//!   creature, animate it to a 0/0 (still an artifact). The Robot subtype isn't
//!   addable via the demonstrated surface (no AddSubtype) — counters, the 0/0
//!   set, and the creature type ARE applied.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Cruel Captain");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let _emblem = reg.interner_mut().intern("Tezzeret, Cruel Captain emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: artifact_enters_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Untap target artifact or creature. If it's an artifact \
                       creature, put a +1/+1 counter on it."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Search your library for an artifact card with mana \
                       value 1 or less, reveal it, put it into your hand, then \
                       shuffle."
                    .into(),
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
                effect: minus_three_tutor,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"At the beginning of combat on \
                       your turn, put three +1/+1 counters on target artifact you \
                       control. If it's not a creature, it becomes a 0/0 Robot \
                       artifact creature.\""
                    .into(),
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

/// Static trigger: an artifact you control enters → loyalty counter on source.
fn artifact_enters_loyalty(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

/// `0: Untap target artifact or creature; if artifact creature, +1/+1 counter.`
fn zero_untap(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    let mut effects = vec![Effect::Untap { target: id }];
    if let Some(obj) = state.objects.get(id) {
        let t = obj.characteristics.types;
        if t.is_artifact() && t.is_creature() {
            effects.push(Effect::AddCounters {
                target: id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}

/// `−3: Tutor an artifact card with mana value ≤ 1 to hand (reveal).`
fn minus_three_tutor(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .with_max_cmc(1),
        reveal: true,
    }]
}

/// `−7: Triggered emblem — each combat, +3 counters on a target artifact.`
fn minus_seven_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Tezzeret, Cruel Captain emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_combat_trigger,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }],
        },
    }]
}

/// Emblem trigger: +3 counters on the target artifact; if not a creature,
/// becomes a 0/0 artifact creature (Robot subtype GAP'd).
fn emblem_combat_trigger(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    let mut effects = vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }];
    if let Some(obj) = state.objects.get(id) {
        if !obj.characteristics.types.is_creature() {
            effects.push(Effect::AddType {
                target: id,
                types: TypeLine::CREATURE.into(),
                duration: Duration::Permanent,
            });
            effects.push(Effect::SetBasePT {
                target: id,
                power: 0,
                toughness: 0,
                duration: Duration::Permanent,
            });
        }
    }
    effects
}
