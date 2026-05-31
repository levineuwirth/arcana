//! Into the Night — `{3}{R}` sorcery. "It becomes night. Discard any
//! number of cards, then draw that many cards plus one."
//!
//! "It becomes night" maps to Effect::SetDayNight(Night). The
//! "discard any number of cards, then draw that many plus one" clause is
//! a player-chosen variable discard whose drawn amount depends on the
//! number discarded; there is no primitive for a free-count discard nor
//! for binding the draw count to that choice (Discard takes a fixed
//! count). That clause is GAP-ed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::turn::DayNight;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Into the Night");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "It becomes night. Discard any number of cards, then draw that many cards plus one.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Discard any number of cards, then draw that many plus one" —
    // a player-chosen variable discard count that feeds the draw count.
    // Effect::Discard takes a fixed count and there is no primitive
    // binding a draw amount to the number of cards just discarded.
    vec![Effect::SetDayNight { value: DayNight::Night }]
}
