//! Deathbellow War Cry — `{5}{R}{R}{R}` sorcery. "Search your library
//! for up to four Minotaur creature cards with different names, put
//! them onto the battlefield, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathbellow War Cry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to four Minotaur creature cards with different names, put them onto the battlefield, then shuffle.".into(),
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
    // "up to four ... with different names" — best effort: a single
    // tutor onto the battlefield for a Minotaur creature card.
    let filter = script_filter(reg);
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter,
        tapped: false,
    }]
    // GAP: cannot search for up to four distinct-name cards in one
    // effect; only one Minotaur creature is fetched.
}

fn script_filter(reg: &CardRegistry) -> arcana_core::targets::ObjectFilter {
    arcana_core::script::subtype_filter(reg, "Minotaur")
}
