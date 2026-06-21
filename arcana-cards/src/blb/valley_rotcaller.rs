//! Valley Rotcaller — `{1}{B}` 1/3 Squirrel Warlock with Menace.
//!
//! Oracle:
//! * Menace.
//! * Whenever this creature attacks, each opponent loses X life and you gain X
//!   life, where X is the number of other Squirrels, Bats, Lizards, and Rats
//!   you control.
//!
//! Menace is a base characteristic. The attack trigger computes X via a
//! subtype-OR count and drains each opponent / gains you that much.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valley Rotcaller");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warlock);
    // Pre-intern the counted subtypes so lookup succeeds at resolution time.
    reg.interner_mut().intern("Bat");
    reg.interner_mut().intern("Lizard");
    reg.interner_mut().intern("Rat");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever this creature attacks, each opponent loses X life and you
            // gain X life, where X = other Squirrels/Bats/Lizards/Rats you
            // control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: drain_per_tribe,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn drain_per_tribe(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let syms: Vec<_> = ["Squirrel", "Bat", "Lizard", "Rat"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(syms);
    let total = script::count_matching(state, &filter, trig.controller);
    // "other" — this creature is itself a Squirrel and is counted above.
    let x = total.saturating_sub(1);
    if x == 0 {
        return Vec::new();
    }
    let mut effects = vec![Effect::GainLife {
        player: trig.controller,
        amount: x,
    }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife {
            player: opp,
            amount: x,
        });
    }
    vec![Effect::Sequence(effects)]
}
