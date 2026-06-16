//! Dack Fayden — `{1}{U}{R}` Legendary Planeswalker — Dack, starting loyalty 3.
//! Colors R/U.
//!
//! +1: Target player draws two cards, then discards two cards.
//! −2: Gain control of target artifact (`Effect::ChangeControl` to you;
//!   permanent control change).
//! −6: emblem ("Whenever you cast a spell that targets one or more permanents,
//!   gain control of those permanents."). Triggered emblem shell with the cast
//!   trigger; effect GAP'd — there is no demonstrated surface to read a spell's
//!   chosen permanent targets and gain control of them.

use arcana_core::effects::{Effect, EmblemDefinition, DiscardChoice};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dack Fayden");
    let dack = reg.interner_mut().intern("Dack");
    let _emblem = reg.interner_mut().intern("Dack Fayden emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dack);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target player draws two cards, then discards two \
                       cards.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Gain control of target artifact.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_steal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Whenever you cast a spell \
                       that targets one or more permanents, gain control of \
                       those permanents.\"".into(),
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

fn plus_one_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::DrawCards { player: *p, count: 2 },
        Effect::Discard { player: *p, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}

fn minus_two_steal(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ChangeControl { target: *id, new_controller: ctx.controller }]
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Dack Fayden emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_gain_control,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_gain_control(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gain control of the permanents that spell targets" — no demonstrated
    //      surface reads a cast spell's chosen permanent targets.
    Vec::new()
}
