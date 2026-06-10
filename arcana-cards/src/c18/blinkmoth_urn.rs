//! Blinkmoth Urn — `{5}` artifact (Mirrodin, 2003).
//! "At the beginning of each player's first main phase, if this
//! artifact is untapped, that player adds {C} for each artifact they
//! control." Modeled as two triggers (one for your precombat main,
//! one for each opponent's) so "that player" is identified in the
//! two-player engine.
//!
//! GAP: the intervening-if 'if this artifact is untapped' has no
//! source-untapped condition helper — the trigger fires
//! unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blinkmoth Urn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if 'if this artifact is untapped' —
                // no source-untapped condition helper.
                intervening_if: None,
                effect: your_main_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if 'if this artifact is untapped' —
                // no source-untapped condition helper.
                intervening_if: None,
                effect: opponent_main_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn your_main_mana(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, trig.controller);
    vec![Effect::AddMana {
        player: trig.controller,
        mana: (0..n)
            .map(|_| ManaUnit::plain(ManaColor::Colorless, trig.source))
            .collect(),
    }]
}

fn opponent_main_mana(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(&opp) = script::opponents(state, trig.controller).first() else {
        return Vec::new();
    };
    let filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, opp);
    vec![Effect::AddMana {
        player: opp,
        mana: (0..n)
            .map(|_| ManaUnit::plain(ManaColor::Colorless, trig.source))
            .collect(),
    }]
}
