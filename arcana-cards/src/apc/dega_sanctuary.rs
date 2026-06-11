//! Dega Sanctuary — `{2}{W}` enchantment (Apocalypse, 2001).
//! "At the beginning of your upkeep, if you control a black or red
//! permanent, you gain 2 life. If you control a black permanent and a
//! red permanent, you gain 4 life instead."
//!
//! Intervening-if (CR 603.4): you control a black or red permanent.
//! The 2-vs-4 escalation is computed at resolution from the live
//! board.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dega Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_control_black_or_red),
                effect: sanctuary_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if you control a black or red permanent…"
fn if_control_black_or_red(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::black())
            .controlled_by(ControllerConstraint::You),
    ) || conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::red())
            .controlled_by(ControllerConstraint::You),
    )
}

/// "…you gain 2 life. If you control a black permanent and a red
/// permanent, you gain 4 life instead."
fn sanctuary_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let has_black = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::black())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    ) > 0;
    let has_red = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::red())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    ) > 0;
    if !has_black && !has_red {
        return Vec::new();
    }
    let amount = if has_black && has_red { 4 } else { 2 };
    vec![Effect::GainLife {
        player: trig.controller,
        amount,
    }]
}
