//! Opal Gargoyle — `{1}{W}` enchantment (Urza's Saga, 1998).
//! "When an opponent casts a creature spell, if this permanent is an
//! enchantment, it becomes a 2/2 Gargoyle creature with flying."
//!
//! Opponent creature-spell `SpellCast` trigger animating the source. GAPs:
//! the intervening-if enchantment check (no source-type condition helper)
//! and the Gargoyle subtype (no add-subtype effect). The animation is
//! AddType + SetBasePT + Flying for as long as the source stays on the
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
    let name = reg.interner_mut().intern("Opal Gargoyle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                            .with_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if "if this permanent is an enchantment"
                // — no source-type condition helper; firing unconditionally.
                intervening_if: None,
                effect: become_gargoyle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 2/2 Gargoyle creature with flying."
fn become_gargoyle(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Gargoyle creature subtype cannot be added (no add-subtype
    // effect); type, P/T, and flying are applied.
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
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
