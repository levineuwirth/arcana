//! Iridian Maelstrom — `{W}{U}{B}{R}{G}` sorcery, "Destroy each creature
//! that isn't all colors."
//!
//! A creature isn't all colors iff it lacks at least one of the five colors.
//! We collect ids from five `ids_matching` passes — one per
//! `without_colors(C)` — deduplicate via HashSet, and apply `ForEach`
//! `DestroyPermanent`.

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
    let name = reg.interner_mut().intern("Iridian Maelstrom");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy each creature that isn't all colors.".into(),
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
    // "isn't all colors" = lacks at least one of {W, U, B, R, G}.
    // Union ids from each without_colors pass; dedup so each id is
    // destroyed at most once.
    let mut seen = std::collections::HashSet::new();
    let mut targets = Vec::new();
    let filters = [
        ObjectFilter::creature().without_colors(ColorSet::white()),
        ObjectFilter::creature().without_colors(ColorSet::blue()),
        ObjectFilter::creature().without_colors(ColorSet::black()),
        ObjectFilter::creature().without_colors(ColorSet::red()),
        ObjectFilter::creature().without_colors(ColorSet::green()),
    ];
    for filter in &filters {
        for id in script::ids_matching(state, filter, entry.controller) {
            if seen.insert(id) {
                targets.push(id);
            }
        }
    }
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
