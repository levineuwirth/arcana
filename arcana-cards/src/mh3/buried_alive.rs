//! Buried Alive — `{2}{B}` sorcery.
//! "Search your library for up to three creature cards, put them into your
//! graveyard, then shuffle."
//!
//! Approximated as three sequential `TutorToGraveyard` effects (each is a
//! separate library search). The "up to" nature means the player may decline
//! to find a card on any search, but the three searches are mandatory rather
//! than optional — best-effort given the single-card tutor primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Buried Alive");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to three creature cards, put them into your graveyard, then shuffle.".into(),
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
    // Approximation: three separate library searches; "up to" constraint is
    // not enforced (player may search for 0 cards each time by choosing none).
    vec![
        Effect::TutorToGraveyard {
            player: entry.controller,
            filter: ObjectFilter::creature(),
            reveal: false,
        },
        Effect::TutorToGraveyard {
            player: entry.controller,
            filter: ObjectFilter::creature(),
            reveal: false,
        },
        Effect::TutorToGraveyard {
            player: entry.controller,
            filter: ObjectFilter::creature(),
            reveal: false,
        },
    ]
}
