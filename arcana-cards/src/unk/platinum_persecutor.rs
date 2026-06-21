//! Platinum Persecutor — `{5}` 5/5 Artifact Creature — Angel Demon with Flying.
//!
//! Oracle:
//! * Flying
//! * Players can't lose the game or win the game. (static — GAP)
//! * At the beginning of your upkeep, if each player has an empty library,
//!   sacrifice Platinum Persecutor and you win the game.
//!
//! Flying is a base keyword. The "players can't lose/win" static has no
//! expressible primitive and is GAP'd. The upkeep trigger's "you win the game"
//! payoff has no expressible win-the-game effect, and the "each player has an
//! empty library" intervening-if has no expressible all-players-library-empty
//! predicate; the whole trigger effect is therefore GAP'd rather than emitted
//! as a partial (sacrifice-only) which would be a materially wrong card.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Platinum Persecutor");
    let angel = reg.interner_mut().intern("Angel");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "Players can't lose the game or win the game."
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if — "if each player has an empty library"
                // has no expressible all-players-library-empty predicate.
                intervening_if: None,
                effect: win_if_decked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn win_if_decked(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice Platinum Persecutor and you win the game." There is no
    // expressible win-the-game effect; emitting only the sacrifice would be a
    // materially wrong card, so the whole effect is GAP'd.
    Vec::new()
}
