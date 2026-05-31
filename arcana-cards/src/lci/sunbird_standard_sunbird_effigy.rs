//! Sunbird Standard // Sunbird Effigy — {3} Artifact (transforming DFC, layout "transform").
//!
//! Front (Sunbird Standard):
//!   {T}: Add one mana of any color.
//!   Craft with one or more {5} ({5}, Exile this artifact, exile one or more other permanents
//!     you control and/or cards from your graveyard: Return this card transformed under its
//!     owner's control. Craft only as a sorcery.)
//! ---
//! Back (Sunbird Effigy) — Artifact Creature — Bird Construct, Flying, Vigilance, Haste:
//!   Power and toughness each equal to the number of colors among the exiled cards used to
//!     craft it.
//!   {T}: For each color among the exiled cards used to craft this creature, add one mana of
//!     that color.
//!
//! GAP (front mana ability): "Add one mana of any color" — Effect::AddMana takes concrete
//! ManaUnit colors and the engine has no any-color player-choice mana primitive. Best-effort:
//! a single mana is produced (white chosen as a placeholder; the color choice is fidelity GAP).
//!
//! GAP (Craft): the Craft keyword/transform-cost mechanic (exile this + exile other permanents
//! and/or graveyard cards as a special action to return transformed) is not in the supported
//! keyword surface and has no engine primitive. Not expressed.
//!
//! GAP (back P/T): "power and toughness each equal to the number of colors among the exiled
//! cards used to craft it" — the set of cards exiled to craft is not tracked, so the dynamic
//! characteristic-defining P/T is not expressible. Back face printed as 0/0 placeholder.
//!
//! GAP (back mana ability): "{T}: For each color among the exiled cards used to craft this
//! creature, add one mana of that color" — same craft-exile-set dependency; not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunbird Standard");
    let bird = reg.interner_mut().intern("Bird");
    let construct = reg.interner_mut().intern("Construct");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Sunbird Effigy");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bird);
    back_subtypes.0.insert(construct);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            // GAP: dynamic P/T (= number of colors among exiled crafted cards); 0/0 placeholder.
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![
                arcana_core::effects::KeywordAbility::Flying,
                arcana_core::effects::KeywordAbility::Vigilance,
                arcana_core::effects::KeywordAbility::Haste,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: Add one mana of any color (color choice is a GAP — see header).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: Default::default(),
                is_instant_speed: false,
                face_gate: Some(0),
                effect: add_one_mana,
            }),
    )
}

fn add_one_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "any color" — no any-color player-choice mana primitive; white as placeholder.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}
