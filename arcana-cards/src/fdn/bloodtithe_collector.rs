//! Bloodtithe Collector — `{4}{B}` 3/4 Creature — Vampire Noble.
//!
//! Oracle:
//! * Flying.
//! * When this creature enters, if an opponent lost life this turn, each
//!   opponent discards a card.
//!
//! Decomposition: Flying keyword + one ETB trigger gated by an intervening-if
//! ("if an opponent lost life this turn"); the effect makes each opponent
//! discard a card.

use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodtithe Collector");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(if_opponent_lost_life),
            effect: each_opponent_discards,
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

fn each_opponent_discards(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
