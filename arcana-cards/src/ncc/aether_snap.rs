//! Aether Snap — `{3}{B}{B}` sorcery. "Remove all counters from all permanents
//! and exile all tokens."
//!
//! GAP: 'Remove all counters from all permanents' (no counter-kind-agnostic
//! mass remove-counters effect) and 'exile all tokens' are not expressible
//! with the current Effect catalog. ForEach with ExilePermanent covers tokens
//! if we can filter tokens_only(), but RemoveCounters requires a specific
//! CounterKind.

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
    let name = reg.interner_mut().intern("Aether Snap");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Remove all counters from all permanents and exile all tokens.".into(),
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
    let token_ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().tokens_only(),
        entry.controller,
    );
    // GAP: remove all counters from all permanents (no counter-agnostic mass RemoveCounters)
    vec![Effect::ForEach {
        targets: token_ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
