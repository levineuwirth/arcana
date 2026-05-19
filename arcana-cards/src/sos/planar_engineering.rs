//! Planar Engineering — `{3}{G}` sorcery. "Sacrifice two lands. Search your
//! library for four basic land cards, put them onto the battlefield tapped,
//! then shuffle."
//!
//! # GAP: "sacrifice two lands" cost-as-effect is not in the catalog.
//! TutorToBattlefield with tapped=true for basic lands is expressible (×4),
//! but the sacrifice requirement is not. We emit four tutors tapped and note
//! the sacrifice gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Planar Engineering");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Sacrifice two lands. Search your library for four basic land cards, put them onto the battlefield tapped, then shuffle.".into(),
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
    // GAP: sacrificing two lands as part of resolution is not expressible
    let filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToBattlefield { player: entry.controller, filter: filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: entry.controller, filter: filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: entry.controller, filter: filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: entry.controller, filter, tapped: true },
    ]
}
