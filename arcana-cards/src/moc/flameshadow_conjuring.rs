//! Flameshadow Conjuring — `{3}{R}` enchantment.
//! "Whenever a nontoken creature you control enters, you may pay {R}.
//! If you do, create a token that's a copy of that creature. That token
//! gains haste. Exile it at the beginning of the next end step."
//!
//! GAP (fidelity): the freshly minted token's id is unknown at
//! resolution, so the haste grant and the delayed exile riders cannot
//! be attached — the pay-{R}-for-a-copy core is faithful.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flameshadow Conjuring");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: pay_for_shadow_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…you may pay {R}. If you do, create a token that's a copy of that
/// creature. That token gains haste. Exile it at the beginning of the
/// next end step."
fn pay_for_shadow_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    // GAP: the token copy's new id is not observable, so "that token
    // gains haste" and the delayed exile at the next end step are not
    // expressible.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}").expect("valid cost")),
        then: Box::new(Effect::CopyPermanent { target: entered }),
        else_effect: None,
    }]
}
