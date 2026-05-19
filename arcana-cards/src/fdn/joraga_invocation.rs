//! Joraga Invocation — `{4}{G}{G}` sorcery. "Each creature you control gets
//! +3/+3 until end of turn and must be blocked this turn if able."
//!
//! GAP: "Must be blocked this turn if able" per-creature restriction is not
//! expressible with the current Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joraga Invocation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each creature you control gets +3/+3 until end of turn and must be blocked this turn if able.".into(),
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
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, entry.controller);
    // GAP: "must be blocked if able" per-creature restriction
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
