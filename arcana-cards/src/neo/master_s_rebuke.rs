//! Master's Rebuke — `{1}{G}` instant. "Target creature you control deals
//! damage equal to its power to target creature or planeswalker you
//! don't control."
//!
//! GAP: DealDamageEqualToPower (one creature dealing damage equal to
//! its own power to another object, reading the source creature's power
//! dynamically) is not a single Effect variant. The closest available
//! primitive is Effect::Fight, but Fight is symmetric (both deal
//! damage); a one-sided power-damage effect is not in the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master's Rebuke");
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
                text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control.".into(),
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
    // GAP: DealDamageEqualToPower (one-sided deal-damage-equal-to-power effect)
    // Using Fight as best approximation (symmetric)
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(second) = entry.targets.targets.get(1) else { return Vec::new(); };
    use arcana_core::targets::TargetChoice;
    let TargetChoice::Object(a) = first else { return Vec::new(); };
    let TargetChoice::Object(b) = second else { return Vec::new(); };
    vec![Effect::Fight { a: *a, b: *b }]
}
