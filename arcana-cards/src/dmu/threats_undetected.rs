//! Threats Undetected — `{2}{G}` sorcery. "Search your library for up
//! to four creature cards with different powers and reveal them. An
//! opponent chooses two of those cards. Shuffle the chosen cards into
//! your library and put the rest into your hand."
//!
//! Only the search-to-hand portion is expressible; the
//! different-powers constraint, the opponent's choice of two, and the
//! shuffle-back are not modeled by the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Threats Undetected");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to four creature cards with \
                   different powers and reveal them. An opponent chooses \
                   two of those cards. Shuffle the chosen cards into your \
                   library and put the rest into your hand."
                .into(),
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
    // GAP: cannot model "different powers" constraint nor the
    // opponent-chooses-two-to-shuffle-back partition; emitting a single
    // creature tutor-to-hand as the closest expressible approximation.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        reveal: true,
    }]
}
