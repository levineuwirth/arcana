//! Waterveil Cavern — nonbasic land (Tempest karoo-pain cycle).
//! "{T}: Add {C}." and "{T}: Add {U} or {B}. This land doesn't untap
//! during your next untap step." The colored half is two mana
//! abilities (one per color).
//!
//! GAP: "This land doesn't untap during your next untap step" — there
//! is no skip-untap effect; the rider is unmodeled and the colored
//! abilities resolve as plain mana abilities.

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
    let name = reg.interner_mut().intern("Waterveil Cavern");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
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
                text: "{T}: Add {U}. This land doesn't untap during your \
                       next untap step."
                    .into(),
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
                text: "{T}: Add {B}. This land doesn't untap during your \
                       next untap step."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black_mana,
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

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "This land doesn't untap during your next untap step" —
    // skip-untap rider not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_black_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "This land doesn't untap during your next untap step" —
    // skip-untap rider not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}
