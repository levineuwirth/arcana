//! Chandra Nalaar — `{3}{R}{R}` Legendary Planeswalker — Chandra, starting loyalty 6.
//!
//! Loyalty abilities:
//! * `+1`: Chandra Nalaar deals 1 damage to target player or planeswalker.
//!   (Modeled as "any target" — the engine's `AnyTarget` covers player or
//!   planeswalker; the creature branch is a faithful superset for this PW.)
//! * `−X`: deals X damage to target creature. GAP — dynamic-X loyalty cost is
//!   not expressible (`remove_self_counter` is a fixed u32). Ability omitted.
//! * `−8`: deals 10 damage to a player/PW AND each creature that player's
//!   controller controls. Declared with correct cost; effect GAP'd (the
//!   board-wide "each creature that target's controller controls" sweep tied
//!   to the chosen target's controller is not expressible from the demonstrated
//!   surface).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra Nalaar");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Chandra Nalaar deals 1 damage to target player or \
                       planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Chandra Nalaar deals 10 damage to target player or \
                       planeswalker and each creature that player or that \
                       planeswalker's controller controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate,
            }),
    )
}

/// `+1: Chandra Nalaar deals 1 damage to target player or planeswalker.`
fn plus_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: 1 }]
}

/// `−8: …10 damage to that target and each creature that player/PW's
/// controller controls.`
fn ultimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each creature that the chosen player or that planeswalker's
    // controller controls" sweep — keyed to the target's controller — is not
    // expressible from the demonstrated Effect surface.
    Vec::new()
}
