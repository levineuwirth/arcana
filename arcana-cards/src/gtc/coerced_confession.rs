//! Coerced Confession — `{4}{U/B}` sorcery, "Target player mills four
//! cards. You draw a card for each creature card put into their
//! graveyard this way."
//!
//! The mill is expressible via `Effect::Mill`. The follow-up draw is
//! gated on how many of the milled cards were creature cards — a
//! quantity that depends on the specific cards that left the library
//! during *this* resolution. None of the `script::*` helpers can
//! observe "cards put into a graveyard this way", so the conditional
//! draw is a GAP rather than a wrong fixed-size draw.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coerced Confession");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player mills four cards. You draw a card for each creature card put into their graveyard this way.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: "draw a card for each creature card put into their graveyard
    // this way" — the count of milled creature cards is not observable
    // from any script:: helper; emitting only the mill, omitting the
    // dynamic draw rather than hardcoding a wrong literal.
    vec![Effect::Mill { player: *p, count: 4 }]
}
