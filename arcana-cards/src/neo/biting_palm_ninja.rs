//! Biting-Palm Ninja — `{2}{B}` 3/3 Human Ninja.
//!
//! Oracle:
//! * Ninjutsu {2}{B} (GAP — not a usable KeywordAbility variant).
//! * This creature enters with a menace counter on it.
//! * Whenever this creature deals combat damage to a player, you may remove a
//!   menace counter from it. When you do, that player reveals their hand and you
//!   choose a nonland card from it. Exile that card.
//!
//! The enters-with-a-counter clause is modeled as an ETB trigger adding a named
//! "menace" counter to itself. The combat-damage reflexive (the "when you do"
//! sub-trigger gated on removing the counter, then hand-reveal + nonland exile)
//! is not expressible with the demonstrated API — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biting-Palm Ninja");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let _menace = reg.interner_mut().intern("menace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Ninjutsu is not in the supported KeywordAbility surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "This creature enters with a menace counter on it."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_menace_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever this creature deals combat damage to a player, you may
            // remove a menace counter from it. When you do, ... exile a nonland."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::new(),
                    target_filter: arcana_core::targets::TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_reflexive,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_menace_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = reg
        .interner()
        .lookup("menace")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::AddCounters { target: trig.source, kind, count: 1 }]
}

fn combat_damage_reflexive(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may remove a menace counter from it. When you do, that player
    // reveals their hand and you choose a nonland card from it. Exile that card."
    // The reflexive "when you do" sub-trigger gated on an optional counter
    // removal, plus the opponent's-hand reveal + chooser-selected nonland exile,
    // is not expressible with the demonstrated API.
    Vec::new()
}
