//! Morningtide — `{1}{W}` sorcery. "Exile all graveyards."
//! Gathers all graveyard object IDs across every player, then ForEach
//! ExileFromGraveyard with NULL_OBJECT_ID as the per-iteration placeholder.

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
    let name = reg.interner_mut().intern("Morningtide");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all graveyards.".into(),
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
    let filter = ObjectFilter::default();
    let all_ids: Vec<_> = script::all_players(state)
        .into_iter()
        .flat_map(|p| script::graveyard_ids_matching(state, &filter, p, entry.controller))
        .collect();
    vec![Effect::ForEach {
        targets: all_ids,
        effect: Box::new(Effect::ExileFromGraveyard {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
