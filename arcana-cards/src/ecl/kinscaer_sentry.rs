//! Kinscaer Sentry — `{1}{W}` 2/2 Kithkin Soldier with First strike and Lifelink.
//! "Whenever this creature attacks, you may put a creature card with mana value X or
//!  less from your hand onto the battlefield tapped and attacking, where X is the
//!  number of attacking creatures you control."

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
    let name = reg.interner_mut().intern("Kinscaer Sentry");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_cheat_in,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_cheat_in(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "put a creature card with mana value X or less from your hand onto the
    //      battlefield tapped and attacking, X = number of attacking creatures you
    //      control" — X is a dynamic mana-value cap on the hand filter, but no
    //      script:: helper counts attacking creatures and the filter cap cannot be
    //      materialized at resolution; emitting an unrestricted put would be a
    //      materially wrong (stronger) card, so the whole gated effect is omitted.
    Vec::new()
}
