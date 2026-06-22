//! The Warring Triad — `{3}` 5/5 Legendary Artifact Creature — God (colorless).
//! Flying, trample, haste.
//! As long as there are fewer than eight cards in your graveyard, The Warring
//! Triad isn't a creature.
//! {T}, Mill a card: Target player adds one mana of any color.
//!
//! Flying / Trample / Haste are base keywords (Scryfall's "Mill" comes from the
//! activation cost, not a creature keyword). The conditional "isn't a creature"
//! static is GAP'd (no type-removal static form). The activated ability is
//! wired with the {T} cost and a target player, but its "Mill a card" cost
//! component has no ActivationCost field, and its "adds one mana of any color"
//! payload has no any-color mana primitive — both GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Warring Triad");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    // GAP: static "As long as there are fewer than eight cards in your
    //      graveyard, this isn't a creature" — no conditional type-removal
    //      static form is expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Mill a card: Target player adds one mana of any color.".into(),
            // GAP (cost): "Mill a card" has no ActivationCost field; only {T}
            //             is paid.
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_any_color,
        }),
    )
}

fn add_any_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Target player adds one mana of any color" — AddMana only mints a
    //      specific ManaColor pip; there is no any-color / chosen-color mana
    //      primitive in scope.
    Vec::new()
}
