//! Ceta Sanctuary — `{2}{U}` enchantment.
//! "At the beginning of your upkeep, if you control a red or green
//! permanent, draw a card, then discard a card. If you control a red
//! permanent and a green permanent, instead draw two cards, then
//! discard a card."
//!
//! The "red or green" gate is the intervening-if; the upgraded
//! "both colors" branch is decided at resolution.

use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Ceta Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                intervening_if: Some(if_control_red_or_green),
                effect: sanctuary_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if you control a red or green permanent…"
fn if_control_red_or_green(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::red() | ColorSet::green()),
    )
}

/// "…draw a card, then discard a card. If you control a red permanent
/// and a green permanent, instead draw two cards, then discard a card."
fn sanctuary_loot(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let red = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::red()),
        trig.controller,
    );
    let green = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::green()),
        trig.controller,
    );
    let draws = if red > 0 && green > 0 { 2 } else { 1 };
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: draws,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
