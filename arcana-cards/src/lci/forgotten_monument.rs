//! Forgotten Monument — nonbasic land, subtype Cave (Lost Caverns of
//! Ixalan, 2023). "{T}: Add {C}." and "Other Caves you control have
//! \"{T}, Pay 1 life: Add one mana of any color.\""
//!
//! GAP: granting an activated ability to OTHER permanents you control
//! ("Other Caves you control have ...") is not expressible — only the
//! colorless mana ability is wired.

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
    let name = reg.interner_mut().intern("Forgotten Monument");
    let cave = reg.interner_mut().intern("Cave");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cave);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "Other Caves you control have '{T}, Pay 1 life: Add one mana
    // of any color.'" — granting activated abilities to other permanents
    // is not expressible with the demonstrated API.
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
