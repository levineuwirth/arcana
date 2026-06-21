//! Sylvan Shepherd — `{2}{G}` 2/3 Human Druid.
//!
//! Vigilance.
//! Whenever this creature attacks, roll a d20.
//!   1—9  | You gain 1 life.
//!   10—19 | You gain 2 life.
//!   20   | You gain 5 life.
//!
//! Vigilance is expressible. The attack trigger is wired, but its body is
//! GAP'd: there is no die-roll (`d20`) primitive, so the bucketed
//! life-gain outcomes can't be expressed without inventing a roll
//! mechanic.

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
    let name = reg.interner_mut().intern("Sylvan Shepherd");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: roll_d20,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn roll_d20(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "roll a d20" with bucketed outcomes — no die-roll primitive.
    Vec::new()
}
