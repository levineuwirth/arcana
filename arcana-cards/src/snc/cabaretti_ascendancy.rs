//! Cabaretti Ascendancy — `{R}{G}{W}` enchantment (Streets of New
//! Capenna, 2022). "At the beginning of your upkeep, look at the top
//! card of your library. If it's a creature or planeswalker card, you
//! may reveal it and put it into your hand. If you don't put the card
//! into your hand, you may put it on the bottom of your library."
//!
//! Wired as `Effect::DigTopN` (count 1, creature-or-planeswalker
//! filter, rest to bottom). Fidelity note: the printed text lets a
//! non-taken card optionally STAY on top; DigTopN always bottoms the
//! rest.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cabaretti Ascendancy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: peek_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…look at the top card of your library. If it's a creature or
/// planeswalker card, you may reveal it and put it into your hand…"
/// (non-taken card is bottomed — a documented fidelity gap vs the
/// optional stay-on-top).
fn peek_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(ObjectFilter::new().with_types_any(TypeLine(
            TypeLine::CREATURE | TypeLine::PLANESWALKER,
        ))),
        rest: DigRest::BottomRandom,
    }]
}
