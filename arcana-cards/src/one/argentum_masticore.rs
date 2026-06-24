//! Argentum Masticore — `{5}` 5/5 Artifact Creature — Phyrexian Masticore.
//! First strike, protection from multicolored.
//! At the beginning of your upkeep, sacrifice this creature unless you
//! discard a card. When you discard a card this way, destroy target
//! nonland permanent an opponent controls with mana value less than or
//! equal to the mana value of the discarded card.
//!
//! The upkeep "sacrifice this creature unless you discard a card" gate is wired
//! via Effect::OptionalPayment { Discard(1) → avoid penalty; else_effect =
//! destroy this creature }.
//!
//! GAP: protection from multicolored is not an expressible KeywordAbility.
//! GAP: the reflexive "When you discard a card this way, destroy target nonland
//! permanent an opponent controls with mana value ≤ the discarded card's mana
//! value" sub-trigger is not expressible — there is no reflexive when-you-discard
//! sub-trigger, and the discarded card's mana value (which bounds the destroy
//! target) is not readable. That payoff is omitted.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Argentum Masticore");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let masticore = reg.interner_mut().intern("Masticore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(masticore);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: protection from multicolored is not an expressible keyword.
        keywords: vec![KeywordAbility::FirstStrike],
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
            effect: upkeep_sac_unless_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_sac_unless_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "sacrifice this creature unless you discard a card" — pay = discard a card
    // (penalty avoided); decline = sacrifice (destroy) this creature.
    //
    // GAP: the reflexive "When you discard a card this way, destroy target
    // nonland permanent an opponent controls with mana value ≤ the discarded
    // card's mana value" payoff is omitted — no reflexive when-you-discard
    // sub-trigger and the discarded card's mana value is not readable here.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DestroyPermanent { target: trig.source })),
    }]
}
