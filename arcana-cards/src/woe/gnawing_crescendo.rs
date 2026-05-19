//! Gnawing Crescendo — `{2}{R}` instant. "Creatures you control get +2/+0 until
//! end of turn. Whenever a nontoken creature you control dies this turn, create
//! a 1/1 black Rat creature token with 'This token can't block.'"
//! GAP: triggered ability "whenever a nontoken creature dies this turn" with
//! deferred token creation.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnawing Crescendo");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control get +2/+0 until end of turn. Whenever a nontoken creature you control dies this turn, create a 1/1 black Rat creature token with \"This token can't block.\"".into(),
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
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    // GAP: "whenever a nontoken creature you control dies this turn" triggered ability
    ids.into_iter().map(|id| Effect::Pump {
        target: id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }).collect()
}
