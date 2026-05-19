//! Cruel Ultimatum — `{U}{U}{B}{B}{B}{R}{R}` sorcery. "Target opponent
//! sacrifices a creature of their choice, discards three cards, then loses
//! 5 life. You return a creature card from your graveyard to your hand,
//! draw three cards, then gain 5 life."
//!
//! # GAP: "target opponent sacrifices a creature" — no Sacrifice effect.
//! The graveyard return, draw, and life effects are expressible.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Ultimatum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{B}{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent sacrifices a creature of their choice, discards three cards, then loses 5 life. You return a creature card from your graveyard to your hand, draw three cards, then gain 5 life.".into(),
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
    let TargetChoice::Player(opponent) = target else { return Vec::new(); };
    // GAP: opponent sacrifices a creature (no Sacrifice effect available)
    vec![
        Effect::Discard { player: *opponent, count: 3, choice: DiscardChoice::ControllerChooses },
        Effect::LoseLife { player: *opponent, amount: 5 },
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::GainLife { player: entry.controller, amount: 5 },
    ]
}
