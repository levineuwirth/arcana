//! Kaya, Ghost Haunter — `{2}{W}{B}` legendary planeswalker, starting loyalty 5.
//!
//! 0: Exile Kaya, Ghost Haunter haunting target creature (haunt GAP).
//! −1: You get an emblem (GAP).
//! −2: You get an emblem (GAP).
//!
//! Scope: all three abilities are GAP'd. The 0 ability is the bespoke
//! "exile haunting" mechanic with no demonstrated Effect; the −1 and −2
//! abilities create emblems with bespoke upkeep triggers. The ability
//! shells (with correct 0 / −1 / −2 costs) are still declared.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Ghost Haunter");
    let kaya = reg.interner_mut().intern("Kaya");
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
                text: "0: Exile Kaya, Ghost Haunter haunting target creature for \
                       as long as that creature remains on the battlefield.".into(),
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
                text: "−1: You get an emblem with, \"At the beginning of your \
                       upkeep, this emblem deals 3 damage to the owner of target \
                       haunted creature.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You get an emblem with, \"At the beginning of your \
                       upkeep, gain control of target haunted creature for as \
                       long as it remains haunted.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_emblem,
            }),
    )
}

/// `0` — exile-haunting mechanic.
fn zero_haunt(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile haunting target creature" haunt mechanic.
    Vec::new()
}

/// `−1` — emblem.
fn minus_one_emblem(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem with haunted-creature upkeep damage trigger.
    Vec::new()
}

/// `−2` — emblem.
fn minus_two_emblem(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem with haunted-creature control-gain trigger.
    Vec::new()
}
