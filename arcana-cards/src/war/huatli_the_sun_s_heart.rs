//! Huatli, the Sun's Heart — `{2}{G/W}` Legendary Planeswalker — Huatli, loyalty 7.
//!
//! Static: Each creature you control assigns combat damage equal to its
//!   toughness rather than its power.
//! −3: You gain life equal to the greatest toughness among creatures you control.
//!
//! # Scope
//! GAP: the "assign combat damage equal to toughness" static (a combat-damage
//!   replacement on all your creatures) is not expressible in the demonstrated
//!   surface; it is not a loyalty ability and carries no loyalty cost, so it is
//!   recorded only in this doc comment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli, the Sun's Heart");
    let huatli = reg.interner_mut().intern("Huatli");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(huatli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "-3: You gain life equal to the greatest toughness among creatures you control.".into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Loyalty, 3)),
                ..ActivationCost::default()
            },
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: true,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: minus_three_gain,
        }),
    )
}

fn minus_three_gain(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let greatest = creatures
        .into_iter()
        .map(|id| script::toughness_of(state, id))
        .max()
        .unwrap_or(0);
    if greatest <= 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: ctx.controller, amount: greatest as u32 }]
}
