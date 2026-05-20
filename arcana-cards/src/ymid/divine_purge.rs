//! Divine Purge — `{1}{W}{W}` sorcery. "Exile all artifacts and creatures
//! with mana value 3 or less. They perpetually gain ... For as long as each
//! of them remain exiled, its owner may play it."
//!
//! GAP: no support for perpetual riders, no 'owner may play exiled cards'
//! mechanic. Best effort: exile all matching permanents.

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
                text: "Exile all artifacts and creatures with mana value 3 or less.".into(),
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
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().with_max_cmc(3),
        entry.controller,
    );
    let artifacts = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .with_max_cmc(3),
        entry.controller,
    );
    let mut ids = creatures;
    ids.extend(artifacts);
    // GAP: perpetual riders and 'owner may play exiled cards' not supported
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
