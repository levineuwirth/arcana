//! Faceless Haven — Snow Land (Kaldheim).
//! "{T}: Add {C}." and "{S}{S}{S}: This land becomes a 4/3 creature
//! with vigilance and all creature types until end of turn. It's
//! still a land."
//!
//! GAP: the animation ability's cost is three SNOW mana ({S}{S}{S});
//! snow mana symbols are not a demonstrated `ManaCost` form and snow
//! mana payment is not modeled — the whole animation activation is
//! omitted. ("All creature types" / Changeling subtype grant would
//! also be inexpressible.) Only the colorless mana ability is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Faceless Haven");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        ..Default::default()
    };
    // GAP: "{S}{S}{S}: This land becomes a 4/3 creature with vigilance
    // and all creature types until end of turn." — snow mana costs
    // ({S}) are not expressible as an ActivationCost, so the animation
    // ability cannot be wired.
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
