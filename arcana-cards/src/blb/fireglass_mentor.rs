//! Fireglass Mentor — `{B}{R}` 2/1 Lizard Warlock.
//! "At the beginning of your second main phase, if an opponent lost life
//!  this turn, exile the top two cards of your library. Choose one of
//!  them. Until end of turn, you may play that card."
//!
//! Trigger fires at the post-combat (second) main phase, gated by an
//! intervening-if (an opponent lost life this turn). The payoff is modeled
//! with ImpulseExile of two cards (the "choose one of them to play" — vs
//! play either — is a documented fidelity gap: ImpulseExile enables
//! playing all exiled cards this turn rather than restricting to one).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fireglass Mentor");
    let lizard = reg.interner_mut().intern("Lizard");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PostCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_opponent_lost_life),
                effect: impulse_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_opponent_lost_life(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::an_opponent_lost_life_this_turn(s, you)
}

fn impulse_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile { player: trig.controller, count: 2 }]
}
