//! Seek the Horizon — `{3}{G}` sorcery. "Search your library for up to
//! three basic land cards, reveal them, put them into your hand, then
//! shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seek the Horizon");
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
                // GAP: "basic land cards" — ObjectFilter cannot restrict
                // to the Basic supertype; filtered to land cards.
                text: "Search your library for up to three basic land cards, reveal them, put them into your hand, then shuffle.".into(),
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
    let land = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToHand { player: entry.controller, filter: land.clone(), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: land.clone(), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: land, reveal: true },
    ]
}
