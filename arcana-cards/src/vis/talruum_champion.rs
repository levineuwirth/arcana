//! Talruum Champion — `{4}{R}` 3/3 red Minotaur with First strike.
//!
//! * First strike — keyword.
//! * "Whenever this creature blocks or becomes blocked by a creature, that
//!   creature loses first strike until end of turn." — the
//!   SelfBlocksOrBecomesBlocked trigger is faithful, but:
//!     - GAP (effect): "loses first strike" has no dedicated keyword-removal
//!       effect; `LoseAllAbilities` would strip every ability (too broad),
//!       so the effect is omitted rather than modeled incorrectly.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talruum Champion");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
            intervening_if: None,
            effect: strip_first_strike,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn strip_first_strike(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that creature loses first strike until end of turn" — no
    // single-keyword removal effect (see module doc).
    Vec::new()
}
