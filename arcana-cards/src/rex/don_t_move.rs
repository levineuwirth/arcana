//! Don't Move — `{3}{W}{W}` sorcery. "Destroy all tapped creatures. Until
//! your next turn, whenever a creature becomes tapped, destroy it."
//!
//! The first half is `ForEach DestroyPermanent` over tapped creatures.
//! GAP: the second half needs a one-shot lingering "becomes tapped"
//! trigger that lasts until your next turn — no DelayedAction/effect
//! variant for that.

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
    let name = reg.interner_mut().intern("Don't Move");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all tapped creatures. Until your next turn, whenever a creature becomes tapped, destroy it.".into(),
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
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().tapped_only(),
        entry.controller,
    );
    // GAP: cannot install a lasting "whenever a creature becomes tapped"
    // trigger from a spell resolve; emitting only the immediate wipe.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
