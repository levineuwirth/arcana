//! Across the Multiverse — `{2}{R}` enchantment. "Nontoken creatures
//! you control which aren't from the MOM main set get +1/+0 and gain
//! haste."
//!
//! Implementation: an ETB trigger installs TWO continuous effects —
//! a [`ContinuousEffect::filtered_pump`] (+1/+0) and a
//! [`ContinuousEffect::filtered_keyword`] (haste), both scoped to
//! NONTOKEN creatures you control, with duration
//! [`Duration::WhileSourceOnBattlefield`].
//!
//! GAP: the "which aren't from the MOM main set" clause is a
//! set-membership filter — `ObjectFilter` has no set/printing
//! predicate, so the effect applies to ALL nontoken creatures you
//! control (a documented approximation; only MOM-main-set creatures
//! are wrongly included).

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Across the Multiverse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
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
                effect: etb_install_nontoken_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "+1/+0" and "haste" for nontoken creatures you
/// control, anchored to this enchantment, lasting while it remains on
/// the battlefield. The MOM-main-set exclusion is GAP'd (no set filter
/// on `ObjectFilter`).
fn etb_install_nontoken_buff(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "which aren't from the MOM main set" — no set-membership
    // predicate on ObjectFilter; applies to all nontoken creatures
    // you control.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                ObjectFilter::creature()
                    .nontoken()
                    .controlled_by(ControllerConstraint::You),
                1,
                0,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                ObjectFilter::creature()
                    .nontoken()
                    .controlled_by(ControllerConstraint::You),
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
