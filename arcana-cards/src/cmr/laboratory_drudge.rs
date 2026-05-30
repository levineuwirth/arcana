//! Laboratory Drudge — `{3}{U}` 3/4 blue Zombie Horror. "At the beginning
//! of each end step, draw a card if you've cast a spell from a graveyard
//! or activated an ability of a card in a graveyard this turn."
//!
//! GAP: The conditional draw ("if you've cast from a graveyard or activated
//! a graveyard ability this turn") has no matching per-turn event counter in
//! the script API. The trigger fires every end step but the draw is only
//! produced unconditionally (wrong) or omitted; we emit the unconditional
//! draw as the closest approximation and flag the condition as a GAP.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Laboratory Drudge");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                // GAP: should only draw if you cast a spell from a graveyard or
                // activated an ability of a graveyard card this turn; no
                // per-turn counter tracks "cast from graveyard" events in the
                // current script API, so we emit the unconditional draw as a
                // best-effort approximation.
                effect: end_step_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: unconditional draw; should be conditional on having cast/activated
    // from a graveyard this turn.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
