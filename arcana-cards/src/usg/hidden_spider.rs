//! Hidden Spider — `{G}` enchantment (Urza's Saga, 1998).
//! "When an opponent casts a creature spell with flying, if this permanent
//! is an enchantment, it becomes a 3/5 Spider creature with reach."
//!
//! Opponent creature-spell `SpellCast` trigger. GAPs: the "with flying"
//! spell restriction (no keyword predicate on ObjectFilter here), the
//! intervening-if enchantment check (no source-type condition helper), and
//! the Spider subtype (no add-subtype effect). The animation itself is
//! AddType + SetBasePT + Reach for as long as the source stays on the
//! battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Hidden Spider");
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
                // GAP: trigger — "a creature spell WITH FLYING"; ObjectFilter
                // has no keyword predicate in this catalog, so the filter is
                // creature-spells-only (over-fires on non-flyers).
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if "if this permanent is an enchantment"
                // — no source-type condition helper; firing unconditionally.
                intervening_if: None,
                effect: become_spider,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 3/5 Spider creature with reach."
fn become_spider(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Spider creature subtype cannot be added (no add-subtype
    // effect); type, P/T, and reach are applied.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 5,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Reach,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
