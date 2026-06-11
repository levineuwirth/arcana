//! Compost — `{1}{G}` enchantment.
//! "Whenever a black card is put into an opponent's graveyard from
//! anywhere, you may draw a card."
//!
//! A graveyard-bound `ZoneChange` trigger filtered to black cards on the
//! opponent side. GAP: the cost-free "you may" gate is not expressible
//! (`OptionalPaymentKind` is Mana / Life only) — the draw is emitted
//! unconditionally, a strictly-beneficial approximation.

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
    let name = reg.interner_mut().intern("Compost");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_colors(ColorSet::black())
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: may_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn may_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the cost-free "you may draw a card" choice is not expressible
    // (OptionalPaymentKind is Mana/Life only); the draw is unconditional.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
