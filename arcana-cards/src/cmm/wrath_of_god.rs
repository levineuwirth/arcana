//! Wrath of God — `{2}{W}{W}` sorcery. "Destroy all creatures. They can't
//! be regenerated."
//!
//! # GAP: "can't be regenerated" — no Effect variant to suppress regeneration.
//! Best effort: destroy all creatures; regeneration prevention omitted.

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
    let name = reg.interner_mut().intern("Wrath of God");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. They can't be regenerated.".into(),
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
    // GAP: "can't be regenerated" — no regeneration-suppression Effect variant
    let targets = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
