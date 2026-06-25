//! Hydro-Channeler — `{1}{U}` 1/3 Merfolk Wizard.
//! "{T}: Add {U}. Spend this mana only to cast an instant or sorcery spell."
//! "{1}, {T}: Add one mana of any color. Spend this mana only to cast an
//! instant or sorcery spell."
//!
//! Ability 1 adds {U} (the "only for instants/sorceries" spend restriction is
//! a fidelity gap — no mana-restriction field). Ability 2's "any color" is
//! modeled as five mana abilities, one per WUBRG color (the player picks the
//! color by choosing which ability to activate; the shared {1}, {T} cost means
//! only one fires).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hydro-Channeler");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}. Spend this mana only to cast an instant or sorcery spell.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue,
            })
            .with_activated_ability(any_color_ability("{1}, {T}: Add {W}. Spend this mana only to cast an instant or sorcery spell.", add_white))
            .with_activated_ability(any_color_ability("{1}, {T}: Add {U}. Spend this mana only to cast an instant or sorcery spell.", add_blue_any))
            .with_activated_ability(any_color_ability("{1}, {T}: Add {B}. Spend this mana only to cast an instant or sorcery spell.", add_black_any))
            .with_activated_ability(any_color_ability("{1}, {T}: Add {R}. Spend this mana only to cast an instant or sorcery spell.", add_red_any))
            .with_activated_ability(any_color_ability("{1}, {T}: Add {G}. Spend this mana only to cast an instant or sorcery spell.", add_green_any)),
    )
}

fn any_color_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    // Fidelity gap: the "spend only on instant/sorcery" restriction has no mana-restriction field.
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{1}").expect("valid cost"),
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity gap: the "spend only on instant/sorcery" restriction has no mana-restriction field.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)] }]
}
fn add_blue_any(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)] }]
}
fn add_black_any(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)] }]
}
fn add_red_any(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)] }]
}
fn add_green_any(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)] }]
}
