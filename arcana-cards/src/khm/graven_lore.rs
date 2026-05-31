//! Graven Lore — `{3}{U}{U}` snow instant. "Scry X, where X is the
//! amount of {S} spent to cast this spell, then draw three cards."
//!
//! Best-effort: the engine can draw three cards, but X (the amount of
//! snow mana {S} spent to cast the spell) is not derivable from the
//! resolution-time scripting helpers, so the dynamic Scry X cannot be
//! sized. We GAP the Scry and emit the fixed draw-three.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graven Lore");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry X, where X is the amount of {S} spent to cast this spell, then draw three cards.".into(),
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
    // GAP: Scry X where X is the amount of snow mana {S} spent to cast
    // this spell — the snow-mana-spent amount is not available from the
    // resolution-time scripting helpers, so the dynamic Scry cannot be
    // sized. The fixed draw-three is emitted.
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
