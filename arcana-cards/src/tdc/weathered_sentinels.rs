//! Weathered Sentinels — `{3}` 2/5 Artifact Creature — Wall.
//! Defender, vigilance, reach, trample.
//! Can attack players who attacked you last turn as though it didn't have
//! defender. (static — GAP)
//! Whenever this creature attacks, it gets +3/+3 and gains indestructible
//! until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Weathered Sentinels");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Defender,
        ],
        ..Default::default()
    };

    // GAP: static — "can attack players who attacked you during their last
    // turn as though it didn't have defender". No primitive for a
    // conditional defender-ignoring attack permission.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack_pump_indestructible,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack_pump_indestructible(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Indestructible],
    }]
}
