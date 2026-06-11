//! Raka Sanctuary — `{2}{R}` enchantment.
//! "At the beginning of your upkeep, if you control a white or blue
//! permanent, this enchantment deals 1 damage to target creature. If you
//! control a white permanent and a blue permanent, this enchantment deals
//! 3 damage instead."

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Raka Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                intervening_if: Some(if_control_white_or_blue),
                effect: sanctuary_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// Intervening-if: "if you control a white or blue permanent".
fn if_control_white_or_blue(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::permanent()
            .with_colors(ColorSet::white() | ColorSet::blue()),
    )
}

/// "…deals 1 damage to target creature. If you control a white permanent
/// and a blue permanent, … 3 damage instead."
fn sanctuary_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let white = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::white()),
        trig.controller,
    );
    let blue = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::blue()),
        trig.controller,
    );
    let amount = if white > 0 && blue > 0 { 3 } else { 1 };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount,
        source: trig.source,
    }]
}
