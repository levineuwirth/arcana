//! Engulf the Shore — `{3}{U}` instant, "Return to their owners' hands all
//! creatures with toughness less than or equal to the number of Islands you
//! control."
//!
//! GAP: counting Islands you control (subtype-based land count) to set a
//! dynamic toughness threshold is not expressible with current script helpers.
//! Partial: return all creatures to hand (unfiltered board wipe bounce).

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
    let name = reg.interner_mut().intern("Engulf the Shore");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return to their owners' hands all creatures with toughness less than or equal to the number of Islands you control.".into(),
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
    // GAP: filter by toughness <= number of Islands you control (Island subtype count not available)
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
