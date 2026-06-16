//! Subversive Acolyte — `{1}{B}` 2/3 black Human.
//! {2}, Pay 2 life: Choose one. Activate only once.
//! • This creature becomes a Human Cleric. It gets +1/+1 and gains lifelink.
//! • This creature becomes a Phyrexian. It gets +3/+2 and gains trample and
//!   "Whenever this creature is dealt damage, sacrifice that many permanents."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Subversive Acolyte");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // The cost (mana + 2 life, activate only once) is expressible; the modal
    // payload is not — activated abilities have no modal-mode dispatch, and the
    // permanent type-change + permanent +1/+1 / +3/+2 + permanent keyword/ability
    // grants are not expressible with the available Effect surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Pay 2 life: Choose one. Activate only once.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    life: 2,
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: choose_one_transform,
            }),
    )
}

fn choose_one_transform(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal activated ability — no mode dispatch on activated abilities, and
    // the permanent type-change + permanent buff + granted-ability payloads are
    // not expressible.
    Vec::new()
}
