//! Psychic Spear — `{B}` sorcery. "Target player reveals their hand.
//! You choose a Spirit or Arcane card from it. That player discards
//! that card."
//!
//! Modeled as a discard where the controller chooses (you pick the
//! card). GAP: the Spirit-or-Arcane filter on the chosen card is not
//! expressible — the engine's Discard effect has no card-filter knob.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychic Spear");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player reveals their hand. You choose a Spirit or Arcane card from it. That player discards that card.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "Spirit or Arcane" subtype filter on the chosen discarded card.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}
