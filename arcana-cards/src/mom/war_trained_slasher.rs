//! War-Trained Slasher — `{3}{R}` 4/3 Creature — Wolverine Dinosaur.
//! Menace.
//! Whenever this creature attacks a battle, double its power until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("War-Trained Slasher");
    let wolverine = reg.interner_mut().intern("Wolverine");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolverine);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "attacks a battle" specifically — no trigger condition
            // distinguishes attacking a battle from attacking a player/PW;
            // approximated as SelfAttacks (may over-fire when attacking a
            // player or planeswalker).
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: double_power,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn double_power(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Double its power" = add its current power to itself until end of turn.
    let p = script::power_of(state, trig.source).max(0);
    vec![Effect::Pump {
        target: trig.source,
        power: p,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
