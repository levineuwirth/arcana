//! Storm the Citadel — `{4}{G}` sorcery. "Until end of turn, creatures you
//! control get +2/+2 and gain 'Whenever this creature deals combat damage to
//! a player or planeswalker, destroy target artifact or enchantment defending
//! player controls.'"
//! GAP: granting a triggered ability to each creature until end of turn is not
//! expressible via the catalog. The board-wide +2/+2 Pump is implemented.

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
    let name = reg.interner_mut().intern("Storm the Citadel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, creatures you control get +2/+2 and gain \"Whenever this creature deals combat damage to a player or planeswalker, destroy target artifact or enchantment defending player controls.\"".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, entry.controller);
    // GAP: granting a triggered ability (combat damage trigger) until end of turn is not expressible
    ids.into_iter().map(|id| Effect::Pump {
        target: id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }).collect()
}
