//! Saheeli Rai — `{1}{U}{R}` Legendary Planeswalker — Saheeli, starting
//! loyalty 3.
//!
//! Loyalty abilities:
//! * `+1`: Scry 1. Saheeli Rai deals 1 damage to each opponent. (Scry +
//!   one `DealDamage` per opponent.)
//! * `−2`: Create a token that's a copy of target artifact or creature you
//!   control … gains haste … exile at next end step. GAP — token-copy of a
//!   chosen permanent with the artifact-type overlay + haste + exile rider is
//!   not expressible. Shell declared.
//! * `−7`: Search your library for up to three artifact cards with different
//!   names, put them onto the battlefield, then shuffle. GAP — a multi-card
//!   "up to three with different names" tutor-to-battlefield is not expressible
//!   (`Search` / `TutorToBattlefield` are single-card). Shell declared.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saheeli Rai");
    let saheeli = reg.interner_mut().intern("Saheeli");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saheeli);

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
                text: "+1: Scry 1. Saheeli Rai deals 1 damage to each \
                       opponent.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a token that's a copy of target artifact or \
                       creature you control, except it's an artifact in \
                       addition to its other types. That token gains haste. \
                       Exile it at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Search your library for up to three artifact cards \
                       with different names, put them onto the battlefield, \
                       then shuffle.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate,
            }),
    )
}

/// `+1: Scry 1. Saheeli Rai deals 1 damage to each opponent.`
fn plus_one(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::Scry { player: ctx.controller, count: 1 }];
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(opp),
            amount: 1,
        });
    }
    effects
}

/// `−2`: token-copy of a chosen permanent.
fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: copy-token of a chosen artifact/creature with the artifact overlay +
    // haste + end-step exile rider is not expressible.
    Vec::new()
}

/// `−7`: tutor up to three differently-named artifacts to the battlefield.
fn ultimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: multi-card "up to three artifact cards with different names" tutor to
    // battlefield is not expressible (Search/TutorToBattlefield are single).
    Vec::new()
}
