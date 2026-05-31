//! Paradoxical Outcome — `{3}{U}` instant. "Return any number of
//! target nonland, nontoken permanents you control to their owners'
//! hands. Draw a card for each card returned to your hand this way."
//!
//! Modeled as a variadic-target spell: `TargetCount::Any` over a
//! permanent filter restricted to nonland, nontoken permanents you
//! control. The resolver bounces every chosen target and draws one
//! card per returned target.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paradoxical Outcome");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return any number of target nonland, nontoken permanents you \
                   control to their owners' hands. Draw a card for each card \
                   returned to your hand this way."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You)
                        .without_types(TypeLine::LAND.into())
                        .nontoken(),
                ),
                count: TargetCount::Any,
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
    let mut effects = Vec::new();
    let mut returned = 0u32;
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnToHand { target: *id });
            returned += 1;
        }
    }
    if returned > 0 {
        effects.push(Effect::DrawCards {
            player: entry.controller,
            count: returned,
        });
    }
    effects
}
