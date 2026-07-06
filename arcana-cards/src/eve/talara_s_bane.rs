//! Talara's Bane — `{1}{B}` sorcery. "Target opponent reveals their hand.
//! You choose a green or white creature card from it. You gain life equal
//! to that creature card's toughness, then that player discards that card."
//!
//! GAP: 'reveal opponent's hand and choose a specific card from it' is not
//! expressible (no Effect for targeted hand reveal + conditional discard of
//! a chosen card). GAP: life gain equal to toughness of a card in hand
//! (not on battlefield) is not expressible. Falling back to targeting
//! opponent and discarding one card (controller chooses approximation).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talara's Bane");
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
                text: "Target opponent reveals their hand. You choose a green or white creature card from it. You gain life equal to that creature card's toughness, then that player discards that card.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
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
    // GAP: reveal hand + choose green/white creature card from hand not expressible
    // GAP: life gain equal to toughness of card in hand not expressible
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::OpponentChooses }]
}
