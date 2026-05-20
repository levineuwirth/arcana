//! Aether Snap — `{3}{B}{B}` sorcery. "Remove all counters from all
//! permanents and exile all tokens."

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
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
    // Partial: "remove all counters from all permanents" has no
    // catalog effect (RemoveCounters needs a kind + count) and is
    // GAPed. "Exile all tokens" is implemented.
    let tokens = script::ids_matching(
        state,
        &ObjectFilter::permanent().tokens_only(),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: tokens,
        effect: Box::new(Effect::ExilePermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}
