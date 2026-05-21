//! Worldfire — `{6}{R}{R}{R}` sorcery. "Exile all permanents. Exile
//! all cards from all hands and graveyards. Each player's life total
//! becomes 1." Hand/graveyard mass-exile isn't catalog-shaped (script
//! helpers enumerate the battlefield only); emit the permanent
//! exile-all and SetLifeTotal, GAP the hand/graveyard exile.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all permanents. Exile all cards from all hands and graveyards. Each player's life total becomes 1.".into(),
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
    let mut effects = Vec::new();
    for id in script::ids_matching(state, &ObjectFilter::permanent(), entry.controller) {
        effects.push(Effect::ExilePermanent { target: id });
    }
    for p in script::all_players(state) {
        effects.push(Effect::SetLifeTotal { player: p, amount: 1 });
    }
    // GAP: exile all cards from all hands and graveyards (script
    // helpers don't enumerate non-battlefield zones).
    effects
}
