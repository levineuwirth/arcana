//! Apocalypse — `{2}{R}{R}{R}` sorcery. "Exile all permanents. You discard
//! your hand."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Apocalypse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile all permanents. You discard your hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let targets = script::ids_matching(state, &ObjectFilter::permanent(), entry.controller);
    vec![
        Effect::ForEach {
            targets,
            effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
        },
        Effect::Discard {
            player: entry.controller,
            count: script::hand_size(state, entry.controller),
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
