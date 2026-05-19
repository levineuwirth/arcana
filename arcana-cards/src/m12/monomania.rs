//! Monomania — `{3}{B}{B}` sorcery. "Target player chooses a card in their
//! hand and discards the rest."
//!
//! The player targeted chooses which card to keep (controller chooses from
//! their own hand what to discard).  Using `DiscardChoice::ControllerChooses`
//! approximates this (the target player decides which cards go).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monomania");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player chooses a card in their hand and discards the rest.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let player = match target {
        arcana_core::targets::TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    let hand = script::hand_size(state, player);
    if hand <= 1 {
        return Vec::new();
    }
    // Discard all but one: hand - 1 cards, target player chooses which to keep
    vec![Effect::Discard {
        player,
        count: hand - 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
