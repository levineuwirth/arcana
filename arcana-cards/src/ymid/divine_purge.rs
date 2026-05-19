//! Divine Purge — `{1}{W}{W}` sorcery.
//! "Exile all artifacts and creatures with mana value 3 or less. They perpetually gain 'This spell
//! costs {2} more to cast' and 'This permanent enters the battlefield tapped.' For as long as each
//! of them remain exiled, its owner may play it."
//! GAP: perpetual cost modification, ETB-tapped replacement, and play-from-exile permission are
//! not expressible with any catalog Effect variant.
//! Implementing only the mass exile of artifacts and creatures with CMC 3 or less.

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
    let name = reg.interner_mut().intern("Divine Purge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all artifacts and creatures with mana value 3 or less. They perpetually gain 'This spell costs {2} more to cast' and 'This permanent enters the battlefield tapped.' For as long as each of them remain exiled, its owner may play it.".into(),
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
    let filter = ObjectFilter::permanent()
        .with_max_cmc(3)
        .without_types(TypeLine::LAND.into());
    let ids = script::ids_matching(state, &filter, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    // GAP: perpetual cost/ETB modification and play-from-exile permission
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
