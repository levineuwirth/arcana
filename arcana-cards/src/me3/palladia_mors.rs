//! Palladia-Mors — `{2}{R}{R}{G}{G}{W}{W}` 7/7 Legendary Elder Dragon
//! with Flying and Trample. "At the beginning of your upkeep,
//! sacrifice Palladia-Mors unless you pay {R}{G}{W}."
//!
//! Flying and Trample are base keywords. The upkeep trigger offers an
//! optional {R}{G}{W} payment; if unpaid, the source is sacrificed.
//!
//! NOTE: "sacrifice ~" is modeled as `DestroyPermanent { target:
//! trig.source }` (the established self-removal idiom at this engine
//! fidelity — there is no id-targeted Sacrifice effect).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Palladia-Mors");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_sacrifice_unless_pay,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_sacrifice_unless_pay(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // pay {R}{G}{W} = no-op; don't pay = sacrifice (self-destroy) this.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DestroyPermanent { target: trig.source })),
    }]
}
