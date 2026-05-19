//! Flashfires — `{3}{R}` sorcery.
//! "Destroy all Plains."

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
    let name = reg.interner_mut().intern("Flashfires");
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all Plains.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plains_filter = script::subtype_filter(reg, "Plains")
        .with_types(TypeLine::LAND.into());
    let ids = script::ids_matching(state, &plains_filter, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
