//! Realms Uncharted — `{2}{G}` instant. Search for up to four land cards
//! with different names, reveal, opponent chooses two for graveyard, rest
//! to hand.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Realms Uncharted");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to four land cards with different names and reveal them. An opponent chooses two of those cards. Put the chosen cards into your graveyard and the rest into your hand. Then shuffle.".into(),
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
    // GAP: opponent-chooses split between graveyard and hand not in catalog.
    // Best-effort tutor two lands to hand.
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToHand { player: entry.controller, filter: land_filter.clone(), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: land_filter, reveal: true },
    ]
}
