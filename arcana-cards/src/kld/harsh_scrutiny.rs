//! Harsh Scrutiny — `{B}` sorcery.
//! "Target opponent reveals their hand. You choose a creature card from it.
//! That player discards that card. Scry 1."
//!
//! # GAP: look at opponent's hand and choose a specific card for discard
//! `Effect::Discard` with `DiscardChoice::ControllerChooses` discards from
//! among the target player's hand but does not filter to creature cards or
//! allow the caster to see and pick from the hand. No "reveal hand and choose
//! a specific card type" variant exists.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harsh Scrutiny");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You choose a creature card from it. That player discards that card. Scry 1.".into(),
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
    // GAP: reveal opponent's hand and choose a specific creature card to discard
    // Best effort: generic controller-chooses discard + scry
    vec![
        Effect::Discard { player: *p, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::Scry { player: entry.controller, count: 1 },
    ]
}
