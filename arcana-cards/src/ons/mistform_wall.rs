//! Mistform Wall — `{2}{U}` 1/4 Creature — Illusion Wall.
//!
//! Oracle:
//! * This creature has defender as long as it's a Wall — a STATIC
//!   characteristic-defining/keyword-gaining ability ("as long as"); not a
//!   triggered/activated ability and not expressible as a conditional static
//!   here. GAP'd.
//! * {1}: This creature becomes the creature type of your choice until end of
//!   turn. — an activated ability whose effect is "choose a creature type, then
//!   set this creature's subtypes to it". There is no effect to set an
//!   arbitrary player-chosen creature type, so the effect body is GAP'd while
//!   the activated-ability shape (cost {1}) is still recorded.

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
    let name = reg.interner_mut().intern("Mistform Wall");
    let illusion = reg.interner_mut().intern("Illusion");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "This creature has defender as long as it's a Wall"
    // (conditional keyword-gaining static).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: This creature becomes the creature type of your choice until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_chosen_type,
            }),
    )
}

fn become_chosen_type(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "becomes the creature type of your choice until end of turn"
    // (no effect to set a player-chosen arbitrary creature subtype).
    Vec::new()
}
