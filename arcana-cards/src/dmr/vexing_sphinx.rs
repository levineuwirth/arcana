//! Vexing Sphinx — `{1}{U}{U}` 4/4 Creature — Sphinx. Flying.
//! "Cumulative upkeep—Discard a card." "When this creature dies, draw a card
//! for each age counter on it."
//!
//! Flying is a base keyword. Cumulative upkeep is NOT an available keyword
//! (no parametrized/keyword surface for it) and its upkeep mechanic — putting
//! an age counter and the discard-or-sacrifice gate — has no expressible
//! primitive, so it is GAP'd. The death trigger reads the age counters the
//! cumulative-upkeep mechanic would have placed.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Vexing Sphinx");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    // Pre-intern the "age" counter name so the death trigger can recover it.
    let _age = reg.interner_mut().intern("age");

    // GAP: Cumulative upkeep—Discard a card. No keyword/primitive expresses the
    // "put an age counter, then sacrifice unless you pay the upkeep cost for
    // each counter" beginning-of-upkeep mechanic.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_draw_per_age,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_draw_per_age(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let Some(age) = reg.interner().lookup("age").map(CounterKind::Named) else {
        return Vec::new();
    };
    let n = state.objects.get(id).map_or(0, |o| o.count_counters(age));
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: trig.controller, count: n }]
}
