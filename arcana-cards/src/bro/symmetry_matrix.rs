//! Symmetry Matrix — `{4}` artifact.
//! "Whenever a creature you control with power equal to its toughness
//! enters, you may pay {1}. If you do, draw a card."
//!
//! The enters trigger is a `ZoneChange` on creatures you control; the
//! "power equal to its toughness" qualifier has no ObjectFilter
//! predicate, so it is checked at resolution via `script::power_of` /
//! `script::toughness_of` on the entering creature. The may-pay gate
//! uses `Effect::OptionalPayment`.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Symmetry Matrix");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger filter — "with power equal to its toughness" has
            // no ObjectFilter predicate; the equality is checked in the
            // effect fn on the entering creature instead.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: may_pay_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn may_pay_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entered = trig.entering_object().unwrap_or(trig.source);
    if script::power_of(state, entered) != script::toughness_of(state, entered) {
        return Vec::new();
    }
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}
