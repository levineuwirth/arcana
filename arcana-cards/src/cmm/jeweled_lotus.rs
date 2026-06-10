//! Jeweled Lotus — `{0}` artifact.
//! "{T}, Sacrifice this artifact: Add three mana of any one color.
//! Spend this mana only to cast your commander." The any-one-color
//! choice is modeled as five mana abilities (one per WUBRG color,
//! each adding three pips); the commander spend restriction is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeweled Lotus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    // GAP: "Spend this mana only to cast your commander" — mana spend
    // restrictions are not expressible; plain mana abilities emitted.
    let mut def = CardDefinition::new(name, chars);
    for (text, effect) in [
        ("{T}, Sacrifice this artifact: Add {W}{W}{W}.", add_white as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}, Sacrifice this artifact: Add {U}{U}{U}.", add_blue),
        ("{T}, Sacrifice this artifact: Add {B}{B}{B}.", add_black),
        ("{T}, Sacrifice this artifact: Add {R}{R}{R}.", add_red),
        ("{T}, Sacrifice this artifact: Add {G}{G}{G}.", add_green),
    ] {
        def = def.with_activated_ability(ActivatedAbilityDef {
            text: text.into(),
            cost: ActivationCost {
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect,
        });
    }
    reg.register(def)
}

fn add_three(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source); 3],
    }]
}

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three(ManaColor::White, ctx)
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three(ManaColor::Blue, ctx)
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three(ManaColor::Black, ctx)
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three(ManaColor::Red, ctx)
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three(ManaColor::Green, ctx)
}
