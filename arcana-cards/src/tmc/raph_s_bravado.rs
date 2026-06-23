//! Raph's Bravado — `{1}{R}` enchantment. "During your turn, attacking
//! creatures get +1/+0."
//!
//! Implementation: an ETB trigger installs a `filtered_pump` of +1/+0
//! over ATTACKING creatures (board-wide — the oracle says "attacking
//! creatures", not "you control", so no controller constraint), with
//! `Duration::WhileControllerTurn` so the buff is live only while the
//! source's controller is the active player (the "during your turn"
//! gate). The layer-cleanup pipeline auto-expires it when the
//! enchantment leaves the battlefield.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raph's Bravado");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "during your turn, attacking creatures get
/// +1/+0" as a board-wide attacking-creature filtered pump scoped to
/// the controller's turn.
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature().attacking_only(),
            1,
            0,
            Duration::WhileControllerTurn,
        ),
    }]
}
