//! Mishra's Toy Workshop — nonbasic land (Unfinity, 2022).
//! "{T}: Add {C}{C}{C}. Spend this mana only on spells and abilities
//! that put tokens onto the battlefield. Use toys to represent the
//! tokens."
//!
//! The spend restriction is not expressible — the plain three-mana
//! ability is emitted with the restriction GAP'd, per the LAND RULES.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mishra's Toy Workshop");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}{C}{C}. Spend this mana only on spells \
                       and abilities that put tokens onto the battlefield."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_three_colorless,
            },
        ),
    )
}

fn add_three_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Spend this mana only on spells and abilities that put tokens
    // onto the battlefield" — mana spend restrictions are not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 3],
    }]
}
