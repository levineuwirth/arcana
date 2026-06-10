//! Urza's Mine — Land — Urza's Mine (Antiquities).
//! "{T}: Add {C}. If you control an Urza's Power-Plant and an Urza's
//! Tower, add {C}{C} instead." One mana ability whose amount is
//! computed at resolution from the Urzatron pieces you control.

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
    let name = reg.interner_mut().intern("Urza's Mine");
    let urzas = reg.interner_mut().intern("Urza's");
    let mine = reg.interner_mut().intern("Mine");
    let _power_plant = reg.interner_mut().intern("Power-Plant");
    let _tower = reg.interner_mut().intern("Tower");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(urzas);
    subtypes.0.insert(mine);
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
                text: "{T}: Add {C}. If you control an Urza's Power-Plant and an Urza's Tower, add {C}{C} instead.".into(),
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
    let power_plants = script::count_matching(
        state,
        &script::subtype_filter(reg, "Power-Plant")
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let towers = script::count_matching(
        state,
        &script::subtype_filter(reg, "Tower")
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let count = if power_plants >= 1 && towers >= 1 { 2 } else { 1 };
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); count],
    }]
}
