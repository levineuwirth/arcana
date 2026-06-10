//! Tolaria — legendary land (Legends, 1994).
//! "{T}: Add {U}." and "{T}: Target creature loses banding and all
//! \"bands with other\" abilities until end of turn. Activate only
//! during any upkeep step."
//! The utility activation is wired (tap cost + target creature) but
//! its effect is a GAP: there is no keyword-REMOVAL effect (only
//! GrantKeyword / LoseAllAbilities, which would strip everything), and
//! the "only during any upkeep step" timing window has no
//! ActivationCost / ActivatedAbilityDef expression.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tolaria");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature loses banding and all \"bands \
                       with other\" abilities until end of turn. Activate \
                       only during any upkeep step."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: remove_banding,
            }),
    )
}

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn remove_banding(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses banding and all 'bands with other' abilities" — no
    // keyword-removal effect exists (LoseAllAbilities would strip far
    // more than banding). GAP additionally: "Activate only during any
    // upkeep step" — activation timing windows are not expressible.
    Vec::new()
}
