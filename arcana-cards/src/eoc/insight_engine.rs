//! Insight Engine — `{2}{U}` artifact (Mirrodin Besieged, 2011).
//! "{2}, {T}: Put a charge counter on this artifact, then draw a card for
//! each charge counter on it."
//!
//! A blue non-creature artifact with one activated ability. The cost
//! ({2}, {T}) is faithful; the effect adds a charge counter and then
//! draws a card for each charge counter now on it (dynamic via
//! script::source_counter_count, +1 for the counter just added).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Insight Engine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}: Put a charge counter on this artifact, then draw a \
                   card for each charge counter on it."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: charge_then_draw,
        }),
    )
}

fn charge_then_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Charge counters AFTER adding one this activation.
    let charges = script::source_counter_count(state, ctx.source, CounterKind::Charge) + 1;
    vec![Effect::Sequence(vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Charge,
            count: 1,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: charges,
        },
    ])]
}
