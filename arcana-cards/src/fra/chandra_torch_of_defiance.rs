//! Chandra, Torch of Defiance — `{2}{R}{R}` Legendary Planeswalker — Chandra, starting loyalty 4.
//!
//! +1: Exile the top card of your library. You may cast that card. If you
//!     don't, Chandra deals 2 damage to each opponent. (GAP — exile-top-and-
//!     may-cast rider + "each opponent" damage branch not expressible.)
//! +1: Add {R}{R}. (GAP — mana production is not part of the demonstrated
//!     Effect surface for this card class.)
//! −3: Chandra deals 4 damage to target creature.
//! −7: You get an emblem with "Whenever you cast a spell, this emblem deals 5
//!     damage to any target." (Implemented as a triggered emblem.)

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Torch of Defiance");
    let chandra = reg.interner_mut().intern("Chandra");
    let _emblem = reg.interner_mut().intern("Chandra, Torch of Defiance emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile the top card of your library. You may cast \
                       that card. If you don't, Chandra deals 2 damage to each \
                       opponent."
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
                effect: plus_one_impulse,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}{R}.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Chandra deals 4 damage to target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_burn,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Whenever you cast a spell, \
                       this emblem deals 5 damage to any target.\""
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

/// `+1: Exile top, may cast; if not, 2 to each opponent.`
fn plus_one_impulse(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: exile-top + may-cast rider + "if you don't, damage each opponent" branch.
    Vec::new()
}

/// `+1: Add {R}{R}.`
fn plus_one_mana(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: mana production not in the demonstrated Effect surface.
    Vec::new()
}

/// `−3: Chandra deals 4 damage to target creature.`
fn minus_three_burn(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}

/// `−7: emblem — Whenever you cast a spell, deal 5 to any target.`
fn minus_seven_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Chandra, Torch of Defiance emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_burn,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }],
        },
    }]
}

/// Emblem: deal 5 damage to any target on each spell you cast.
fn emblem_burn(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let dt = match trig.targets.targets.first() {
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::ObjectOrPlayer(oop)) => match oop {
            arcana_core::targets::ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            arcana_core::targets::ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 5,
    }]
}
