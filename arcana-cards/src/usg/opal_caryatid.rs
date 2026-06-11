//! Opal Caryatid — `{W}` enchantment.
//! "When an opponent casts a creature spell, if this permanent is an
//! enchantment, it becomes a 2/2 Soldier creature."
//!
//! An opponent creature-spell `SpellCast` trigger animating this
//! permanent (AddType + SetBasePT, battlefield-bound duration).
//! Documented GAPs: the intervening "if this permanent is an
//! enchantment" guard has no `conditions::` predicate (the animation is
//! idempotent once awake), and the Soldier subtype cannot be added by an
//! effect.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Opal Caryatid");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(
                        TypeLine::CREATURE.into(),
                    )),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if "if this permanent is an
                // enchantment" — no conditions:: predicate checks the
                // source's current types; fires unconditionally (the
                // animation is idempotent once awake).
                intervening_if: None,
                effect: awaken,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 2/2 Soldier creature."
fn awaken(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Soldier subtype cannot be added (no add-subtype effect);
    // the permanent animation is modeled with the battlefield-bound
    // duration.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 2,
            toughness: 2,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
