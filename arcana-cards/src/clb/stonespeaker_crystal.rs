//! Stonespeaker Crystal — `{4}` artifact.
//! "{T}: Add {C}{C}." and "{2}, {T}, Sacrifice this artifact: Exile
//! any number of target players' graveyards. Draw a card."
//!
//! GAP: "Exile any number of target players' graveyards" — there is
//! no exile-entire-graveyard Effect (only single-card
//! `ExileFromGraveyard` with a known id); the targets are declared
//! but only the draw resolves.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stonespeaker Crystal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}{C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_colorless,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice this artifact: Exile any number \
                       of target players' graveyards. Draw a card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Any,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_graveyards_draw,
            }),
    )
}

fn add_two_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 2],
    }]
}

fn exile_graveyards_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exile any number of target players' graveyards" — no
    // exile-whole-graveyard Effect variant; only the draw is wired.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
