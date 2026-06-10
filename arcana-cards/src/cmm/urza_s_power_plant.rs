//! Urza's Power Plant — nonbasic land, subtypes Urza's Power-Plant
//! (Antiquities, 1994). "{T}: Add {C}. If you control an Urza's Mine
//! and an Urza's Tower, add {C}{C} instead." The conditional amount is
//! computed at resolution from the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Power Plant");
    let urzas = reg.interner_mut().intern("Urza's");
    let power_plant = reg.interner_mut().intern("Power-Plant");
    // Pre-intern the sibling subtypes the resolver checks for.
    let _mine = reg.interner_mut().intern("Mine");
    let _tower = reg.interner_mut().intern("Tower");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(urzas);
    subtypes.0.insert(power_plant);
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
                       Urza's Tower, add {C}{C} instead."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_urzatron_mana,
            },
        ),
    )
}

fn add_urzatron_mana(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mine = script::subtype_filter(reg, "Mine")
        .controlled_by(ControllerConstraint::You);
    let tower = script::subtype_filter(reg, "Tower")
        .controlled_by(ControllerConstraint::You);
    let n: usize = if script::count_matching(state, &mine, ctx.controller) > 0
        && script::count_matching(state, &tower, ctx.controller) > 0
    {
        2
    } else {
        1
    };
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); n],
    }]
}
