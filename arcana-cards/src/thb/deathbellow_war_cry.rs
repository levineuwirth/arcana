//! Deathbellow War Cry — `{5}{R}{R}{R}` sorcery. "Search your library for
//! up to four Minotaur creature cards with different names, put them onto
//! the battlefield, then shuffle."
//!
//! # GAP: subtype-filtered tutor for multiple cards (up to 4 Minotaurs
//! with different names) — ObjectFilter has no subtype predicate and
//! TutorToBattlefield finds one card. Emitting a single TutorToBattlefield
//! for a creature as the closest approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: subtype filter (Minotaur) not available on ObjectFilter; multi-card tutor not supported
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
