//! Ulamog's Crusher — `{8}` 8/8 Eldrazi.
//! Annihilator 2 (Whenever this creature attacks, defending player sacrifices
//! two permanents of their choice.)
//! This creature attacks each combat if able.
//!
//! Annihilator is not in the usable keyword surface, but its reminder text is a
//! plain attack trigger, so it is wired as a `SelfAttacks` trigger that makes
//! the defending player sacrifice two permanents. "Attacks each combat if able"
//! is a SELF must-attack requirement (CR 508.1a), installed on ETB.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulamog's Crusher");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        // GAP: Annihilator is not in the usable keyword surface; its reminder
        // text is wired below as a triggered ability instead.
        keywords: vec![],
        ..Default::default()
    };

    // "Attacks each combat if able" — SELF must-attack, installed on ETB.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: annihilator_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_must_attack(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn annihilator_two(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Sacrifice {
        player: p,
        filter: ObjectFilter::permanent(),
        count: 2,
    }]
}
