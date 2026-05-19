//! Heartless Conscription — `{6}{B}{B}` sorcery. "Exile all creatures.
//! For each card exiled this way, you may play that card for as long as
//! it remains exiled, and mana of any type can be spent to cast that
//! spell. Exile Heartless Conscription."
//!
//! GAP: play-from-exile grants for each exiled creature, any-mana
//! casting permission, and self-exile are not expressible. Best-effort:
//! exile all creatures.

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
    let name = reg.interner_mut().intern("Heartless Conscription");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all creatures. For each card exiled this way, you may play that card for as long as it remains exiled, and mana of any type can be spent to cast that spell. Exile Heartless Conscription.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: play-from-exile grants per exiled card
    // GAP: self-exile
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
