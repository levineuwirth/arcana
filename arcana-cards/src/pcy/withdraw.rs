//! Withdraw — `{U}{U}` instant. "Return target creature to its
//! owner's hand. Then return another target creature to its owner's
//! hand unless its controller pays {1}."
//!
//! First creature target is bounced. The second target's "unless its
//! controller pays {1}" rider has no catalog representation for a
//! bounce (CounterUnlessPays applies only to spells on the stack).
//!
//! GAP: "return ... unless its controller pays {1}" optional-tax
//! bounce not expressible; only the first creature is returned.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature to its owner's hand. Then return another target creature to its owner's hand unless its controller pays {1}.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: second "return unless controller pays {1}" bounce-tax not expressible.
    vec![Effect::ReturnToHand { target: *id }]
}
