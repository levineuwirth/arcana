//! Vigean Hydropon — `{1}{G}{U}` 0/0 Plant Mutant.
//!
//! Graft 5 (This creature enters with five +1/+1 counters on it. Whenever
//!   another creature enters, you may move a +1/+1 counter from this creature
//!   onto it.)
//! This creature can't attack or block.
//!
//! Graft 5 is a fully-implemented parametrized keyword. The static
//! "can't attack or block" self-restriction is installed as two self-targeted
//! continuous effects (`cant_attack` + `cant_block`) lasting while this
//! creature remains on the battlefield, via a `SelfEntersBattlefield` trigger.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Vigean Hydropon");
    let plant = reg.interner_mut().intern("Plant");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Graft(5)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cant_attack_or_block,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Static "This creature can't attack or block": install self-targeted
/// CantAttack + CantBlock continuous effects lasting while this creature is on
/// the battlefield.
fn install_cant_attack_or_block(
    _s: &GameState,
    trig: &PendingTrigger,
    _r: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::cant_attack(
                trig.source,
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::cant_block(
                trig.source,
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
