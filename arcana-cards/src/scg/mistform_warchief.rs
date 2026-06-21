//! Mistform Warchief — `{2}{U}` 1/3 Illusion.
//!
//! * Creature spells you cast that share a creature type with this
//!   creature cost {1} less to cast. (static cost reduction — GAP)
//! * `{T}`: This creature becomes the creature type of your choice
//!   until end of turn. (GAP — no subtype-setting effect.)

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
    let name = reg.interner_mut().intern("Mistform Warchief");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    // GAP: "Creature spells you cast that share a creature type with
    // this creature cost {1} less" is a static cost reduction; no
    // expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature becomes the creature type of your choice until end of turn.".into(),
                cost: ActivationCost::tap_only(),
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
    // GAP: "becomes the creature type of your choice until end of turn"
    // requires a chosen-subtype-setting effect; no such Effect variant.
    Vec::new()
}
