//! Sol'Kanar the Tainted — `{2}{U}{B}{R}` 5/5 Legendary Elemental Demon.
//!
//! * "At the beginning of your end step, choose one that hasn't been
//!   chosen — [draw a card] / [each opponent loses 2, you gain 2] /
//!   [deal 3 to up to one other target creature or planeswalker] /
//!   [exile Sol'Kanar, return it under an opponent's control]."
//!   A triggered ability has no modal-dispatch surface (modes are only
//!   wired for spell abilities), and "that hasn't been chosen" requires
//!   per-game mode tracking with no primitive. The end-step trigger is
//!   recorded but its modal effect is GAP'd (returns no effects).

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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sol'Kanar the Tainted");
    let elemental = reg.interner_mut().intern("Elemental");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
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
                effect: end_step_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one that hasn't been chosen" on a triggered
    // ability — no modal-dispatch on triggers and no per-game
    // mode-exclusion tracking primitive.
    Vec::new()
}
