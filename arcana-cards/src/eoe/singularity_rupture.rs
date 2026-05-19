//! Singularity Rupture — `{3}{U}{B}{B}` sorcery.
//! Colors: B, U
//! "Destroy all creatures, then any number of target players each mill half their library, rounded down."
//! GAP: targeting 'any number of players' (TargetCount::Any with Player filter) and milling
//! half their library (dynamic amount from library_size/2).

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
    let name = reg.interner_mut().intern("Singularity Rupture");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures, then any number of target players each mill half their library, rounded down.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![
        Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        // GAP: no multi-player targeting for mill-half-library effect
    ]
}
