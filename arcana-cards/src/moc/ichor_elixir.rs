//! Ichor Elixir — `{4}` artifact.
//! "If you would roll one or more planar dice, instead roll that many
//! planar dice plus one and ignore one." and "{T}: Add {C}{C}." The
//! planar-dice replacement is a GAP (Planechase dice are not modeled);
//! the mana ability is wired.

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
    let name = reg.interner_mut().intern("Ichor Elixir");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    // GAP: "If you would roll one or more planar dice, instead roll that
    // many planar dice plus one and ignore one" — Planechase planar dice
    // and dice-roll replacement effects are not modeled.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}{C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_colorless,
            },
        ),
    )
}

fn add_two_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
        ],
    }]
}
