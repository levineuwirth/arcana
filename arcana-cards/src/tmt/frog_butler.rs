//! Frog Butler — `{1}{G}` 1/1 Frog Spirit with Deathtouch.
//! "{T}: Add one mana of any color.
//!  {2}: This creature gains reach until end of turn."
//! "any color" is modeled as five {T} mana abilities, one per WUBRG color
//! (the shared tap cost means only one fires).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frog Butler");
    let frog = reg.interner_mut().intern("Frog");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{T}: Add one mana of any color" — five tap mana abilities, one per color.
            .with_activated_ability(fb_mana_ability("{T}: Add {W}.", fb_add_white))
            .with_activated_ability(fb_mana_ability("{T}: Add {U}.", fb_add_blue))
            .with_activated_ability(fb_mana_ability("{T}: Add {B}.", fb_add_black))
            .with_activated_ability(fb_mana_ability("{T}: Add {R}.", fb_add_red))
            .with_activated_ability(fb_mana_ability("{T}: Add {G}.", fb_add_green))
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature gains reach until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_reach,
            }),
    )
}

fn gain_reach(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Reach,
        duration: Duration::EndOfTurn,
    }]
}

fn fb_mana_ability(
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

fn fb_add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn fb_add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    fb_add_one(ctx, ManaColor::White)
}
fn fb_add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    fb_add_one(ctx, ManaColor::Blue)
}
fn fb_add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    fb_add_one(ctx, ManaColor::Black)
}
fn fb_add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    fb_add_one(ctx, ManaColor::Red)
}
fn fb_add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    fb_add_one(ctx, ManaColor::Green)
}
