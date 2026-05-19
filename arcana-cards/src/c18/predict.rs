//! Predict — `{1}{U}` instant, "Choose a card name, then target player mills a card. If a card
//! with the chosen name was milled this way, you draw two cards. Otherwise, you draw a card."
//!
//! GAP: Named card choice at cast time gating a conditional draw based on which card was milled
//! is not expressible (no 'choose a card name' choice or conditional based on milled card identity).
//! The Mill portion for the target player is implemented.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predict");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose a card name, then target player mills a card. If a card with the chosen name was milled this way, you draw two cards. Otherwise, you draw a card.".into(),
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
    // GAP: named card choice; conditional draw 2 vs draw 1 based on milled card's identity
    vec![
        Effect::Mill { player: *p, count: 1 },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
