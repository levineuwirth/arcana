//! Aether Snap — `{3}{B}{B}` sorcery. "Remove all counters from all
//! permanents and exile all tokens." Bulk counter removal (only the
//! single-permanent RemoveCounters is in catalog) — we emit the
//! token-exile part and GAP the counter clear.

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
    // GAP: 'remove ALL counters from ALL permanents' — RemoveCounters
    // is single-target and counter-kind-specific.
    let token_ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().tokens_only(),
        entry.controller,
    );
    token_ids.into_iter().map(|id| Effect::ExilePermanent { target: id }).collect()
}
