//! Cleansing — `{W}{W}{W}` sorcery. "For each land, destroy that
//! land unless any player pays 1 life."
//!
//! "Unless any player pays" cost-prompt per land is not in catalog;
//! best-effort: destroy all lands (the optional life-pay opt-out is
//! GAP'd).

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
    let name = reg.interner_mut().intern("Cleansing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "For each land, destroy that land unless any player pays 1 life.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        entry.controller,
    );
    // GAP: "unless any player pays 1 life" per-permanent opt-out cost not in catalog.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
