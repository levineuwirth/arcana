//! Depopulate — `{2}{W}{W}` sorcery. Each player who controls a
//! multicolored creature draws a card; then destroy all creatures.

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
    let name = reg.interner_mut().intern("Depopulate");
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
                text: "Each player who controls a multicolored creature draws a card. Then destroy all creatures.".into(),
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
    // GAP: 'multicolored' filter and 'each player who controls one' —
    // ObjectFilter cannot test multi-color cardinality. Default: every
    // player just draws.
    let mut effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect();
    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    effects.push(Effect::ForEach {
        targets: creature_ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    });
    effects
}
