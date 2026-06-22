//! Syx, Robot Overlord — `{U}{R}` 2/2 Legendary Artifact Creature — Robot Rogue.
//!
//! * "Before this game begins, if this is your commander, secretly
//!   write down the name of five nonartifact creature cards in your deck
//!   to be invaders. Reveal them after the game." — a pregame
//!   hidden-information setup with no engine analogue. GAP'd entirely.
//! * "At the beginning of your upkeep, target opponent may guess if a
//!   creature you control is an invader. If they guess right, exile that
//!   creature. If they guess wrong, they lose 3 life. Then you may
//!   reveal the name of an invader you control. If you do, put two
//!   +1/+1 counters on it. It becomes a Robot artifact creature." — a
//!   multi-step guess/reveal interaction depending on the unmodeled
//!   "invader" designation. GAP'd: the upkeep trigger shell is emitted
//!   with an empty effect body.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syx, Robot Overlord");
    let robot = reg.interner_mut().intern("Robot");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: pregame "invader" designation is a hidden-info setup with no
    // engine analogue.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_guess,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_guess(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: opponent-guesses-an-invader interaction depends on the
    // unmodeled "invader" designation and a multi-branch reveal; no
    // primitive expresses it.
    Vec::new()
}
