//! Worldpurge — `{4}{W/U}{W/U}{W/U}{W/U}` sorcery. "Return all
//! permanents to their owners' hands. Each player chooses up to seven
//! cards in their hand, then shuffles the rest into their library.
//! Each player loses all unspent mana."

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
    let name = reg.interner_mut().intern("Worldpurge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W/U}{W/U}{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                // GAP: "each player chooses up to seven cards, shuffles
                // the rest into their library" and "loses all unspent
                // mana" have no Effect variants; only the mass bounce
                // is emitted.
                text: "Return all permanents to their owners' hands.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::permanent(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
