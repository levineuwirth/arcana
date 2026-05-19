//! Environmental Sciences — `{2}` sorcery — Lesson. "Search your library for
//! a basic land card, put it onto the battlefield tapped, then shuffle. You
//! gain 2 life."
//!
//! GAP: basic land (supertype Basic) filter not available in ObjectFilter;
//! `.with_types(TypeLine::LAND.into())` gets all lands but not specifically
//! basic lands. TutorToBattlefield with generic land filter and GainLife 2 are
//! expressed; the "basic" restriction is lost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Environmental Sciences");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle. You gain 2 life.".into(),
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
    // GAP: basic land supertype filter not available; using generic land filter
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
        Effect::GainLife { player: entry.controller, amount: 2 },
    ]
}
