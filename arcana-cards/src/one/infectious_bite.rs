//! Infectious Bite — `{1}{G}` instant. "Target creature you control
//! deals damage equal to its power to target creature you don't
//! control. Each opponent gets a poison counter."
//!
//! GAP: DealDamageEqualToPower (one creature dealing damage equal to
//! its own power to another creature) — see Master's Rebuke; Fight used
//! as approximation. Also PoisonCounterForEachOpponent (giving each
//! opponent a poison counter) is not in the engine effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infectious Bite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control. Each opponent gets a poison counter.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: DealDamageEqualToPower (one-sided power-based damage)
    // GAP: PoisonCounterForEachOpponent (give each opponent a poison counter)
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(second) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(a) = first else { return Vec::new(); };
    let TargetChoice::Object(b) = second else { return Vec::new(); };
    vec![Effect::Fight { a: *a, b: *b }]
}
