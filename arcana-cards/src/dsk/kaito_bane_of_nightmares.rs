//! Kaito, Bane of Nightmares — `{2}{U}{B}` Legendary Planeswalker — Kaito, starting loyalty 5.
//!
//! Ninjutsu {1}{U}{B}.
//! During your turn, as long as Kaito has one or more loyalty counters on him,
//! he's a 3/4 Ninja creature and has hexproof.
//! +1: You get an emblem with "Ninjas you control get +1/+1."
//! 0: Surveil 2. Then draw a card for each opponent who lost life this turn.
//! −2: Tap target creature. Put two stun counters on it.
//!
//! # Scope
//! - Ninjutsu and the during-your-turn self-animation static are not part of the
//!   demonstrated loyalty surface — GAP'd (no `keywords` slot for Ninjutsu;
//!   conditional self-animation is a continuous effect, not a loyalty ability).
//! - `+1`: EMBLEM with "Ninjas you control get +1/+1" — implemented via a
//!   filtered_pump anthem on the Ninja subtype.
//! - `0`: Surveil 2 is emitted; the "draw a card for each opponent who lost life
//!   this turn" rider is dynamic and GAP'd (the Surveil portion still resolves).
//! - `−2`: Tap target creature + put two stun counters on it — implemented.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaito, Bane of Nightmares");
    let kaito = reg.interner_mut().intern("Kaito");
    let _ninja = reg.interner_mut().intern("Ninja");
    let _emblem = reg.interner_mut().intern("Kaito, Bane of Nightmares emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaito);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You get an emblem with \"Ninjas you control get \
                       +1/+1.\""
                    .into(),
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
                effect: plus_one_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Surveil 2. Then draw a card for each opponent who lost \
                       life this turn."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_surveil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Tap target creature. Put two stun counters on it.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_tap_stun,
            }),
    )
}

/// `+1: You get an emblem with "Ninjas you control get +1/+1."`
fn plus_one_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Kaito, Bane of Nightmares emblem")
        .expect("emblem name interned");
    let ninja = reg.interner().lookup("Ninja").expect("Ninja interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![arcana_core::layers::ContinuousEffect::filtered_pump(
                arcana_core::objects::NULL_OBJECT_ID,
                ObjectFilter {
                    subtypes: Some(vec![ninja]),
                    ..Default::default()
                },
                1,
                1,
                Duration::Permanent,
            )],
            abilities: Vec::new(),
        },
    }]
}

/// `0: Surveil 2. Then draw a card for each opponent who lost life this turn.`
fn zero_surveil(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // The "draw a card for each opponent who lost life this turn" rider is a
    // dynamic, history-dependent count — GAP'd. Surveil 2 still resolves.
    vec![Effect::Surveil {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−2: Tap target creature. Put two stun counters on it.`
fn minus_two_tap_stun(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 2,
        },
    ]
}
