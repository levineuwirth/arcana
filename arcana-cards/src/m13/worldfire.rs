//! Worldfire — `{6}{R}{R}{R}` sorcery. "Exile all permanents. Exile
//! all cards from all hands and graveyards. Each player's life total
//! becomes 1."
//!
//! Exile-all-permanents is modeled via ForEach. Exile-all-hands/
//! graveyards is GAP'd (no zone-wide exile Effect). SetLifeTotal=1
//! across all players is modeled.

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
    let name = reg.interner_mut().intern("Worldfire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile all permanents. Exile all cards from all hands and graveyards. Each player's life total becomes 1.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::permanent(), entry.controller);
    // GAP: zone-wide exile of all hands and all graveyards has no catalog Effect.
    let mut effects = vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }];
    for p in script::all_players(state) {
        effects.push(Effect::SetLifeTotal { player: p, amount: 1 });
    }
    effects
}
