//! Urza's Workshop — land — Urza's.
//! "{T}: Add {C}." and "Metalcraft — {T}: Add {C} for each Urza's land you
//! control. Activate only if you control three or more artifacts." The
//! dynamic mana amount is computed at resolution; the metalcraft activation
//! restriction is approximated as a resolution-time gate.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Workshop");
    let urzas = reg.interner_mut().intern("Urza's");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(urzas);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Metalcraft — {T}: Add {C} for each Urza's land you control. Activate only if you control three or more artifacts.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: metalcraft_add,
            }),
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

fn metalcraft_add(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Activate only if you control three or more artifacts"
    // (metalcraft) is an activation restriction; approximated as a
    // resolution-time gate.
    let artifacts = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    if artifacts < 3 {
        return Vec::new();
    }
    let urzas_lands = script::count_matching(
        state,
        &script::subtype_filter(reg, "Urza's").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Colorless, ctx.source);
            urzas_lands as usize
        ],
    }]
}
