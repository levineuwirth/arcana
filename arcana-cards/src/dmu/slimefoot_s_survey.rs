//! Slimefoot's Survey — `{4}{G}` sorcery. Domain — "Search your library for
//! up to two land cards that each have a basic land type, put them onto the
//! battlefield tapped, then shuffle. Look at the top X cards of your library,
//! where X is the number of basic land types among lands you control. Put up
//! to one of them on top of your library and the rest on the bottom in a
//! random order."
//!
//! # GAP: TutorUpToTwo — TutorToBattlefield takes a single filter; no
//!   variant for 'up to two' with a basic-land-type predicate
//! # GAP: Domain/LookAtTopX — no Effect variant for variable Scry/Look
//!   based on basic land type count

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slimefoot's Survey");
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
                text: "Domain — Search your library for up to two land cards that each have a basic land type, put them onto the battlefield tapped, then shuffle. Look at the top X cards of your library, where X is the number of basic land types among lands you control. Put up to one of them on top of your library and the rest on the bottom of your library in a random order.".into(),
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
    // GAP: TutorUpToTwo — TutorToBattlefield takes a single filter; no 'up to two' variant
    // GAP: Domain/LookAtTopX — no Effect variant for variable look/scry based on basic land type count
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
    ]
}
