//! Peacekeeper — `{2}{W}` 1/1 white Human.
//!
//! Oracle:
//! * "At the beginning of your upkeep, sacrifice this creature unless you
//!   pay {1}{W}." — a `StepBegins`(Upkeep / You) trigger modeled as an
//!   inverted-polarity `Effect::OptionalPayment` ("Z unless you pay X"):
//!   the engine prompts the controller to pay `{1}{W}`; on no-pay the
//!   punishment in `else_effect` fires.
//! * "Creatures can't attack." — a PURE STATIC continuous ability with no
//!   trigger word and no cost. Not expressible as a triggered/activated
//!   ability on this card class; GAP'd in `register` (see below).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peacekeeper");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: static "Creatures can't attack." — no static-continuous
            // ability mechanism in this card class; omitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_pay_or_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Sacrifice this creature unless you pay {1}{W}." Inverted-polarity
/// optional payment: the engine prompts the controller; on no-pay the
/// `else_effect` punishment fires.
fn upkeep_pay_or_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{W}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        // GAP-partial: sacrifice should be THIS creature specifically; no
        // source-targeted sacrifice Effect exists, so this uses a
        // controller-chosen creature sacrifice as the closest available
        // primitive.
        else_effect: Some(Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            count: 1,
        })),
    }]
}
