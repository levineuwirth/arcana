//! Richlau, Headmaster — `{1}{W}{U}` 2/4 Legendary Human Advisor.
//! At the beginning of your end step, you may pay {1}. When you do,
//! target artifact card in your graveyard perpetually gains "This spell
//! costs {1} less to cast." If it's a creature or Vehicle card, it
//! perpetually gets +2/+2. Put it into your library second from the top.
//!
//! GAP: "perpetually gains" (cost reduction + P/T buff persisting across zones)
//! is not modeled in the engine — there is no Effect::Perpetually variant.
//! GAP: "put it into your library second from the top" — only PutOnTopOfLibrary
//! is available; "second from the top" placement is not expressible.
//! GAP: the "when you do" sub-trigger on a triggered ability is not expressible
//! as a nested optional-payment trigger; approximated with OptionalPayment +
//! a partial effect (PutOnTopOfLibrary for the graveyard target).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Richlau, Headmaster");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .controlled_by(ControllerConstraint::You),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn end_step_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "perpetually gains cost reduction" not modeled.
    // GAP: "perpetually gets +2/+2 if creature or Vehicle" not modeled.
    // GAP: "put it into your library second from the top" — using PutOnTopOfLibrary as approximation.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::PutOnTopOfLibrary { target: *id }),
        else_effect: None,
    }]
}
