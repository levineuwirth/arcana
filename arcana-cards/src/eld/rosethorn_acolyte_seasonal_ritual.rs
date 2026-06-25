//! Rosethorn Acolyte // Seasonal Ritual — `{2}{G}` / `{G}` Adventure
//!
//! Creature: `{2}{G}` Creature — Elf Druid (2/3)
//!   {T}: Add one mana of any color. (Modeled as five mana abilities, one
//!   per WUBRG color; the player picks the color by choosing which ability
//!   to activate.)
//!
//! Adventure: `{G}` Sorcery — Seasonal Ritual
//!   Add one mana of any color.
//!   GAP: a spell-resolve "add one mana of any color" can't use the
//!   per-color mana-ability idiom (the chooser doesn't pick an ability),
//!   and there is no chosen-color-mana follow-up; emitting Green as an
//!   approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rosethorn Acolyte");
    let elf_sub = reg.interner_mut().intern("Elf");
    let druid_sub = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf_sub);
    subtypes.0.insert(druid_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Seasonal Ritual");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Add one mana of any color.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            // Creature face: "{T}: Add one mana of any color" — one mana
            // ability per color.
            .with_activated_ability(mana_ability("{T}: Add {W}.", add_white_mana))
            .with_activated_ability(mana_ability("{T}: Add {U}.", add_blue_mana))
            .with_activated_ability(mana_ability("{T}: Add {B}.", add_black_mana))
            .with_activated_ability(mana_ability("{T}: Add {R}.", add_red_mana))
            .with_activated_ability(mana_ability("{T}: Add {G}.", add_green_mana))
            .with_adventure(adventure),
    )
}

/// `{T}: Add one mana of any color` — one tap-only mana ability per color.
fn mana_ability(
    text: &str,
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
        face_gate: None,
        effect,
    }
}

fn add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
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

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: spell-resolve "any color" — no chosen-color-mana follow-up;
    // emitting Green as approximation.
    vec![Effect::AddMana { player: entry.controller, mana: vec![ManaUnit::plain(ManaColor::Green, entry.source)] }]
}
