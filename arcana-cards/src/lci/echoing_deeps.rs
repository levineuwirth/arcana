//! Echoing Deeps — land — Cave (The Lost Caverns of Ixalan, 2023).
//! "You may have this land enter tapped as a copy of any land card in
//! a graveyard, except it's a Cave in addition to its other types."
//! and "{T}: Add {C}."
//!
//! The colorless mana ability and the Cave subtype are wired; the
//! optional enter-as-a-copy replacement is GAP'd (no copy-on-entry
//! EntersWithSpec exists, and unconditionally entering tapped would be
//! wrong since the land enters untapped when the copy option is
//! declined).

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
    let name = reg.interner_mut().intern("Echoing Deeps");
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
    // GAP: "You may have this land enter tapped as a copy of any land
    // card in a graveyard, except it's a Cave in addition to its other
    // types." — an optional enter-the-battlefield copy replacement; no
    // EntersWithSpec variant models copy-on-entry, and EntersWithSpec::
    // Tapped alone would wrongly tap the non-copy entry.
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
