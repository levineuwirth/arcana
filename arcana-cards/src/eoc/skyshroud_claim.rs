//! Skyshroud Claim — `{3}{G}` sorcery. "Search your library for up
//! to two Forest cards, put them onto the battlefield, then shuffle."
//!
//! No land-subtype ObjectFilter refinement exists, so "Forest" is
//! approximated as a land search; the "up to two" is two tutor
//! effects.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyshroud Claim");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to two Forest cards, put them onto the battlefield, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no land-subtype filter — "Forest" approximated as any land.
    let filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: filter.clone(),
            tapped: false,
        },
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter,
            tapped: false,
        },
    ]
}
