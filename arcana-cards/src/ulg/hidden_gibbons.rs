//! Hidden Gibbons — `{G}` enchantment (Urza's Saga, 1998).
//! "When an opponent casts an instant spell, if this permanent is an
//! enchantment, it becomes a 4/4 Ape creature."
//!
//! Wired on a filtered opponent `SpellCast`; the animation is the
//! AddType + SetBasePT pair anchored while the source remains on the
//! battlefield. GAPs: the "if this permanent is an enchantment"
//! intervening-if (no type-of-source predicate; harmless — the effect
//! is idempotent) and the Ape subtype (no add-subtype effect).

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
    let name = reg.interner_mut().intern("Hidden Gibbons");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::INSTANT.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if "if this permanent is an enchantment"
                // — no conditions:: predicate inspects the source's own
                // types; firing again is idempotent (it is already a 4/4
                // creature), so the gate is omitted.
                intervening_if: None,
                effect: animate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 4/4 Ape creature."
fn animate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Ape creature subtype — no catalog Effect adds a subtype.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 4,
            toughness: 4,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
