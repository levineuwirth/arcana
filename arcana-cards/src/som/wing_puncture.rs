//! Wing Puncture — `{G}` instant.
//! "Target creature you control deals damage equal to its power to target
//! creature with flying."
//!
//! Modelled as a Fight between two targeted creatures; the 'deals damage
//! equal to its power' is exactly the Fight mechanic (each deals damage
//! equal to its power to the other), though strictly this card is one-sided.
//!
//! # GAP: one-sided fight (only the attacker deals damage) — Effect::Fight
//! is mutual; using it as the closest approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wing Puncture");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature with flying.".into(),
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
    // GAP: one-sided fight — Fight is mutual; using as approximation
    let mut iter = entry.targets.targets.iter();
    let first = iter.next();
    let second = iter.next();
    match (first, second) {
        (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) => {
            vec![Effect::Fight { a: *a, b: *b }]
        }
        _ => Vec::new(),
    }
}
