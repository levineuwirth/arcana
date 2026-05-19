//! Consign to Dream — `{2}{U}` instant. "Return target permanent to its owner's hand unless its
//! controller pays {3}. If they don't, put that permanent on top of its owner's library instead."
//!
//! # GAP
//! - No support for opponent-pays cost to redirect effect
//! - No support for conditional routing between ReturnToHand and PutOnTopOfLibrary based on
//!   whether the target's controller pays a cost

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Consign to Dream");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target permanent to its owner's hand unless its controller pays {3}. If they don't, put that permanent on top of its owner's library instead.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Permanent(
                        arcana_core::targets::ObjectFilter::new(),
                    ),
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: conditional routing based on whether target controller pays {3}
    vec![Effect::ReturnToHand { target: *id }]
}
