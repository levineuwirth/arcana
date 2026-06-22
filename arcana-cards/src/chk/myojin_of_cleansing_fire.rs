//! Myojin of Cleansing Fire — `{5}{W}{W}{W}` 4/6 Legendary Spirit.
//!
//! Oracle:
//! * Myojin enters with a divinity counter on it if you cast it from your
//!   hand.
//! * Myojin has indestructible as long as it has a divinity counter on it.
//! * Remove a divinity counter from Myojin: Destroy all other creatures.
//!
//! The activated "remove a divinity counter: destroy all other creatures"
//! is wired: the cost removes one `Named("divinity")` counter and the
//! effect destroys every creature except this source.
//!
//! GAP: "enters with a divinity counter on it if you cast it from your
//! hand" — the conditional enters-with-counter face spec is not part of
//! the documented effect/keyword surface, so the ETB counter is omitted.
//! GAP (static): "has indestructible as long as it has a divinity counter
//! on it" — a counter-gated static keyword grant is not expressible with
//! the documented triggered/activated surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Cleansing Fire");
    let spirit = reg.interner_mut().intern("Spirit");
    let divinity = reg.interner_mut().intern("divinity");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove a divinity counter from Myojin of Cleansing Fire: \
                   Destroy all other creatures."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Named(divinity), 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_all_other_creatures,
        }),
    )
}

/// Destroy every creature on the battlefield other than this source.
fn destroy_all_other_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Any),
        ctx.controller,
    )
    .into_iter()
    .filter(|id| *id != ctx.source)
    .map(|id| Effect::DestroyPermanent { target: id })
    .collect()
}
