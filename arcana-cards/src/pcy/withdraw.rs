//! Withdraw — `{U}{U}` instant. "Return target creature to its owner's hand.
//! Then return another target creature to its owner's hand unless its
//! controller pays {1}."
//!
//! The first bounce is unconditional; the second has a payment escape.
//! # GAP: "unless controller pays {1}" conditional bounce has no Effect
//! variant. Best-effort: bounce first target unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Withdraw");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature to its owner's hand. Then return another target \
                       creature to its owner's hand unless its controller pays {1}."
                    .into(),
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
    // GAP: second bounce conditional on "unless controller pays {1}" not supported
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnToHand { target: *id });
        }
    }
    effects
}
