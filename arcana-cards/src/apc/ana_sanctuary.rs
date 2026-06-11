//! Ana Sanctuary — `{2}{G}` enchantment.
//! "At the beginning of your upkeep, if you control a blue or black
//! permanent, target creature gets +1/+1 until end of turn. If you
//! control a blue permanent and a black permanent, that creature gets
//! +5/+5 until end of turn instead."
//!
//! The "if you control a blue or black permanent" gate is a CR 603.4
//! intervening-if; the +1/+1 vs +5/+5 branch is decided at resolution
//! from the live board.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ana Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
            intervening_if: Some(if_control_blue_or_black),
            effect: sanctuary_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

/// "…if you control a blue or black permanent…"
fn if_control_blue_or_black(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent().with_colors(ColorSet::blue() | ColorSet::black()),
    )
}

/// "…target creature gets +1/+1 until end of turn. If you control a
/// blue permanent and a black permanent, … +5/+5 … instead."
fn sanctuary_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let has_blue = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::blue())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    ) > 0;
    let has_black = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::black())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    ) > 0;
    let bonus = if has_blue && has_black { 5 } else { 1 };
    vec![Effect::Pump {
        target: *id,
        power: bonus,
        toughness: bonus,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
