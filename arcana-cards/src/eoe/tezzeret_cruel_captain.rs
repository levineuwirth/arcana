//! Tezzeret, Cruel Captain — `{3}` Legendary Planeswalker — Tezzeret, starting loyalty 3.
//! Static: whenever an artifact you control enters, put a loyalty counter on Tezzeret — implemented.
//! 0: Untap target artifact or creature; if artifact creature, put a +1/+1 counter — implemented (target permanent; conditional checked at resolution).
//! -3: Search library for an artifact MV<=1, reveal, to hand, shuffle — implemented (with_max_cmc(1)).
//! -7: emblem (begin combat: three +1/+1 on target artifact you control; if not a creature becomes 0/0 Robot) — emblem trigger implemented; becomes-creature part GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Cruel Captain");
    let sub = reg.interner_mut().intern("Tezzeret");
    let _emblem = reg.interner_mut().intern("Tezzeret, Cruel Captain emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

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
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: artifact_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Untap target artifact or creature. If it's an artifact creature, \
                       put a +1/+1 counter on it."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
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
                text: "-3: Search your library for an artifact card with mana value 1 or less, \
                       reveal it, put it into your hand, then shuffle."
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
                text: "-7: You get an emblem with \"At the beginning of combat on your turn, \
                       put three +1/+1 counters on target artifact you control. If it's not a \
                       creature, it becomes a 0/0 Robot artifact creature.\""
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

/// Static triggered: artifact you control enters → put a loyalty counter on Tezzeret.
fn artifact_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

/// `0`: untap target permanent; if it's an artifact creature, put a +1/+1 counter on it.
fn zero_untap(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Untap { target: *id }];
    if let Some(obj) = state.object_or_lki(*id) {
        let types = obj.characteristics.types;
        if types.has(TypeLine::ARTIFACT) && types.has(TypeLine::CREATURE) {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}

/// `-3`: tutor an artifact with mana value 1 or less to hand.
fn minus_three_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .with_max_cmc(1),
        reveal: true,
    }]
}

/// `-7`: triggered emblem — begin combat on your turn.
fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Tezzeret, Cruel Captain emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_begin_combat,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
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

/// Emblem trigger: put three +1/+1 counters on target artifact you control.
fn emblem_begin_combat(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "if it's not a creature, it becomes a 0/0 Robot artifact creature" — no
    // becomes-creature continuous effect available; only the +1/+1 counters are applied.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}
