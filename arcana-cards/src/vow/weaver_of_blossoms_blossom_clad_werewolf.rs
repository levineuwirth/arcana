//! Weaver of Blossoms // Blossom-Clad Werewolf
//!
//! Front face: Creature — Human Werewolf, 2/3, {2}{G}
//!   {T}: Add one mana of any color.
//!     "any color" is modeled as five front-face (face_gate 0) {T} mana
//!     abilities, one per WUBRG color; the shared tap cost means only one fires.
//!   Daybound (If a player casts no spells during their own turn, it becomes night next turn.)
//!     GAP: Daybound keyword / day-night cycle not modeled.
//!
//! Back face: Creature — Werewolf, 5/5.
//!   {T}: Add two mana of any one color.
//!     "any one color" is modeled as five back-face (face_gate 1) {T} mana
//!     abilities, each producing TWO mana of one color.
//!   Nightbound (If a player casts at least two spells during their own turn, it becomes day.)
//!     GAP: Nightbound keyword / day-night cycle not modeled.
//!
//! GAP: Daybound and Nightbound transform conditions not modeled (day/night cycle not in engine).
//!   Transform triggers are omitted; the card is registered without transform triggers.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weaver of Blossoms");

    let sub_human = reg.interner_mut().intern("Human");
    let sub_werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Blossom-Clad Werewolf
    let back_name = reg.interner_mut().intern("Blossom-Clad Werewolf");
    let back_sub_werewolf = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_sub_werewolf);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: "{T}: Add one mana of any color" — five color abilities.
            .with_activated_ability(wb_mana_ability("{T}: Add {W}.", Some(0), wb_front_white))
            .with_activated_ability(wb_mana_ability("{T}: Add {U}.", Some(0), wb_front_blue))
            .with_activated_ability(wb_mana_ability("{T}: Add {B}.", Some(0), wb_front_black))
            .with_activated_ability(wb_mana_ability("{T}: Add {R}.", Some(0), wb_front_red))
            .with_activated_ability(wb_mana_ability("{T}: Add {G}.", Some(0), wb_front_green))
            // Back face: "{T}: Add two mana of any one color" — five color abilities,
            // each producing two mana of that color.
            .with_activated_ability(wb_mana_ability("{T}: Add {W}{W}.", Some(1), wb_back_white))
            .with_activated_ability(wb_mana_ability("{T}: Add {U}{U}.", Some(1), wb_back_blue))
            .with_activated_ability(wb_mana_ability("{T}: Add {B}{B}.", Some(1), wb_back_black))
            .with_activated_ability(wb_mana_ability("{T}: Add {R}{R}.", Some(1), wb_back_red))
            .with_activated_ability(wb_mana_ability("{T}: Add {G}{G}.", Some(1), wb_back_green))
            // GAP: Daybound/Nightbound day-night cycle not modeled.
            // No transform triggers — the transform condition is not expressible.
    )
}

fn wb_mana_ability(
    text: &str,
    face_gate: Option<u8>,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate,
        effect,
    }
}

fn wb_add(ctx: &ActivationContext, color: ManaColor, count: usize) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source); count],
    }]
}

fn wb_front_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::White, 1)
}
fn wb_front_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Blue, 1)
}
fn wb_front_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Black, 1)
}
fn wb_front_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Red, 1)
}
fn wb_front_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Green, 1)
}

fn wb_back_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::White, 2)
}
fn wb_back_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Blue, 2)
}
fn wb_back_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Black, 2)
}
fn wb_back_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Red, 2)
}
fn wb_back_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    wb_add(ctx, ManaColor::Green, 2)
}
