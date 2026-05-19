//! Righteous Fury — `{4}{W}{W}` sorcery. "Destroy all tapped creatures. You gain 2 life
//! for each creature destroyed this way."
//!
//! GAP: ObjectFilter does not support filtering by tapped state. Emitting board wipe of all
//! creatures and fixed life gain as best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Righteous Fury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all tapped creatures. You gain 2 life for each creature destroyed this way.".into(),
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
    // GAP: ObjectFilter cannot filter by tapped state; destroying all creatures as best-effort.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let count = ids.len() as u32;
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        Effect::GainLife { player: entry.controller, amount: count * 2 },
    ]
}
