//! Foster — `{2}{G}{G}` enchantment.
//! "Whenever a creature you control dies, you may pay {1}. If you do,
//! reveal cards from the top of your library until you reveal a creature
//! card. Put that card into your hand and the rest into your graveyard."
//!
//! A graveyard-bound `ZoneChange` trigger on creatures you control; the
//! optional payment wraps an `Effect::RevealUntil` (found card to hand,
//! rest to graveyard).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{DigRest, Effect, RevealDest};
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
    let name = reg.interner_mut().intern("Foster");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: pay_and_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {1}. If you do, reveal cards from the top of your
/// library until you reveal a creature card. Put that card into your
/// hand and the rest into your graveyard."
fn pay_and_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::RevealUntil {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            found_dest: RevealDest::Hand,
            rest: DigRest::Graveyard,
            max_reveal: None,
        }),
        else_effect: None,
    }]
}
