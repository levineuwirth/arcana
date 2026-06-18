//! Taeko, the Patient Avalanche — `{3}{U}` 4/5 Legendary Turtle Ninja.
//! Taeko enters tapped (modeled as an ETB self-tap).
//! Whenever another creature you control leaves the battlefield, if it didn't die,
//! scry 1 and put a +1/+1 counter on Taeko. (The "leaves the battlefield" condition
//! needs an unconstrained destination + a didn't-die intervening-if — GAP'd.)
//! Whenever Taeko attacks, you may pay {U/B}; if you do, target attacking creature
//! can't be blocked this turn. (Modeled as a SelfAttacks OptionalPayment; the
//! reflexive "when you do" target is approximated to Taeko itself — GAP'd target choice.)

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Taeko, the Patient Avalanche");
    let turtle = reg.interner_mut().intern("Turtle");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "Whenever another creature you control leaves the battlefield, if it
            // didn't die, scry 1 and put a +1/+1 counter on Taeko" — ZoneChange requires a
            // concrete destination (leaves = any zone) plus a didn't-die intervening-if;
            // not expressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_may_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enters_tapped(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Tap { target: trig.source }]
}

fn attack_may_pay(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reflexive "When you do, target attacking creature can't be blocked" —
    // the chosen attacking creature target is approximated to Taeko itself.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U/B}").expect("valid cost")),
        then: Box::new(Effect::CantBeBlocked {
            target: trig.source,
            duration: Duration::EndOfTurn,
        }),
        else_effect: None,
    }]
}
