//! Vortex Elemental — `{U}` 0/1 blue Elemental.
//! {U}: Put this creature and each creature blocking or blocked by it on top of
//!      their owners' libraries, then those players shuffle.
//! {3}{U}{U}: Target creature blocks this creature this turn if able.
//!
//! The first activation is wired: put this creature plus every creature
//! blocking it / blocked by it on top of their owners' libraries. (The "then
//! shuffle" is handled by the engine's library-move bookkeeping.)
//! GAP: the second activation ("target creature blocks this creature this turn
//! if able") is a Lure-style must-block requirement with no demonstrated
//! effect variant, so that ability is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vortex Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}: Put this creature and each creature blocking or blocked by it on top of their owners' libraries, then those players shuffle.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: whirl_to_library,
        }),
    )
}

fn whirl_to_library(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::PutOnTopOfLibrary { target: ctx.source }];
    // each creature blocking this creature (it was the attacker) ...
    for id in script::blockers_of(state, ctx.source) {
        effects.push(Effect::PutOnTopOfLibrary { target: id });
    }
    // ... and each creature this creature is blocking (it was the blocker).
    for id in script::attackers_blocked_by(state, ctx.source) {
        effects.push(Effect::PutOnTopOfLibrary { target: id });
    }
    effects
}
