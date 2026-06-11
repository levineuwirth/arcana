//! Pedantic Learning — `{U}{U}` enchantment (Odyssey, 2001).
//! "Whenever a land card is put into your graveyard from your library, you
//! may pay {1}. If you do, draw a card."
//!
//! A graveyard-bound `ZoneChange` trigger over land cards; the may-pay gate
//! is `Effect::OptionalPayment`. GAP: the FROM zone should be "your
//! library", but Zone::Library is not part of this catalog (only
//! Battlefield / Graveyard / Hand are demonstrated), so `from: None`
//! over-fires for land cards reaching the graveyard from anywhere.

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
    let name = reg.interner_mut().intern("Pedantic Learning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "from your LIBRARY"; Zone::Library is not in
                // this catalog, so from: None (any zone) is the closest. The
                // "your graveyard" half is approximated by the you-control
                // filter constraint.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: pay_to_learn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {1}. If you do, draw a card."
fn pay_to_learn(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        }),
        else_effect: None,
    }]
}
