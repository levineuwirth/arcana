//! Veiled Apparition — `{1}{U}` enchantment.
//! "When an opponent casts a spell, if this permanent is an enchantment,
//! it becomes a 3/3 Illusion creature with flying and 'At the beginning
//! of your upkeep, sacrifice this creature unless you pay {1}{U}.'"
//!
//! // GAP: intervening-if — "if this permanent is an enchantment" (a
//! // source-type check) has no `conditions::` predicate; the trigger
//! // fires unconditionally (re-animation is idempotent).
//! // GAP: the animated creature should gain the Illusion subtype; no
//! // effect sets subtypes.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veiled Apparition");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: become_illusion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 3/3 Illusion creature with flying and 'At the
/// beginning of your upkeep, sacrifice this creature unless you pay
/// {1}{U}.'"
fn become_illusion(
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
            keyword: KeywordAbility::Flying,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantTriggeredAbility {
            target: trig.source,
            ability: Box::new(TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_tax,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}

/// Granted: "At the beginning of your upkeep, sacrifice this creature
/// unless you pay {1}{U}."
fn upkeep_tax(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}{U}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter {
                name: reg.interner().lookup("Veiled Apparition"),
                ..ObjectFilter::default()
            },
            count: 1,
        })),
    }]
}
