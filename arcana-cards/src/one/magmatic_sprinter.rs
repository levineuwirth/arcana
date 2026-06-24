//! Magmatic Sprinter — `{2}{R}` 3/2 Phyrexian Warrior.
//! Haste; "When this creature enters, put two oil counters on target
//! artifact or creature you control."; "At the beginning of your end
//! step, return this creature to its owner's hand unless you remove two
//! oil counters from it."
//!
//! Haste + the ETB two-oil-counters trigger are wired. The end-step
//! ability is GAP'd at the effect level: "unless you remove two oil
//! counters" is a counter-REMOVAL payment, and OptionalPaymentKind has
//! Mana / Life / Sacrifice / Discard but no remove-counters cost — so the
//! unless-pay gate is inexpressible (returning unconditionally would be
//! wrong). The trigger is wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magmatic Sprinter");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warrior = reg.interner_mut().intern("Warrior");
    let _oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_oil_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_bounce_unless_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_two_oil_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let Some(oil) = reg.interner().lookup("oil") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(oil),
        count: 2,
    }]
}

fn end_step_bounce_unless_pay(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return this to hand unless you remove two oil counters from it"
    //      — OptionalPaymentKind (Mana/Life/Sacrifice/Discard) has no
    //      counter-removal cost; the unless-pay gate is inexpressible.
    Vec::new()
}
