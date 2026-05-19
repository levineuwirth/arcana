//! Lay Bare the Heart — `{1}{B}` sorcery, "Target opponent reveals their
//! hand. You choose a nonlegendary, nonland card from it. That player discards
//! that card."
//!
//! # GAP
//! - "Reveal opponent's hand" is not an expressible effect in the catalog.
//! - "You choose which card the opponent discards" (controller-directed
//!   targeted discard with nonlegendary/nonland restriction) is not
//!   expressible. `DiscardChoice::OpponentChooses` lets the opponent pick;
//!   there is no `ControllerChooses` variant scoped to the target player.
//! Best effort: emit a 1-card discard where the opponent chooses. The
//! nonlegendary/nonland restriction and controller-selects-the-card are
//! omitted.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lay Bare the Heart");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You choose a nonlegendary, nonland card from it. That player discards that card.".into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: hand reveal; caster-directed discard with nonlegendary/nonland filter not in catalog
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}
