//! Urza's Tower — nonbasic land, subtypes Urza's Tower (Antiquities,
//! 1994). "{T}: Add {C}. If you control an Urza's Mine and an Urza's
//! Power-Plant, add {C}{C}{C} instead." One mana ability whose output
//! is computed at resolution from the board (Tron check).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Tower");
    let urzas = reg.interner_mut().intern("Urza's");
    let tower = reg.interner_mut().intern("Tower");
    // Pre-intern the sibling Tron subtypes for resolve-time lookup.
    let _mine = reg.interner_mut().intern("Mine");
    let _power_plant = reg.interner_mut().intern("Power-Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(urzas);
    subtypes.0.insert(tower);
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
                text: "{T}: Add {C}. If you control an Urza's Mine and an \
                       Urza's Power-Plant, add {C}{C}{C} instead."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_tron_mana,
            },
        ),
    )
}

fn add_tron_mana(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // The Mine / Power-Plant subtypes only appear on the Urza lands,
    // so a single-subtype check is faithful.
    let has_mine = script::count_matching(
        state,
        &script::subtype_filter(reg, "Mine"),
        ctx.controller,
    ) >= 1;
    let has_power_plant = script::count_matching(
        state,
        &script::subtype_filter(reg, "Power-Plant"),
        ctx.controller,
    ) >= 1;
    let count = if has_mine && has_power_plant { 3 } else { 1 };
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); count],
    }]
}
