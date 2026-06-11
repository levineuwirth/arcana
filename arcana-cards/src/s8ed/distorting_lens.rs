//! Distorting Lens — `{2}` artifact.
//! "{T}: Target permanent becomes the color of your choice until end of
//! turn."
//!
//! The color choice is modeled as five parallel activated abilities,
//! one per color — choosing which ability to activate IS the color
//! choice (the same idiom the catalog prescribes for "add one mana of
//! any color").

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

fn target_permanent_req() -> Vec<TargetRequirement> {
    vec![TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::permanent()),
        count: TargetCount::Exactly(1),
        controller: None,
    }]
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Distorting Lens");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target permanent becomes white until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: target_permanent_req(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_white,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target permanent becomes blue until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: target_permanent_req(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_blue,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target permanent becomes black until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: target_permanent_req(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_black,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target permanent becomes red until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: target_permanent_req(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_red,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target permanent becomes green until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: target_permanent_req(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_green,
            }),
    )
}

fn set_color(ctx: &ActivationContext, colors: ColorSet) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::SetColor {
        target: *id,
        colors,
        duration: Duration::EndOfTurn,
    }]
}

fn become_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    set_color(ctx, ColorSet::white())
}

fn become_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    set_color(ctx, ColorSet::blue())
}

fn become_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    set_color(ctx, ColorSet::black())
}

fn become_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    set_color(ctx, ColorSet::red())
}

fn become_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    set_color(ctx, ColorSet::green())
}
