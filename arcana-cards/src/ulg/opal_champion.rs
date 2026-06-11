//! Opal Champion — `{2}{W}` enchantment.
//! "When an opponent casts a creature spell, if this permanent is an
//! enchantment, it becomes a 3/3 Knight creature with first strike."
//!
//! Animation is modeled as AddType(CREATURE) + SetBasePT(3/3) +
//! GrantKeyword(FirstStrike) with `Duration::WhileSourceOnBattlefield`
//! (no "permanently" duration exists). GAP: the intervening-if "if
//! this permanent is an enchantment" has no source-type predicate, so
//! the trigger re-fires after animation; GAP: the Knight subtype
//! cannot be added (no add-subtype effect).

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
    let name = reg.interner_mut().intern("Opal Champion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
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
                        ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if — "if this permanent is an
                // enchantment" (i.e. not yet animated) has no available
                // predicate; the trigger fires unconditionally.
                intervening_if: None,
                effect: awaken_champion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 3/3 Knight creature with first strike."
/// GAP: the Knight subtype is not addable (no add-subtype effect).
fn awaken_champion(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
