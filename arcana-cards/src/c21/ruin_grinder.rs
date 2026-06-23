//! Ruin Grinder — `{5}{R}` 7/4 Artifact Creature — Construct.
//!
//! Oracle:
//! * Menace
//! * When this creature dies, each player may discard their hand and draw
//!   seven cards.
//! * Mountaincycling {2}  (a typecycling variant — emitted as generic
//!   Cycling {2} per the keyword conventions; the basic-type search is not
//!   separately modeled.)
//!
//! The dies trigger discards each player's whole hand (count = current hand
//! size) then draws seven, per player. The optional "may" is a resolution-time
//! choice that is not gateable here, so the discard/draw is applied to each
//! player (fidelity: the per-player "may" opt-out is unmodeled).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruin Grinder");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Menace,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: each_player_wheels,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
    ))
}

fn each_player_wheels(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        let hand = script::hand_size(state, p);
        if hand > 0 {
            effects.push(Effect::Discard {
                player: p,
                count: hand,
                choice: DiscardChoice::ControllerChooses,
            });
        }
        effects.push(Effect::DrawCards {
            player: p,
            count: 7,
        });
    }
    vec![Effect::Sequence(effects)]
}
