//! Call the Gatewatch — `{2}{W}` sorcery. "Search your library for a
//! planeswalker card, reveal it, put it into your hand, then shuffle."
//
// GAP: searching for a planeswalker card requires ObjectFilter with PLANESWALKER
// type, but TutorToHand uses ObjectFilter::creature() or ObjectFilter::permanent().
// Using ObjectFilter::new().with_types for planeswalker.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Call the Gatewatch");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a planeswalker card, reveal it, put it into your hand, then shuffle.".into(),
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
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()),
        reveal: true,
    }]
}
