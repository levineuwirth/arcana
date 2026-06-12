//! Spirit of Resistance — `{2}{W}` enchantment. "As long as you
//! control a permanent of each color, prevent all damage that would
//! be dealt to you."
//!
//! Implementation: an ETB trigger installs a
//! `ReplacementCondition::WouldDealDamageToSpecific` replacement on
//! the controller's player with `ReplacementKind::PreventAllDamage`;
//! the "as long as you control a permanent of each color" half lives
//! on a custom `state_gate` built from `conditions::you_control_a`
//! over each of the five colors. Lasts while the enchantment is on
//! the battlefield.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit of Resistance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_prevention,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// State gate: do you control at least one permanent of EACH of the
/// five colors?
fn controls_permanent_of_each_color(
    state: &GameState,
    _src: ObjectId,
    you: PlayerId,
) -> bool {
    [
        ColorSet::white(),
        ColorSet::blue(),
        ColorSet::black(),
        ColorSet::red(),
        ColorSet::green(),
    ]
    .into_iter()
    .all(|c| {
        conditions::you_control_a(
            state,
            you,
            &ObjectFilter::permanent().with_colors(c),
        )
    })
}

/// ETB trigger: install "prevent all damage that would be dealt to
/// you" gated on controlling a permanent of each color, anchored to
/// this enchantment.
fn etb_install_prevention(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldDealDamageToSpecific {
                target: DamageTarget::Player(trig.controller),
            },
            kind: ReplacementKind::PreventAllDamage,
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: Some(controls_permanent_of_each_color),
        }),
    }]
}
