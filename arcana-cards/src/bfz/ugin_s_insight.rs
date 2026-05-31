//! Ugin's Insight — `{3}{U}{U}` sorcery. "Scry X, where X is the
//! greatest mana value among permanents you control, then draw three
//! cards."
//!
//! The draw-three is a fixed `Effect::DrawCards`. The Scry amount X is
//! the GREATEST mana value among permanents you control — a max-over-CMC
//! quantity that none of the `script::*` helpers expose (they offer
//! count_matching / power / devotion / sizes, not a max-CMC reducer).
//! Per the dynamic-amount rule we must not hardcode a literal Scry
//! count, so the Scry is GAP-ed; the deterministic draw is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ugin's Insight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry X, where X is the greatest mana value among permanents you control, then draw three cards.".into(),
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
    // GAP: "Scry X, where X is the greatest mana value among permanents
    // you control" needs a max-over-CMC reducer; no script:: helper
    // exposes a maximum mana value, so the Scry portion is omitted.
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
