//! Stew the Coneys — `{2}{G}` instant.
//! "Target creature you control deals damage equal to its power to target
//! creature you don't control. Create a Food token."
//!
//! # GAP: Food token — no Food artifact token shape in the catalog (Food
//! has an activated ability, which TokenDefinition.abilities cannot encode).
//! The Fight effect is modelled; the Food token creation is noted as a gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stew the Coneys");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control. Create a Food token.".into(),
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
    // GAP: Food token creation — TokenDefinition cannot encode the Food activated ability
    let mut targets = entry.targets.targets.iter();
    let first = targets.next();
    let second = targets.next();
    match (first, second) {
        (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) => {
            vec![Effect::Fight { a: *a, b: *b }]
        }
        _ => Vec::new(),
    }
}
