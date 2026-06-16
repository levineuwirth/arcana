//! Chandra, Pyrogenius — `{4}{R}{R}` planeswalker, starting loyalty 5.
//! Legendary Planeswalker — Chandra.
//!
//! # Rules references
//!
//! * CR 113.3c — entering the battlefield with loyalty counters equal
//!   to printed loyalty.
//! * CR 606 — loyalty abilities (activated abilities whose cost is
//!   adding or removing loyalty counters from the source permanent).
//! * CR 606.3 — controller-only, sorcery-speed, stack empty, once per
//!   turn per planeswalker (engine-enforced).
//! * CR 704.5i — a 0-loyalty planeswalker is sacrificed as an SBA.
//!
//! # Scope
//!
//! * `+2` ("Chandra deals 2 damage to each opponent") — no
//!   "damage to each opponent" primitive (DamageTarget is a single
//!   Player/Object and no opponent-enumeration helper is demonstrated);
//!   shell declared with the correct `+2` cost, effect GAP'd.
//! * `−3` ("Chandra deals 4 damage to target creature") — modeled.
//! * `−10` ("…6 damage to target player or planeswalker and each
//!   creature that player or that planeswalker's controller controls")
//!   — bespoke target-plus-board-wide effect with no demonstrated
//!   primitive for the "and each creature that player controls" rider;
//!   shell declared with the correct `−10` cost, effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Pyrogenius");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Chandra, Pyrogenius deals 2 damage to each \
                       opponent."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_each_opponent,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Chandra, Pyrogenius deals 4 damage to target \
                       creature."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_creature,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Chandra, Pyrogenius deals 6 damage to target \
                       player or planeswalker and each creature that \
                       player or that planeswalker's controller controls."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_ultimate,
            }),
    )
}

/// `+2: Chandra, Pyrogenius deals 2 damage to each opponent.`
fn plus_two_each_opponent(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no "deal damage to each opponent" primitive in the demonstrated
    // Effect surface — DamageTarget is a single Player/Object and no
    // opponent-enumeration helper is shown.
    Vec::new()
}

/// `−3: Chandra, Pyrogenius deals 4 damage to target creature.`
fn minus_three_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}

/// `−10: Chandra, Pyrogenius deals 6 damage to target player or
/// planeswalker and each creature that player or that planeswalker's
/// controller controls.`
fn minus_ten_ultimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: bespoke ultimate — damage to a chosen player/planeswalker plus
    // board-wide damage to that controller's creatures; the "and each
    // creature that player controls" rider has no demonstrated primitive.
    Vec::new()
}
