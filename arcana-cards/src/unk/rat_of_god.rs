//! Rat of God — `{3}{B}{B}` Kindred Sorcery. "Destroy all non-Rat
//! creatures. Each player amasses Rats X, where X is the number of
//! creatures that died this way."
//!
//! Note: Kindred type is not separately modelled; type line is Sorcery only.
//!
//! # GAP
//! 1. `.without_subtype("Rat")` is not in the ObjectFilter API, so the
//!    non-Rat filter cannot be expressed; best-effort destroys all creatures.
//! 2. Amass mechanic (put +1/+1 counters on an Army, create one if absent)
//!    is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rat of God");
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
                text: "Destroy all non-Rat creatures. Each player amasses Rats X, where X is the number of creatures that died this way.".into(),
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
    // GAP: .without_subtype("Rat") not in ObjectFilter API; destroys all creatures
    // GAP: Amass mechanic not expressible
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
