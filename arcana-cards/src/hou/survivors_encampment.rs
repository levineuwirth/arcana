//! Survivors' Encampment — nonbasic land, subtype Desert (Hour of
//! Devastation, 2017). "{T}: Add {C}." and "{T}, Tap an untapped
//! creature you control: Add one mana of any color." The second
//! ability's additional cost (tap an untapped creature you control)
//! has no `ActivationCost` field, so that ability is GAP'd rather
//! than wired under-costed.
//!
//! // GAP: "{T}, Tap an untapped creature you control: Add one mana of
//! // any color." — ActivationCost has no tap-another-permanent cost
//! // field; wiring it as tap-only would materially under-cost the
//! // ability, so it is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Survivors' Encampment");
    let desert = reg.interner_mut().intern("Desert");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(desert);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            },
        ),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
