//! The Warring Triad — `{3}` 5/5 Legendary Artifact Creature — God (colorless).
//! Flying, trample, haste.
//! As long as there are fewer than eight cards in your graveyard, The Warring
//! Triad isn't a creature.
//! {T}, Mill a card: Target player adds one mana of any color.
//!
//! Flying / Trample / Haste are base keywords (Scryfall's "Mill" comes from the
//! activation cost, not a creature keyword). The conditional "isn't a creature"
//! static is GAP'd (no type-removal static form). "Target player adds one mana
//! of any color" is modeled as five mana abilities, one per WUBRG color — the
//! activating player picks the color by choosing which ability to activate, and
//! the mana is added to the chosen target player.
//! GAP (cost): "Mill a card" has no ActivationCost field; only {T} is paid.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

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
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{T}, Mill a card: Target player adds {W}.", add_white_mana))
            .with_activated_ability(mana_ability("{T}, Mill a card: Target player adds {U}.", add_blue_mana))
            .with_activated_ability(mana_ability("{T}, Mill a card: Target player adds {B}.", add_black_mana))
            .with_activated_ability(mana_ability("{T}, Mill a card: Target player adds {R}.", add_red_mana))
            .with_activated_ability(mana_ability("{T}, Mill a card: Target player adds {G}.", add_green_mana)),
    )
}

/// `{T}, Mill a card: Target player adds one mana of any color` — one
/// mana ability per color; the mana is added to the chosen target player.
/// GAP (cost): "Mill a card" has no ActivationCost field; only {T} is paid.
fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: vec![TargetRequirement::target_player()],
        // Not a mana ability: it targets (CR 605.1a — abilities that
        // target are not mana abilities), matching the original wiring.
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    // Mana goes to the chosen target player; fall back to the controller
    // if no player target was recorded.
    let recipient = match ctx.targets.targets.first() {
        Some(TargetChoice::Player(p)) => *p,
        _ => ctx.controller,
    };
    vec![Effect::AddMana {
        player: recipient,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn add_white_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::White)
}
fn add_blue_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Blue)
}
fn add_black_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Black)
}
fn add_red_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Red)
}
fn add_green_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Green)
}
