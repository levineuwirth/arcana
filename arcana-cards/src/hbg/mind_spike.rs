//! Mind Spike — `{B}` sorcery. "Target opponent reveals each
//! noncreature, nonland card in their hand. You choose a card
//! revealed this way. That player discards that card. You lose 2
//! life. If they didn't reveal a card this way, you draw a card."
//! Conditional discard-or-draw with filtered reveal not in catalog;
//! emit life loss and discard as best effort, GAP the reveal/draw.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mind Spike");
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
                text: "Target opponent reveals each noncreature, nonland card in their hand. You choose a card revealed this way. That player discards that card. You lose 2 life. If they didn't reveal a card this way, you draw a card.".into(),
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
    // GAP: filtered hand-reveal + "if they didn't reveal a card, draw" conditional — not in catalog.
    vec![
        Effect::Discard {
            player: *p,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
