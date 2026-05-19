//! Ego Drain — `{B}` sorcery.
//! "Target opponent reveals their hand. You choose a nonland card from it. That player discards
//! that card. If you don't control a Faerie, exile a card from your hand."
//!
//! # GAP: RevealOpponentHand — no Effect variant for revealing an opponent's hand to the
//! controller, letting the controller choose a card from it, and forcing that specific card to
//! be discarded (targeted discard).
//! # GAP: ConditionalSelfExile — no Effect variant for "if you don't control a [subtype],
//! exile a card from your hand".

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ego Drain");
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
                text: "Target opponent reveals their hand. You choose a nonland card from it. That player discards that card. If you don't control a Faerie, exile a card from your hand.".into(),
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
    // GAP: RevealOpponentHand — no Effect for revealing a hand and letting controller choose
    // a specific nonland card to discard (targeted discard by controller selection).
    // GAP: ConditionalSelfExile — no Effect for exiling from hand if you don't control a subtype.
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::OpponentChooses }]
}
