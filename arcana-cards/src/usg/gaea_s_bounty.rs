//! Gaea's Bounty — `{2}{G}` sorcery. "Search your library for up to two
//! Forest cards, reveal those cards, put them into your hand, then
//! shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gaea's Bounty");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to two Forest cards, reveal those cards, put them into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest_filter = script::subtype_filter(reg, "Forest");
    // "Up to two" — repeat TutorToHand twice; engine handles the "up to"
    // by allowing zero finds on subsequent invocations.
    vec![
        Effect::TutorToHand {
            player: entry.controller,
            filter: forest_filter.clone(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: forest_filter,
            reveal: true,
        },
    ]
}
