//! Necra Sanctuary — `{2}{B}` enchantment.
//! "At the beginning of your upkeep, if you control a green or white
//! permanent, target player loses 1 life. If you control a green
//! permanent and a white permanent, that player loses 3 life instead."
//!
//! The "if you control a green or white permanent" gate is a CR 603.4
//! intervening-if; the 1-vs-3 escalation is computed at resolution from
//! the board.

use arcana_core::effects::Effect;
use arcana_core::conditions;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necra Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_control_green_or_white),
            effect: sanctuary_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

/// "if you control a green or white permanent"
fn if_control_green_or_white(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent().with_colors(ColorSet::green()),
    ) || conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent().with_colors(ColorSet::white()),
    )
}

/// "target player loses 1 life. If you control a green permanent and a
/// white permanent, that player loses 3 life instead."
fn sanctuary_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let green = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::green())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let white = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::white())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let amount = if green > 0 && white > 0 { 3 } else { 1 };
    vec![Effect::LoseLife { player: *p, amount }]
}
