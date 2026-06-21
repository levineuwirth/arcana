//! Angel of Destiny — `{3}{W}{W}` 2/6 Creature — Angel Cleric.
//!
//! Flying, double strike
//! * Whenever a creature you control deals combat damage to a player, you
//!   and that player each gain that much life.
//! * At the beginning of your end step, if you have at least 15 life more
//!   than your starting life total, each player this creature attacked this
//!   turn loses the game.
//!
//! GAP: the second ability is fully unexpressible — there is no "starting
//! life total" condition helper for the intervening-if, no per-turn "players
//! this creature attacked" tracking, and no "loses the game" effect; the
//! effect is GAP'd (and the intervening-if omitted).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angel of Destiny");
    let angel = reg.interner_mut().intern("Angel");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: gain_life_both,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "15+ life over starting total" not
                // expressible.
                intervening_if: None,
                effect: end_step_lose_game_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_life_both(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = trig.damage_amount().unwrap_or(0);
    if amount == 0 {
        return Vec::new();
    }
    let Some(them) = trig.damaged_player() else { return Vec::new(); };
    vec![
        Effect::GainLife { player: trig.controller, amount },
        Effect::GainLife { player: them, amount },
    ]
}

fn end_step_lose_game_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no "loses the game" effect; no "players this creature attacked
    // this turn" tracking.
    Vec::new()
}
