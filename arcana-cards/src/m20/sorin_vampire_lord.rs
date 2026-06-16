//! Sorin, Vampire Lord — `{4}{B}{B}` planeswalker, starting loyalty 5.
//! Legendary Planeswalker — Sorin.
//!
//! Oracle text:
//! * `+1`: Up to one target creature gets +2/+0 until end of turn.
//! * `−2`: Sorin deals 4 damage to any target. You gain 4 life.
//! * `−8`: Until end of turn, each Vampire you control gains
//!   "{T}: Gain control of target creature."
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 704.5i — a planeswalker with 0 loyalty is sacrificed (SBA).
//!
//! # Scope
//!
//! The `+1` (up-to-one-target pump) and `−2` (deal 4 to any target,
//! gain 4 life) are fully modeled. The `−8` grants an activated control
//! ability to a creature subset until end of turn — granting an
//! arbitrary activated ability is not in the demonstrated surface, so
//! its loyalty shell is declared with a GAP'd effect body.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Vampire Lord");
    let sorin = reg.interner_mut().intern("Sorin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets +2/+0 until end \
                       of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Sorin deals 4 damage to any target. You gain 4 \
                       life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_burn,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Until end of turn, each Vampire you control gains \
                       \"{T}: Gain control of target creature.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate_grant,
            }),
    )
}

/// `+1: Up to one target creature gets +2/+0 until end of turn.`
fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

/// `−2: Sorin deals 4 damage to any target. You gain 4 life.`
fn minus_two_burn(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage {
            source: ctx.source,
            target: dt,
            amount: 4,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 4,
        },
    ]
}

/// `−8: Until end of turn, each Vampire you control gains "{T}: Gain
/// control of target creature."`
fn ultimate_grant(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granting an arbitrary activated ability (control-theft) to a
    // creature subset is not in the demonstrated Effect surface.
    Vec::new()
}
