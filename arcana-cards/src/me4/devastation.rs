//! Devastation — `{5}{R}{R}` sorcery. "Destroy all creatures and lands."

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
    let name = reg.interner_mut().intern("Devastation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures and lands.".into(),
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
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let land_ids = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::LAND.into()),
        entry.controller,
    );
    let mut effects = Vec::new();
    if !creature_ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        });
    }
    if !land_ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: land_ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        });
    }
    effects
}
