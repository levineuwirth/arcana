//! Kiora, Master of the Depths — `{2}{G}{U}` Legendary Planeswalker — Kiora, starting loyalty 4.
//!
//! +1: Untap up to one target creature and up to one target land.
//! −2: Reveal the top four cards of your library. You may put a creature card
//!     and/or a land card from among them into your hand. Put the rest into your
//!     graveyard.
//! −8: You get an emblem with "Whenever a creature you control enters, you may
//!     have it fight target creature." Then create three 8/8 blue Octopus
//!     creature tokens.
//!
//! # Scope
//! - `+1`: untaps up to one target creature and up to one target land —
//!   implemented (two `up to one` target clauses, `Untap` on each chosen).
//! - `−2`: reveal-top-four / put-a-creature-and/or-land-to-hand / rest to
//!   graveyard is a bespoke reveal-and-bin selection not expressible with the
//!   demonstrated Effect surface — ability shell with correct `−2` cost, GAP'd
//!   body.
//! - `−8`: EMBLEM with a triggered ability ("Whenever a creature you control
//!   enters, you may have it fight target creature") — the emblem and its
//!   trigger SHELL are created (ZoneChange creature-enters condition); the
//!   "have it fight target creature" effect body is GAP'd. The three 8/8 blue
//!   Octopus tokens are part of the −8 RESOLUTION and ARE created.

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiora, Master of the Depths");
    let kiora = reg.interner_mut().intern("Kiora");
    let _octopus = reg.interner_mut().intern("Octopus");
    let _emblem = reg.interner_mut().intern("Kiora, Master of the Depths emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kiora);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap up to one target creature and up to one target \
                       land."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Reveal the top four cards of your library. You may put \
                       a creature card and/or a land card from among them into \
                       your hand. Put the rest into your graveyard."
                    .into(),
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
                effect: minus_two_reveal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever a creature you \
                       control enters, you may have it fight target creature.\" \
                       Then create three 8/8 blue Octopus creature tokens."
                    .into(),
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

/// `+1: Untap up to one target creature and up to one target land.`
fn plus_one_untap(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for choice in &ctx.targets.targets {
        if let TargetChoice::Object(id) = choice {
            effects.push(Effect::Untap { target: *id });
        }
    }
    effects
}

/// `−2: Reveal top four; put a creature/land to hand; rest to graveyard.`
fn minus_two_reveal(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reveal-top-N then selectively keep a creature card and/or land card
    // and bin the rest is a bespoke reveal-and-sort selection not expressible
    // with the demonstrated Effect surface.
    Vec::new()
}

/// `−8: emblem + three 8/8 blue Octopus tokens.`
fn minus_eight_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Kiora, Master of the Depths emblem")
        .expect("emblem name interned");
    let octopus = reg.interner().lookup("Octopus").expect("Octopus interned");

    let mut octopus_subtypes = SubtypeSet::default();
    octopus_subtypes.0.insert(octopus);
    let octopus_token = TokenDefinition {
        name: octopus,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: octopus_subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: Vec::new(),
        abilities: Vec::new(),
    };

    vec![
        Effect::CreateEmblem {
            controller: ctx.controller,
            emblem: EmblemDefinition {
                name: emblem_name,
                statics: Vec::new(),
                abilities: vec![TriggeredAbilityDef {
                    id: 1,
                    trigger_condition: TriggerCondition::ZoneChange {
                        filter: ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                        from: None,
                        to: Zone::Battlefield,
                    },
                    intervening_if: None,
                    effect: emblem_fight,
                    trigger_zones: vec![Zone::Command],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: Vec::new(),
                }],
            },
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token: octopus_token.clone(),
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token: octopus_token.clone(),
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token: octopus_token,
        },
    ]
}

/// Emblem: "Whenever a creature you control enters, you may have it fight target
/// creature."
fn emblem_fight(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may have THE ENTERING CREATURE fight target creature" — the
    // fight needs the just-entered object as combatant `a` and a chosen target
    // as `b`; the trigger shell fires but the optional fight body is not
    // expressible without a target-requirement + entering-object accessor here.
    Vec::new()
}
