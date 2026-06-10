//! Dimir Guildgate — nonbasic land, subtype Gate (Return to Ravnica,
//! 2012). "Dimir Guildgate enters the battlefield tapped." and
//! "{T}: Add {U} or {B}." The seed touchstone for the tapland half
//! of the UtilityLand card-gen shape: enters-tapped plus a two-color
//! mana choice.
//!
//! # Rules references
//!
//! * CR 614.12 — "enters the battlefield tapped" is a replacement
//!   applied as the permanent enters; modeled with
//!   [`EntersWithSpec::Tapped`].
//! * CR 605.1a / CR 106.3 — "Add {U} or {B}" is one printed mana
//!   ability with a color choice. Modeled as TWO separately
//!   activatable mana abilities ({T}: Add {U} / {T}: Add {B}) — the
//!   engine's choice of which to activate IS the color choice, and
//!   each is a faithful mana ability (no stack, no target). This is
//!   the catalog-wide idiom for "Add [one color] or [another]".
//! * The Gate subtype is mechanically relevant ("for each Gate you
//!   control") and is interned like any creature subtype.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dimir Guildgate");
    let gate = reg.interner_mut().intern("Gate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gate);
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
            .with_enters_with(EntersWithSpec::Tapped)
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
                text: "{T}: Add {B}.".into(),
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

fn add_black_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}
