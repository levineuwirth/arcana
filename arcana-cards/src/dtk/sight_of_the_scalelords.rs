//! Sight of the Scalelords — `{4}{G}` enchantment.
//! "At the beginning of combat on your turn, creatures you control with
//! toughness 4 or greater get +2/+2 and gain vigilance until end of
//! turn."
//!
//! `ObjectFilter` has no min-toughness refinement, so the toughness-4+
//! cut is applied per-id via `script::toughness_of` at resolution.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sight of the Scalelords");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_tough_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…creatures you control with toughness 4 or greater get +2/+2 and
/// gain vigilance until end of turn."
fn pump_tough_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let tough: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    )
    .into_iter()
    .filter(|&id| script::toughness_of(state, id) >= 4)
    .collect();
    if tough.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: tough,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Vigilance],
        }),
    }]
}
