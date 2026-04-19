//! Tangled Florahedron // Tangled Vale — ZNR common MDFC. Front face
//! is a 1/1 green Elf Druid for `{G}`; back face is a tapped land
//! that taps for `{G}`. The seed's touchstone for CR 712.4 Modal
//! Double-Faced Card — cast the front as a creature from hand, or
//! play the back as the turn's land drop.
//!
//! # Scope compression
//!
//! Tangled Florahedron's printed rules text includes the ability
//! "{T}, Sacrifice this: Add one mana of any color." This engine's
//! Phase 2 seed ships the front face as a *vanilla* 1/1 Elf Druid —
//! the sac-to-any-color mana ability is deferred. The goal of the
//! seed is to prove MDFC cast/play paths, not to exercise every
//! printed ability. When any-color mana production lands as a
//! general capability (cascading choices, five-color fixing test
//! fixtures), revisit this file.
//!
//! # Rules references
//!
//! * CR 712.4 — Modal double-faced card. Both faces have their own
//!   name, mana cost, type line, and rules text; the player chooses
//!   which face to cast (or play, in the land-back case) at
//!   play/cast time.
//! * CR 712.4e — A player may cast either face of an MDFC from hand.
//!   The engine surfaces this as two legal actions: the front-face
//!   cast (via the normal hand-cast loop) and the back-face "play
//!   land" (via the MDFC-back land-play branch).
//! * CR 305.2 — Playing a land gives control of it to the player
//!   who played it. [`arcana_core::engine::apply_play_land`] sets
//!   the controller explicitly so the back-face land's mana
//!   ability (which reads `ctx.controller`) produces the correct
//!   mana for the right player.
//!
//! # Engine wiring
//!
//! The front face rides on [`CardDefinition::base_characteristics`];
//! the back face lives on [`CardDefinition::alternate_face`] as
//! [`AlternateFace::Mdfc(CardFace)`]. The back face's own
//! `ActivatedAbilityDef` (tap-for-green) sits on `CardFace` too —
//! but since the back face's type is `Land`, the engine reaches
//! this ability via the on-battlefield characteristics after the
//! [`arcana_core::engine::apply_play_land`] swap, not via the
//! registry's main `activated_abilities` list (which belongs to the
//! front face).
//!
//! **Known limitation** (documented at `with_mdfc_back`): on the
//! back-face land's activated abilities, the registry stores them
//! on the front-face `CardDefinition::activated_abilities` slot
//! today — see the activation wiring below. A later commit should
//! move activated abilities onto [`CardFace`] proper when the
//! first MDFC with distinct per-face activations lands.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    // --- Front face: Tangled Florahedron ({G} 1/1 Elf Druid) ----------
    let front_name = reg.interner_mut().intern("Tangled Florahedron");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(elf);
    front_subs.0.insert(druid);
    let front_chars = Characteristics {
        name: front_name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid front cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // --- Back face: Tangled Vale (land, tapped, {T}: {G}) -------------
    let back_name = reg.interner_mut().intern("Tangled Vale");
    let back_chars = Characteristics {
        name: back_name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes: SubtypeSet::default(),
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // Back-face mana ability. Registered on the CardDefinition as
    // an activated ability so on-battlefield activation lookup finds
    // it via `obj.card_id` → registry.activated_abilities. The
    // activation's `is_land_ability = true` via the `ActivationZone`
    // default; legal-action enumeration reaches it when the object
    // on the battlefield is the back-face land. Front-face creature
    // has no activated abilities, so the single ability here is
    // unambiguous.
    //
    // Note: this places the land's tap-for-mana onto the shared
    // `activated_abilities` list rather than a per-face slot; CR
    // strictly speaking says abilities belong to the face they're
    // printed on, so an MDFC with activations on BOTH faces would
    // need `CardFace::activated_abilities`. No current seed
    // exercises that — the simpler shared-list wiring is the one
    // that shipped.
    reg.register(
        CardDefinition::new(front_name, front_chars)
            .with_mdfc_back(back_face)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                // Back-face only: the 1/1 creature face does not
                // have a tap-for-green ability.
                face_gate: Some(1),
                effect: add_green_mana,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
