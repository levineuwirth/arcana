//! Kaya, Ghost Haunter — `{2}{W}{B}` Legendary Planeswalker — Kaya, starting loyalty 5.
//!
//! 0: Exile Kaya, Ghost Haunter haunting target creature for as long as
//!   that creature remains on the battlefield. GAP: the "haunt" exile
//!   mechanic (CR 702.55) isn't modeled in the demonstrated surface.
//! −1: You get an emblem with "At the beginning of your upkeep, this
//!   emblem deals 3 damage to the owner of target haunted creature."
//!   The emblem is created with an upkeep trigger; the effect references
//!   a "haunted creature" (haunt isn't modeled), so the effect is GAP'd.
//! −2: You get an emblem with "At the beginning of your upkeep, gain
//!   control of target haunted creature for as long as it remains
//!   haunted." Created with an upkeep trigger; the haunted-creature
//!   targeting and conditional control aren't expressible — GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter, TargetRequirement};
use arcana_core::turn::Step;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Ghost Haunter");
    let kaya = reg.interner_mut().intern("Kaya");
    let _emblem_a = reg.interner_mut().intern("Kaya, Ghost Haunter emblem (damage)");
    let _emblem_b = reg.interner_mut().intern("Kaya, Ghost Haunter emblem (control)");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Exile Kaya, Ghost Haunter haunting target creature \
                       for as long as that creature remains on the \
                       battlefield.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_haunt,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: You get an emblem with, \"At the beginning of your \
                       upkeep, this emblem deals 3 damage to the owner of \
                       target haunted creature.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: You get an emblem with, \"At the beginning of your \
                       upkeep, gain control of target haunted creature for \
                       as long as it remains haunted.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_emblem,
            }),
    )
}

fn zero_haunt(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the haunt mechanic (exile this haunting a creature) is not
    // modeled in the demonstrated surface.
    Vec::new()
}

fn minus_one_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Kaya, Ghost Haunter emblem (damage)")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_noop,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                // "target haunted creature" — haunt isn't modeled, but the
                // shell uses a creature target requirement.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
            }],
        },
    }]
}

fn minus_two_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Kaya, Ghost Haunter emblem (control)")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_noop,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
            }],
        },
    }]
}

fn emblem_noop(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: both emblems reference a "haunted creature" — the haunt
    // mechanic isn't modeled, so the effect can't be expressed.
    Vec::new()
}
