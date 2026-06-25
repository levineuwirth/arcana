//! Emberwilde Caliph — `{2}{U}{R}` 4/4 Creature — Djinn. R/U.
//! Flying, trample.
//! "This creature attacks each combat if able." — a SELF must-attack
//! requirement (CR 508.1a), installed on ETB.
//! "Whenever this creature deals damage, you lose that much life." —
//! DamageDealt (any target, all damage) → controller loses that much life.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emberwilde Caliph");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "This creature attacks each combat if able." — install a SELF
            // must-attack requirement on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (fidelity): source_filter approximates "this creature" as
            // "a creature you control" — no self-only DamageDealt filter.
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::AnyTarget,
                combat_only: false,
            },
            intervening_if: None,
            effect: lose_that_much_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn lose_that_much_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    vec![Effect::LoseLife { player: trig.controller, amount: n }]
}
