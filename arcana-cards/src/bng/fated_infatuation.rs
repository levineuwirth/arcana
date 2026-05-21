//! Fated Infatuation — `{U}{U}{U}` instant. "Create a token that's a
//! copy of target creature you control. If it's your turn, scry 2."
//!
//! GAP: copy-token of a target creature isn't a primitive — emit the
//! conditional scry and document the missing copy.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fated Infatuation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a token that's a copy of target creature you control. If it's your turn, scry 2.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
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
    // GAP: copy-token of a target creature isn't expressible. Also no
    // "if it's your turn" gate available; emit the scry unconditionally.
    vec![Effect::Scry { player: entry.controller, count: 2 }]
}
