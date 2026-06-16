//! Wicker Warcrawler — `{5}` 6/6 Artifact Creature — Scarecrow.
//! "Whenever this creature attacks or blocks, put a -1/-1 counter on it
//! at end of combat."
//!
//! GAP: trigger fires on BOTH attack and block; there is no single
//! TriggerCondition combining both. Using SelfAttacks as the primary;
//! a second TriggeredAbilityDef for SelfBlocks would be needed for full
//! coverage. Emitting two separate triggered ability defs.
//!
//! Also GAP: "at end of combat" timing — TriggeredAbilityDef fires on
//! the event but the counter lands immediately. Using EndCombat step
//! trigger as closest approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wicker Warcrawler");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_or_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: on_attack_or_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack_or_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
