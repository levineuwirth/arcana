//! Cut the Tethers — `{2}{U}{U}` sorcery.
//! "For each Spirit, return it to its owner's hand unless that player pays {3}."
//!
//! # GAP: subtype-filtered 'for each' with per-permanent pay-or-bounce choice.
//! Best effort: return all Spirits to their owners' hands via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cut the Tethers");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each Spirit, return it to its owner's hand unless that player pays {3}.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: pay-{3}-or-bounce choice per permanent; best effort returns all Spirits
    let filter = script::subtype_filter(reg, "Spirit");
    let targets = script::ids_matching(state, &filter, entry.controller);
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
