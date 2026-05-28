//! Gigantoplasm — `{3}{U}` 0/0 blue Shapeshifter. "You may have this creature
//! enter as a copy of any creature on the battlefield, except it has '{X}:
//! This creature has base power and toughness X/X.'"
//!
//! GAP: "enter as a copy ... except it has [ability]" — copy-on-entry with
//! ability injection not expressible. The {X}: set base PT activation is also
//! not directly expressible (X-cost activation with SetBasePT). Emitting a
//! basic creature shell with a GAP placeholder.

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
    let name = reg.interner_mut().intern("Gigantoplasm");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}: This creature has base power and toughness X/X.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: set_base_pt_x,
            }),
    )
}

fn set_base_pt_x(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X-cost activation not expressible; ctx.x_value carries the X.
    let x = ctx.x_value.unwrap_or(0) as i32;
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: x,
        toughness: x,
        duration: arcana_core::layers::Duration::EndOfTurn,
    }]
}
