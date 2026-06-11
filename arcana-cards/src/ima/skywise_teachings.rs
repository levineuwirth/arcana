//! Skywise Teachings — `{3}{U}` enchantment.
//! "Whenever you cast a noncreature spell, you may pay {1}{U}. If you
//! do, create a 2/2 blue Djinn Monk creature token with flying."
//!
//! A filtered `SpellCast` trigger; the pay-gate is an
//! `Effect::OptionalPayment` whose `then` mints the token.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skywise Teachings");
    // Pre-intern the token subtypes so the resolver's read-only lookup
    // finds them.
    let _djinn = reg.interner_mut().intern("Djinn");
    let _monk = reg.interner_mut().intern("Monk");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: may_pay_for_djinn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {1}{U}. If you do, create a 2/2 blue Djinn Monk
/// creature token with flying."
fn may_pay_for_djinn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Djinn") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Monk") {
        subtypes.0.insert(s);
    }
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}{U}").expect("valid cost"),
        ),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: reg.interner().lookup("Djinn").unwrap_or_default(),
                colors: ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
