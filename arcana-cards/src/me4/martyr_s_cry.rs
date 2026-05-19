//! Martyr's Cry — `{W}{W}` sorcery. "Exile all white creatures. For each
//! creature exiled this way, its controller draws a card."
//!
//! The exile-all-white sweep uses `script::ids_matching` with a white-creature
//! filter fed into `Effect::ForEach { effect: ExilePermanent }`.
//!
//! GAP: "its controller draws a card" per exiled creature requires applying
//! two effects per target (exile + draw for that permanent's controller) —
//! `ForEach` accepts a single boxed `Effect`, so the per-controller draw
//! rider cannot be expressed and is omitted.

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
    let name = reg.interner_mut().intern("Martyr's Cry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all white creatures. For each creature exiled this way, its controller draws a card.".into(),
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
    let filter = ObjectFilter::creature().with_colors(ColorSet::white());
    let targets = script::ids_matching(state, &filter, entry.controller);
    // GAP: per-exiled-creature controller draw not expressible via ForEach single-effect
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
