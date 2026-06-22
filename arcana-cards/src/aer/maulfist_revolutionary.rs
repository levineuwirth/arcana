//! Maulfist Revolutionary — `{1}{G}{G}` 3/3 Human Warrior (green).
//! Trample.
//! "When this creature enters or dies, for each kind of counter on target
//! permanent or player, give that permanent or player another counter of that
//! kind."
//!
//! The "enters or dies" clause is decomposed into two TriggeredAbilityDefs
//! (SelfEntersBattlefield + SelfDies), each targeting any permanent or player.
//! The PAYOFF — "for each kind of counter currently on the target, add another
//! of that kind" — is GAP'd: there is no primitive that enumerates the kinds of
//! counters already present on an arbitrary object/player and adds one of each.
//! (`Effect::Proliferate` is board-wide, not single-target, so it is not a
//! faithful substitute.) The targets are declared so the trigger shape is
//! preserved.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maulfist Revolutionary");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let target = || TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::permanent()),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: double_each_counter_kind,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: double_each_counter_kind,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target()],
            }),
    )
}

fn double_each_counter_kind(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each kind of counter on target permanent or player, give that
    // permanent or player another counter of that kind." No primitive
    // enumerates the kinds of counters already on an arbitrary single
    // object/player and adds one more of each (Proliferate is board-wide, not
    // single-target).
    Vec::new()
}
