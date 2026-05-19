//! Whelming Wave — `{2}{U}{U}` sorcery. "Return all creatures to their owners'
//! hands except for Krakens, Leviathans, Octopuses, and Serpents."
//!
//! # GAP: Multi-subtype exclusion ("except Krakens, Leviathans, Octopuses, and
//! Serpents") — `ObjectFilter` supports subtype_filter for one subtype at a
//! time but no OR-combination of multiple excluded subtypes. Best effort:
//! return all creatures (ignores the tribal exception).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Whelming Wave");
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
                text: "Return all creatures to their owners' hands except for Krakens, Leviathans, Octopuses, and Serpents.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: multi-subtype exclusion (Kraken/Leviathan/Octopus/Serpent) not expressible
    let targets = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
