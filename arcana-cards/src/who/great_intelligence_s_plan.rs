//! Great Intelligence's Plan — `{4}{U}{B}` sorcery. "Draw three cards.
//! Then target opponent faces a villainous choice — They discard three
//! cards, or you may cast a spell from your hand without paying its mana
//! cost."
//!
//! GAP: no villainous-choice (opponent picks one of two outcomes) nor
//! cast-from-hand-for-free machinery; the draw three is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Great Intelligence's Plan");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards. Then target opponent faces a villainous choice — They discard three cards, or you may cast a spell from your hand without paying its mana cost.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    // GAP: villainous choice / cast-for-free branch not modeled.
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
