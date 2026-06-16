//! Centaur of Attention — `{3}{G}{G}` 3/3 green Centaur Performer.
//!
//! Oracle:
//! * "When this creature enters, roll five six-sided dice and store
//!   those results on it." — dice-rolling and per-object stored results
//!   are not modeled by the engine. GAP (effect returns `Vec::new()`).
//! * "At the beginning of combat on your turn, you may reroll any number
//!   of this creature's stored results." — same dice/store subsystem;
//!   GAP.
//! * "This creature gets +X/+X, where X is the greatest number of stored
//!   results on it of the same value." — a pure static continuous ability
//!   keyed on the (unmodeled) stored dice results; not expressible. GAP.
//!
//! Bones are emitted faithfully; both triggered abilities are wired with
//! their correct conditions but resolve to no effect, and the static is
//! noted as a GAP comment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Centaur of Attention");
    let centaur = reg.interner_mut().intern("Centaur");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(performer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "This creature gets +X/+X, where X is the greatest number
    // of stored dice results of the same value" — depends on unmodeled
    // stored-dice-results subsystem; not expressible as a continuous effect.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_roll_and_store,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_reroll,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_roll_and_store(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "roll five six-sided dice and store those results on it" — no
    // dice-rolling or per-object stored-results primitive in the engine.
    Vec::new()
}

fn combat_reroll(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reroll any number of this creature's stored results" — same
    // unmodeled dice/stored-results subsystem.
    Vec::new()
}
