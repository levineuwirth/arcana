//! Sigarda's Summons — `{4}{W}{W}` enchantment. "Creatures you
//! control with +1/+1 counters on them have base power and toughness
//! 4/4, have flying, and are Angels in addition to their other types."
//!
//! Implementation: the "have flying" clause is installed as a
//! [`ContinuousEffect::filtered_keyword`] scoped to creatures you
//! control with a +1/+1 counter. The base-P/T-setting and type-adding
//! clauses are not expressible with the Wave-1 static surface and are
//! gapped below.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigarda's Summons");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control with +1/+1 counters on
/// them have flying", anchored to this enchantment.
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "have base power and toughness 4/4" (no base-P/T-setting
    // builder) and "are Angels in addition to their other types" (no
    // type-adding builder) are not expressible; only the flying grant is
    // installed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter {
                has_counter: Some(CounterKind::PlusOnePlusOne),
                ..ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
            },
            KeywordAbility::Flying,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
