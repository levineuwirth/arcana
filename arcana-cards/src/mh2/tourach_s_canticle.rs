//! Tourach's Canticle — `{3}{B}` sorcery. "Target opponent reveals
//! their hand. You choose a card from it. That player discards that
//! card, then discards a card at random."
//!
//! GAP: reveal-hand + you-pick isn't modeled; emit a controller-chooses
//! discard plus a random discard.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tourach's Canticle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target opponent reveals their hand. You choose a card from it. That player discards that card, then discards a card at random.".into(),
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
    let Some(t) = entry.targets.targets.first() else { return Vec::new(); };
    let p = match t {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    // GAP: reveal-hand-and-pick — approximate with OpponentChooses
    // first discard (the chooser of the discard being the spell's
    // controller is not modeled distinctly; closest match is
    // OpponentChooses since "you" choose from another player's hand).
    let _ = entry;
    vec![
        Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
        Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::Random,
        },
    ]
}
