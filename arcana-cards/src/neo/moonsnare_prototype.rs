//! Moonsnare Prototype — `{U}` artifact.
//! "{T}, Tap an untapped artifact or creature you control: Add {C}."
//! and "Channel — {4}{U}, Discard this card: The owner of target
//! nonland permanent puts it on their choice of the top or bottom of
//! their library." The mana ability's extra tap-another-permanent cost
//! is a GAP (no such ActivationCost field); the Channel activation is
//! wired from hand with discard_self, with the owner's top-or-bottom
//! choice approximated as top of library (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moonsnare Prototype");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: additional cost 'Tap an untapped artifact or
                // creature you control' — no tap-another-permanent
                // ActivationCost field; wired as tap-only
                text: "{T}, Tap an untapped artifact or creature you \
                       control: Add {C}."
                    .into(),
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
                text: "Channel — {4}{U}, Discard this card: The owner of \
                       target nonland permanent puts it on their choice of \
                       the top or bottom of their library."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: channel_bounce_to_library,
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

/// Channel: put target nonland permanent on its owner's library.
// GAP: 'their choice of the top or bottom' — the owner's top-or-bottom
// choice is not expressible; approximated as top of library
fn channel_bounce_to_library(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
