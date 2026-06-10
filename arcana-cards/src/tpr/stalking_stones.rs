//! Stalking Stones — Land.
//! "{T}: Add {C}." and "{6}: This land becomes a 3/3 Elemental
//! artifact creature that's still a land. (This effect lasts
//! indefinitely.)"
//!
//! Animation is `AddType` (artifact creature, additive — still a land)
//! + `SetBasePT` 3/3 with `Duration::WhileSourceOnBattlefield` (the
//! closest available duration to "indefinitely"). The Elemental
//! subtype add is a GAP.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stalking Stones");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}: This land becomes a 3/3 Elemental artifact creature that's still a land. (This effect lasts indefinitely.)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Elemental creature subtype is not added (no
    // subtype-adding effect); 'indefinitely' is approximated with
    // Duration::WhileSourceOnBattlefield.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
