//! In Garruk's Wake — `{7}{B}{B}` sorcery. "Destroy all creatures you don't
//! control and all planeswalkers you don't control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("In Garruk's Wake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures you don't control and all planeswalkers you don't control.".into(),
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
    let creature_filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let creature_ids = script::ids_matching(state, &creature_filter, entry.controller);
    vec![Effect::ForEach {
        targets: creature_ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
    // GAP: also destroy all planeswalkers opponents control (no planeswalker ObjectFilter variant)
}
